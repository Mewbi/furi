mod config;
mod state;
mod api;
mod infrastructure;

use std::net::SocketAddr;
use tokio::sync::mpsc::{self, Sender, Receiver};
use tokio::task;

use state::init_state;
use api::router::create_router;
use infrastructure::redpanda::{UserData, handle_user_data};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().with_max_level(tracing::Level::DEBUG).init();

    let (tx, rx): (Sender<UserData>, Receiver<UserData>) = mpsc::channel(1000);

    let state = init_state(tx).await;
    task::spawn(handle_user_data(rx, state.config.clone()));

    let address: String = format!("{}:{}", state.config.server.host, state.config.server.port);
    let app = create_router(state);

    println!("Starting Furi API: {}", address );
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>()).await.unwrap();
}

