use axum::{
    http::{header, HeaderValue, Method},
    routing::get,
    Router
};

use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;

use std::sync::Arc;
use crate::state::AppState;
use crate::infrastructure::repository::Repository;

use super::handlers::{status, get_uri_metrics};

pub fn create_router<T:Repository + Send + Sync + 'static>(state: Arc<AppState<T>>) -> Router {
    let origin = format!("{}", state.config.server.cors_addr);
    Router::new()
        .route("/", get(status))
        .route("/:id", get(get_uri_metrics))
        .with_state(state)
        .layer(CorsLayer::new()
                .allow_origin(origin.parse::<HeaderValue>().unwrap())
                .allow_methods([Method::GET, Method::POST])
                .allow_headers([header::CONTENT_TYPE])
                .allow_credentials(true))
        .layer(TraceLayer::new_for_http())
}
