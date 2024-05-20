mod config;
mod handlers;
mod repository;

use config::{read_config_file, AppConfig};
use handlers::{get_url, create_url, status};
use repository::{connect_redis, connect_postgres};
use axum::{
    http::{header, HeaderValue, Method},
    routing::{get, post},
    Router
};
// use http::header;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use std::sync::Arc;

use bb8::Pool;
use bb8_redis::RedisConnectionManager;
use bb8_redis::bb8;
use bb8_postgres::PostgresConnectionManager;
use tokio_postgres::NoTls;

#[derive(Debug)]
struct AppState {
    config: AppConfig,
    redis: Pool<RedisConnectionManager>,
    postgres: Pool<PostgresConnectionManager<NoTls>>
}


fn init_router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/", get(status))
        .route("/:id", get(get_url))
        .route("/url", post(create_url))
        .with_state(state)
        .layer(CorsLayer::new()
                .allow_origin("http://localhost:5173".parse::<HeaderValue>().unwrap())
                .allow_methods([Method::GET, Method::POST])
                .allow_headers([header::CONTENT_TYPE])
                .allow_credentials(true))
        .layer(TraceLayer::new_for_http())
}

async fn init_state() -> Arc<AppState> {

    let mut config = AppConfig::default();
    match read_config_file("config.toml") {
        Ok(c) => config = c,
        Err(err) => panic!("Error loading config file: {}", err),
    }

    match connect_redis(&config.redis).await {
        Ok(pool_redis) => {
            match connect_postgres(&config.postgres).await {
                Ok(pool_postgres) => {
                    return Arc::new(AppState { 
                        redis: pool_redis,
                        postgres: pool_postgres,
                        config: config,
                    });
                },
                Err(err) => panic!("Error connecting to redis: {}", err),
            }
        },
        Err(err) => panic!("Error connecting to redis: {}", err),
    }
}


#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().with_max_level(tracing::Level::DEBUG).init();

    let state = init_state().await;
    let address: String = format!("0.0.0.0:{}", state.config.server.port);
    let app = init_router(state);

    println!("i'm initing poggers in {}", address );
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

