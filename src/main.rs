pub mod client;

use crate::client::Client;
use axum::routing::get;

#[tokio::main]
async fn main() {
    let client = Client::new();
    let client = client.setup_get_route("", get(|| async { "OK" }));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, client.app).await.unwrap();
}
