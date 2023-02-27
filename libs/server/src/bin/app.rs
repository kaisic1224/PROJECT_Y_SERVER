use axum::{routing::get, Router, Server};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(|| async { "Hello world" }));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

async fn shutdown_signal() {
    tokio::signal::ctrl_c().await.expect("Error shutting down");
    println!("Shutting server down");
}
