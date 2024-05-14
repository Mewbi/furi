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
use bb8_postgres::PostgresConnectionManager;
use tokio_postgres::NoTls;

#[derive(Debug)]
struct AppState {
    server: Server,
    redis: Pool<RedisConnectionManager>,
    postgres: Pool<PostgresConnectionManager<NoTls>>
}

#[derive(Debug)]
struct Server {
    port: u16,
    host: String
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

    let manager_redis = RedisConnectionManager::new("redis://localhost").unwrap();
    let pool_redis = bb8::Pool::builder().build(manager_redis).await.unwrap();

    let manager_postgres = PostgresConnectionManager::new_from_stringlike("host=localhost port=5432 user=admin password=pass dbname=data", NoTls).unwrap();
    let pool_postgres = Pool::builder().build(manager_postgres).await.unwrap();

    Arc::new(AppState { 
        redis: pool_redis,
        postgres: pool_postgres,
        server: Server{
            port: 3000,
            host: "127.0.0.1".to_string()
        },
    })
}


#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().with_max_level(tracing::Level::DEBUG).init();

    let state = init_state().await;
    let address: String = format!("0.0.0.0:{}", state.server.port);
    let app = init_router(state);

    println!("i'm initing poggers in {}", address );
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

