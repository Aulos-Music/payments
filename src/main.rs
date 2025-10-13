pub mod app;
pub mod error;
pub mod server;

use crate::app::setup_app;

#[tokio::main]
async fn main() {
    match setup_app().await {
        Ok((client, listener)) => {
            println!("Listening on port 3000");
            axum::serve(listener, client.app)
                .with_graceful_shutdown(app::shutdown_signal())
                .await
                .expect("Server failed");
        }
        Err(e) => {
            eprintln!("Failed to start: {e}");
        }
    };
}
