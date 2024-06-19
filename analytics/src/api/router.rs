use axum::{
    http::{header, HeaderValue, Method},
    routing::{get, post},
    Router,
};

use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;

use crate::infrastructure::repository::Repository;
use crate::state::AppState;
use std::sync::Arc;

use super::handlers::{get_uri_metrics, status};

pub fn create_router<T: Repository + Send + Sync + 'static>(state: Arc<AppState<T>>) -> Router {
    let origin = state.config.server.cors_addr.clone();
    Router::new()
        .route("/", get(status))
        .route("/:id", post(get_uri_metrics))
        .with_state(state)
        .layer(
            CorsLayer::new()
                .allow_origin(origin.parse::<HeaderValue>().unwrap())
                .allow_methods([Method::GET, Method::POST])
                .allow_headers([header::CONTENT_TYPE])
                .allow_credentials(true),
        )
        .layer(TraceLayer::new_for_http())
}
