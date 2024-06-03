use axum::{
    Json,
    extract::{State, Path, ConnectInfo}
};
use std::net::SocketAddr;
use serde::Deserialize;
use std::sync::Arc;
use url::Url;
use rand::{distributions::Alphanumeric, Rng};
use tokio_postgres::error::SqlState;
use chrono::prelude::*;
use super::response::{ApiResponse, ApiError, ShortenerMessage};

use crate::{infrastructure::redpanda::UserData, state::AppState};
use crate::infrastructure::repository::Repository;

#[derive(Deserialize)]
pub struct UserUrl {
    url: String
}

pub async fn status() -> Result<ApiResponse, ApiError>  {
    Ok(ApiResponse::OK(Some("Ok".to_string())))
}

pub async fn get_url<T:Repository + Send + Sync>(
    Path(id): Path<String>,
    State(state): State<Arc<AppState<T>>>,
    headers: axum::http::HeaderMap,
    ConnectInfo(remote_addr): ConnectInfo<SocketAddr>
) -> Result<ApiResponse, ApiError> {

    if id.len() != state.config.uri_size {
        let not_found = format!("{}/not-found", state.config.server.cors_addr);
        return Err(ApiError::Redirect(not_found));
    }
   
    let tmp = remote_addr.ip().to_string();
    let ip = headers.get("x-forwarded-for")
            .and_then(|v| v.to_str().ok())
            .unwrap_or(&tmp);

    let user_agent = headers.get("user-agent")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("-");

    let now: DateTime<Utc> = Utc::now();

    let url = match state.repository.get_url(id.clone()).await {
        Ok(url) => url,
        Err(err) => {
            println!("error getting url: {}", err);
            let not_found = format!("{}/not-found", state.config.server.cors_addr);
            return Err(ApiError::Redirect(not_found));
        }
    };

    let mut user_data = UserData {
        date: now,
        ip: ip.to_string(),
        uri: id,
        country: "".to_string(),
        user_agent: user_agent.to_string()
    };

    match state.repository.get_geoip(&ip).await {
        Ok(geoip) => {
            match geoip.country.and_then(|c| c.iso_code) {
                Some(country) => {
                    user_data.country = country.to_string();
                },
                None => println!("country of ip {} not found", ip),
            }
        },
        Err(err) => {
            println!("error getting geoip to ip {}: {}", ip, err);
        }
    }

    if let Err(err) = state.analytics.send(user_data).await {
        println!("error sending user analytics througth channel: {}", err);
    }

    return Ok(ApiResponse::Redirect(url));
}

pub async fn create_url<T:Repository + Send + Sync>(
    State(state): State<Arc<AppState<T>>>,
    Json(data): Json<UserUrl>
) -> Result<ApiResponse, ApiError> {

    let url = match Url::parse(&data.url) {
        Ok(url) => url,
        Err(_) => return Err(ApiError::BadRequest(Some("Invalid URL informed".to_string()))),
    };

    for _ in 0..5 {
        let uri = random_uri(state.config.uri_size);
        match state.repository.create_url(url.to_string(), uri.clone()).await {
            Ok(_) => {
                let short_url = format!("{}/{}", state.config.domain, uri);
                let res = ShortenerMessage { original_url: url.to_string(), short_url };
                return Ok(ApiResponse::Created(res));
            },
            Err(err) => {
                if let Some(db_err) = err.downcast_ref::<tokio_postgres::Error>() {
                    if let Some(err_code) = db_err.code() {
                        if err_code == &SqlState::UNIQUE_VIOLATION {
                            println!("short URI colision: {}", uri);
                            continue;
                        }
                    }
                }
                println!("error creating short URI: {}", err)
            }
        }
    }
    return Err(ApiError::InternalServerError)
}

fn random_uri(size: usize) -> String {
    let uri: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(size)
        .map(char::from)
        .collect();
    uri
}

