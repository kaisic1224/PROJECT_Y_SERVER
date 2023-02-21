pub mod models;
pub mod schema;

use diesel::pg::PgConnection;
use diesel::{prelude, Connection};
use dotenvy::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("Database URL not found in env file");
    PgConnection::establish(&db_url).unwrap_or_else(|_| panic!("Error connecting to {}", &db_url))
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {}
}
