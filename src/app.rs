use axum::routing::get;
use tokio::net::TcpListener;

use crate::{error::SetupError, server::Server};

pub async fn setup_app() -> Result<(Server, TcpListener), SetupError> {
    let client = Server::new();
    let client = client.setup_get_route("/", get(|| async { "OK" }));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    Ok((client, listener))
}

pub async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("Failed to install CTRL-C handler");
    println!("\nShutting down...")
}
