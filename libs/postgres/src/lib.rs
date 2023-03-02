pub mod models;
pub mod schema;

use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use diesel::Connection;
use dotenvy::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().expect("No env file found");

    let db_url = env::var("DATABASE_URL").expect("Database URL not found in env file");
    PgConnection::establish(&db_url).unwrap_or_else(|_| panic!("Error connecting to {}", &db_url))
}
