pub mod models;
pub mod schema;

use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::{Connection, RunQueryDsl};
use dotenvy::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().expect("No env file found");

    let db_url = env::var("DATABASE_URL").expect("Database URL not found in env file");
    PgConnection::establish(&db_url).unwrap_or_else(|_| panic!("Error connecting to {}", &db_url))
}

pub type PgPool = Pool<ConnectionManager<PgConnection>>;

pub fn create_connection_pool() -> Pool<ConnectionManager<PgConnection>> {
    dotenv().expect("no env file found");

    let db_url = env::var("DATABASE_URL").expect("Database url not specified in env");

    let manager = ConnectionManager::<PgConnection>::new(db_url);
    Pool::builder()
        .test_on_check_out(true)
        .build(manager)
        .expect("Can not build connection")
}

use self::models::{Account, NewAccount};
pub fn create_acc(
    conn: &mut PgConnection,
    email: &str,
    account_username: &str,
    account_password: &str,
) -> Account {
    use crate::schema::account;

    let new_post = NewAccount {
        account_username,
        email,
        account_password,
    };

    diesel::insert_into(account::table)
        .values(&new_post)
        .get_result(conn)
        .expect("Error saving new post")
}
