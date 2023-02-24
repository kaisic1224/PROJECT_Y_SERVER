pub mod models;
pub mod schema;

extern crate redis;
use diesel::pg::PgConnection;
use diesel::Connection;
use dotenvy::dotenv;
use redis::Commands;
use std::env;

pub fn test_redis() -> redis::RedisResult<()> {
    dotenv().expect("No env file found");
    let redis_url = env::var("REDIS_URL").expect("Invalid redis url");
    let client = redis::Client::open(redis_url)?;
    let mut con = client.get_connection()?;

    /* do something here */
    con.set("fruit", "banana")?;
    let fruit: String = con.get("fruit")?;

    println!("{}", fruit);

    Ok(())
}

pub fn establish_connection() -> PgConnection {
    dotenv().expect("No env file found");

    let db_url = env::var("DATABASE_URL").expect("Database URL not found in env file");
    PgConnection::establish(&db_url).unwrap_or_else(|_| panic!("Error connecting to {}", &db_url))
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
