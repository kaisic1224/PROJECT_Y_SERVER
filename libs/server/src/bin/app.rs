use aws_config::load_from_env;
use aws_sdk_s3::Client;
use axum::{
    extract::{Query, State},
    http::StatusCode,
    routing::get,
    Json, Router, Server,
};
use chrono::prelude::*;
use jsonwebtoken::{
    decode, encode, errors::ErrorKind, DecodingKey, EncodingKey, Header, Validation,
};
use postgres::create_acc;
use redis::save_refresh_tokens;
use ring::rand::SecureRandom;
use serde::{Deserialize, Serialize};
use std::{collections::HashSet, fs, net::SocketAddr, thread};

#[macro_use]
extern crate lazy_static;
lazy_static! {
    static ref PRIVATE_KEY: Vec<u8> = fs::read("./jwt_private_key.pem").unwrap();
    static ref PUBLIC_KEY: Vec<u8> = fs::read("./jwt_public_key.pem").unwrap();
    static ref ISS: HashSet<String> =
        HashSet::from_iter(vec!["http://localhost:3000".to_string()].iter().cloned());
}

#[tokio::main]
async fn main() {
    let config = load_from_env().await;
    let _client = Client::new(&config);

    let redis_pool = redis::create_connection_pool();
    let pg_pool = postgres::create_connection_pool();
    let app = Router::new()
        .route("/", get(|| async { "Hello world" }))
        .route("/auth", get(authorize))
        .route("/token", get(token))
        .with_state(AppState {
            pg_pool,
            redis_pool,
        });

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    match Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
    {
        Ok(c) => c,
        Err(err) => {
            panic!("{}", err);
        }
    };
}

async fn shutdown_signal() {
    tokio::signal::ctrl_c().await.expect("Error shutting down");
    println!("Shutting server down");
}
#[derive(Clone)]
pub struct AppState {
    pg_pool: postgres::PgPool,
    redis_pool: redis::RedisPool,
}

#[derive(Serialize, Deserialize)]
struct AuthToken {
    token: String,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct UserRequest {
    pub email: String,
    pub pw: String,
    pub username: String,
    // pub code_verifier: String,
}

#[derive(Deserialize, Serialize)]
pub struct UserResponse {
    pub email: String,
    pub username: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    pub email: String,
    pub username: String,
    pub exp: usize,
    pub iat: usize,
    pub iss: HashSet<String>,
    pub sub: String,
}

async fn authorize(
    Query(params): Query<UserRequest>,
    State(state): State<AppState>,
) -> Json<AuthToken> {
    // read private key
    // public key is used for verifying the private key, rsa is based on input from private key and outputted public key
    // so create the jwt/initially sign it with the private key
    // then we look up to verify it hasnt changed with the public key
    // symetric encryption - uses same key for encrypt and decrypt
    // yo refactor the pg account method creation this sucks LOL
    let state = state.clone();
    let pg_pool = state.pg_pool;
    // use redis to save the refresh token for 7 days TTL
    // on visit to /refresh_token
    // check if requested token exists in redis
    // generate new access refresh token pair
    // (rotating tokens) then invalidate the old one
    let redis_pool = state.redis_pool;
    let other_params = params.clone();
    let mut access_token: [u8; 0] = [];
    let refresh_token = "sdfsdf";
    let sr = ring::rand::SystemRandom::new();
    print!("{}", std::str::from_utf8(&access_token).unwrap());
    sr.fill(&mut access_token).unwrap();
    let t = thread::spawn(move || {
        let conn = &mut pg_pool.get().unwrap();
        let redis_conn = &mut redis_pool.get().unwrap();
        let _acc = create_acc(
            conn,
            other_params.email.as_str(),
            other_params.username.as_str(),
            other_params.pw.as_str(),
        );

        let res = match save_refresh_tokens(
            redis_conn,
            &refresh_token,
            std::str::from_utf8(&access_token).unwrap(),
            &(Utc::now().timestamp() + 7 * 24 * 60 * 60)
                .try_into()
                .unwrap(),
        ) {
            Ok(c) => c,
            Err(err) => format!("Error setting tokens, {}", err),
        };
    });
    t.join().unwrap();

    let claims = Claims {
        email: params.email,
        username: params.username.to_owned(),
        sub: params.username.to_owned(), //change to db id later or uuid
        exp: (Utc::now().timestamp() + 1 * 60 * 60).try_into().unwrap(),
        iat: Utc::now().timestamp().try_into().unwrap(),
        iss: ISS.clone(),
    };

    let token = encode(
        &Header::new(jsonwebtoken::Algorithm::RS512),
        &claims,
        &EncodingKey::from_rsa_pem(&PRIVATE_KEY).unwrap(),
    )
    .unwrap();
    Json(AuthToken { token })
}

// add other type here, will either be a new token, or existing session
// should add some sort of reference here to ensure that we can check the sub of the signature to be intedended for that same person
async fn token(Query(token): Query<AuthToken>) -> (StatusCode, Json<UserResponse>) {
    // turbofish into the struct we want to deserialzie it into
    let mut validation = Validation::new(jsonwebtoken::Algorithm::RS512);
    // make new deserialize struct to retain token payload
    validation.iss = Some(ISS.clone());
    // validate pkce code
    let (code, token_data) = match decode::<UserResponse>(
        &token.token,
        &DecodingKey::from_rsa_pem(&PUBLIC_KEY).unwrap(),
        &validation,
    ) {
        Ok(c) => (StatusCode::OK, c),
        Err(err) => match *err.kind() {
            // check for auth and expiry here
            ErrorKind::InvalidToken => todo!(), //return (StatusCode::FORBIDDEN), // Example on how to handle a specific error
            ErrorKind::InvalidIssuer => todo!(), // Example on how to handle a specific error,
            ErrorKind::ExpiredSignature => todo!(),
            _ => todo!(),
        },
    };
    // check code verifier

    (code, Json(token_data.claims))
}

async fn save_to_bucket() {}
