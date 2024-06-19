mod api;
mod config;
mod infrastructure;
mod state;

use api::router::create_router;
use state::init_state;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let state = init_state().await;
    let address: String = format!("{}:{}", state.config.server.host, state.config.server.port);

    println!("Stating {}: {}", state.config.name, address);
    let app = create_router(state);
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
