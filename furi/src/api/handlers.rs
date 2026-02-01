use axum::{
    Json,
    extract::{State, Path, ConnectInfo}
};
use std::{net::IpAddr, str::FromStr};
use std::net::SocketAddr;
use serde::Deserialize;
use std::sync::Arc;
use url::Url;
use rand::{distributions::Alphanumeric, Rng};
use tokio_postgres::error::SqlState;
use chrono::prelude::*;
use super::response::{ApiResponse, ApiError, ShortenerMessage};

use crate::{infrastructure::timescale::UserData, state::AppState};
use crate::infrastructure::repository::Repository;

#[derive(Deserialize)]
pub struct UserUrl {
    url: String
}

pub async fn status<T:Repository + Send + Sync>(
    State(state): State<Arc<AppState<T>>>
) -> Result<ApiResponse, ApiError>  {
    let address = format!("{}", state.config.server.cors_addr);
    Ok(ApiResponse::Redirect(address))
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
   
    let remote_addr = remote_addr.ip().to_string();
    let xff = headers.get("x-forwarded-for")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("-");
    let ip = match parse_xff(xff) {
        Some(res) => res,
        None => remote_addr,
    };

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

    match state.repository.get_geoip(&user_data.ip.clone()).await {
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
        println!("error sending user analytics through channel: {}", err);
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

    match validate_url(url.clone()) {
        Ok(_) => (),
        Err(msg) => return Err(ApiError::BadRequest(Some(msg))),
    }

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

fn parse_xff(xff: &str) -> Option<String> {
    for val in xff.split(',') {
        match IpAddr::from_str(val.trim()) {
            Ok(ip) => return Some(ip.to_string()),
            Err(_) => continue,
        }
    }
    None
}

fn validate_url(url: Url) -> Result<(), String> {
    let host_str = url.host_str().ok_or("URL has no host")?;
    
    if host_str == "localhost" {
        return Err("URL points to an internal IP address".to_string());
    }
    
    if let Ok(ip) = host_str.parse::<IpAddr>() {
        if is_internal_ip(&ip) {
            return Err("URL points to an internal IP address".to_string());
        }
    } 
    
    Ok(())
}

fn is_internal_ip(ip: &IpAddr) -> bool {
    match ip {
        IpAddr::V4(ipv4) => {
            ipv4.is_loopback() || ipv4.is_private() || ipv4.is_link_local() || ipv4.is_broadcast() || ipv4.is_documentation()
        }
        IpAddr::V6(ipv6) => {
            ipv6.is_loopback() || ipv6.is_unspecified()
        }
    }
}
