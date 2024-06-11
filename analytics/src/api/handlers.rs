use axum::extract::{State, Path};
use std::sync::Arc;
use super::response::{ApiError, ApiResponse, OkTypes};

use crate::state::AppState;
use crate::infrastructure::repository::Repository;


pub async fn status() -> Result<ApiResponse, ApiError>  {
    Ok(ApiResponse::OK(OkTypes::Empty))
}

pub async fn get_uri_metrics<T:Repository + Send + Sync>(
    State(state): State<Arc<AppState<T>>>,
    Path(id): Path<String>
) -> Result<ApiResponse, ApiError> {
    match state.repository.get_uri_metrics(id.clone()).await {
        Ok(metrics) => {
            return Ok(ApiResponse::OK(OkTypes::Metrics(metrics)));
        },
        Err(err) => {
            println!("error getting uri metrics: {}", err);
            return Err(ApiError::InternalServerError);
        }
    };
}

