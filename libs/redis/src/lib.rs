extern crate redis;
use dotenvy::dotenv;
use r2d2::Pool;
use r2d2_redis::{r2d2, RedisConnectionManager};
use std::env;

pub type RedisPool = Pool<RedisConnectionManager>;

pub fn create_connection_pool() -> Pool<RedisConnectionManager> {
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
