use aws_config::load_from_env;
use aws_sdk_s3::Client;
use axum::{extract::Query, http::StatusCode, routing::get, Json, Router, Server};
use jsonwebtoken::{encode, EncodingKey, Header};
use postgres;
use serde::{Deserialize, Serialize};
use server;
use std::{fs, net::SocketAddr};

#[tokio::main]
async fn main() {
    let config = load_from_env().await;
    let _client = Client::new(&config);

    let redis_pool = server::create_connection_pool();
    let pg_pool = postgres::create_connection_pool();
    let app = Router::new()
        .route("/", get(|| async { "Hello world" }))
        .route("/redis", get(save_to_redis))
        .route("/login", get(login))
        .with_state(redis_pool)
        .with_state(pg_pool);

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

#[derive(Serialize)]
struct AuthToken {
    token: String,
    expires_in: i32,
}

#[derive(Deserialize, Serialize)]
pub struct User {
    pub email: String,
    pub pw: String,
}

async fn save_to_redis(Query(params): Query<User>) -> Json<AuthToken> {
    // read private key
    // public key is used for verifying the private key, rsa is based on input from private key and outputted public key
    // so create the jwt/initially sign it with the private key
    // then we look up to verify it hasnt changed with the public key
    let key = fs::read("./private-key.pem").unwrap();
    let token = encode(&Header::default(), &params, &EncodingKey::from_secret(&key)).unwrap();
    Json(AuthToken {
        expires_in: 3600,
        token,
    })
}

async fn login() -> Result<String, StatusCode> {
    Ok("suck my cock idot".to_string())
}
