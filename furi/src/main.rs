mod api;
mod config;
mod infrastructure;
mod state;

use std::net::SocketAddr;
use tokio::sync::mpsc::{self, Receiver, Sender};
use tokio::task;

use api::router::create_router;
use infrastructure::redpanda::{handle_user_data, UserData};
use state::init_state;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let (tx, rx): (Sender<UserData>, Receiver<UserData>) = mpsc::channel(1000);

    let state = init_state(tx).await;
    task::spawn(handle_user_data(rx, state.config.clone()));

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
