extern crate r2d2_redis;
use dotenvy::dotenv;
use r2d2_redis::{
    r2d2,
    redis::{self, RedisResult},
    RedisConnectionManager,
};
use std::{env, ops::DerefMut};

pub type RedisPool = r2d2::Pool<RedisConnectionManager>;

pub fn create_connection_pool() -> r2d2::Pool<RedisConnectionManager> {
    dotenv().ok();
    let redis_url = env::var("REDIS_URL").expect("Redis URL not found");
    let manager = RedisConnectionManager::new(redis_url).unwrap();
    let pool = r2d2::Pool::builder().build(manager).unwrap();

    pool
    // for _ in 0..20 {
    //     let pool = pool.clone();
    //     handles.push(thread::spawn(move || {
    //         let mut connection = pool.get().unwrap();
    //         // use connection here
    //         cmd("GET")
    //             .arg("pp")
    //             .query::<String>(connection.deref_mut())
    //             .unwrap();
    //     }))
    // }
    // for h in handles {
    //     h.join().unwrap();
    // }
}

pub fn save_refresh_tokens(
    conn: &mut r2d2::PooledConnection<RedisConnectionManager>,
    refresh_token: &str,
    access_token: &str,
    ex: &usize,
) -> RedisResult<String> {
    redis::cmd("SET")
        .arg(&[
            refresh_token,
            access_token,
            "NX",
            "GET",
            "EX",
            &ex.to_string(),
        ])
        .query::<String>(conn.deref_mut())
}
