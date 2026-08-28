use axum::{routing::get, Router};
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber;

mod network;
mod storage;
mod config;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let config = config::Config::load();

    let app = Router::new()
        .route("/health", get(network::server::health_check))
        .route("/api/data", get(network::server::get_data))
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        );

    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));
    tracing::info!("Server listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
