use aws_config::load_from_env;
use aws_sdk_s3::Client;
use axum::{extract::Query, routing::get, Router, Server};
use r2d2::Pool;
use r2d2_redis::redis::Commands;
use r2d2_redis::{r2d2, RedisConnectionManager};
use serde::Deserialize;
use std::net::SocketAddr;
use std::{env, thread};

#[tokio::main]
async fn main() {
    let config = load_from_env().await;
    let _client = Client::new(&config);

    let pool = create_connection_pool();
    let app = Router::new()
        .route("/", get(|| async { "Hello world" }))
        .route("/redis", get(save_to_redis))
        .with_state(pool);

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

#[derive(Deserialize)]
struct AuthToken {
    id: i32,
    signature: String,
}

async fn save_to_redis(params: Query<AuthToken>) {
    let auth_token = params.0;
}

pub fn create_connection_pool() -> Pool<RedisConnectionManager> {
    dotenvy::dotenv().ok();
    let redis_url = env::var("REDIS_URL").expect("Redis URL not found");
    let manager = RedisConnectionManager::new(redis_url).unwrap();
    let pool = r2d2::Pool::builder().build(manager).unwrap();
    pool
}
