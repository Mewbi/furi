mod config;
mod state;
mod api;
mod infrastructure;

use state::init_state;
use api::router::create_router;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().with_max_level(tracing::Level::DEBUG).init();

    let state = init_state().await;
    let address: String = format!("0.0.0.0:{}", state.config.server.port);
    let app = create_router(state);

    println!("i'm initing poggers in {}", address );
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

