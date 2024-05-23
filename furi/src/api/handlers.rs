use axum::{
    Json,
    extract::{State, Path}
};

use serde::Deserialize;
use std::sync::Arc;
use url::Url;
use rand::{distributions::Alphanumeric, Rng};
use tokio_postgres::error::SqlState;
use super::response::{ApiResponse, ApiError, ShortenerMessage};

use crate::state::AppState;
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
    State(state): State<Arc<AppState<T>>>
) -> Result<ApiResponse, ApiError> {

    if id.len() != state.config.uri_size {
        let not_found = format!("http://{}:5173/not-found", state.config.server.host);
        return Err(ApiError::Redirect(not_found));
    }

    match state.repository.get_url(id).await {
        Ok(url) => {
            return Ok(ApiResponse::Redirect(url));
        },

        Err(err) => {
            println!("error getting url: {}", err);
            let not_found = format!("http://{}:5173/not-found", state.config.server.host);
            return Err(ApiError::Redirect(not_found));
        }
    };
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
                let short_url = format!("http://{}:{}/{}", state.config.server.host, state.config.server.port, uri);
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

