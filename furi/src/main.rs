mod api;
mod config;
mod infrastructure;
mod state;

use std::net::SocketAddr;
use tokio::sync::mpsc::{self, Receiver, Sender};
use tokio::task;

use api::router::create_router;
use infrastructure::timescale::{create_pool, handle_user_data, UserData};
use state::init_state;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let (tx, rx): (Sender<UserData>, Receiver<UserData>) = mpsc::channel(1000);

    let state = init_state(tx).await;

    let timescale_pool = match create_pool(&state.config.timescale).await {
        Ok(pool) => pool,
        Err(err) => panic!("Error connecting to TimescaleDB: {}", err),
    };

    let buffer_size = state.config.timescale.buffer_size;
    let flush_interval = state.config.timescale.flush_interval;
    task::spawn(handle_user_data(rx, timescale_pool, buffer_size, flush_interval));

    let address: String = format!("{}:{}", state.config.server.host, state.config.server.port);

    println!("Starting {}: {}", state.config.name, address);

    let app = create_router(state);
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
}
