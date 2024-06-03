use axum::{
    Json,
    extract::State
};
use serde::Deserialize;
use std::sync::Arc;
use super::response::{ApiResponse, ApiError};

use crate::state::AppState;
use crate::infrastructure::repository::Repository;

#[derive(Deserialize)]
pub struct UserUrl {
    url: String
}

pub async fn status() -> Result<ApiResponse, ApiError>  {
    Ok(ApiResponse::OK(Some("Ok".to_string())))
}

pub async fn create_url<T:Repository + Send + Sync>(
    State(state): State<Arc<AppState<T>>>,
    Json(data): Json<UserUrl>
) -> Result<ApiResponse, ApiError> {
    Ok(ApiResponse::OK(Some("Ok".to_string())))
}

