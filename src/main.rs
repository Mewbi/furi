mod handlers;

use handlers::{get_url, create_url, status};
use axum::{
    routing::{get, post},
    Router
};
use tower_http::trace::TraceLayer;
use std::sync::Arc;

use bb8::Pool;
use bb8_redis::RedisConnectionManager;
use bb8_redis::bb8;

#[derive(Debug)]
struct AppState {
    redis: Pool<RedisConnectionManager>
}


fn init_router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/", get(status))
        .route("/:id", get(get_url))
        .route("/url", post(create_url))
        .with_state(state)
        .layer(TraceLayer::new_for_http())
}

async fn init_state() -> Arc<AppState> {
    let manager = RedisConnectionManager::new("redis://localhost").unwrap();
    let pool = bb8::Pool::builder().build(manager).await.unwrap();

    Arc::new(AppState { 
        redis: pool,
    })
}


#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().with_max_level(tracing::Level::DEBUG).init();

    let state = init_state().await;
    let app = init_router(state);

    println!("i'm initing poggers in :3000");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

