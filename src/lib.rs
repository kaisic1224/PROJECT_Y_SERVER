pub mod models;
pub mod schema;

use diesel::pg::PgConnection;
use diesel::{Connection, RunQueryDsl};
use dotenvy::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().expect("No env file found");

    let db_url = env::var("DATABASE_URL").expect("Database URL not found in env file");
    PgConnection::establish(&db_url).unwrap_or_else(|_| panic!("Error connecting to {}", &db_url))
}

use self::models::*;
pub fn create_post(conn: &mut PgConnection, title: &str, body: &str) -> Post {
    use crate::schema::posts;

    let new_post = NewPost { title, body };

    diesel::insert_into(posts::table)
        .values(&new_post)
        .get_result::<Post>(conn)
        .expect("Error uploading posts")
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
