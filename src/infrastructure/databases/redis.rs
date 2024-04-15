use crate::domain::constants::{
    CACHE_POOL_EXPIRE_SECONDS, CACHE_POOL_MAX_OPEN, CACHE_POOL_MIN_IDLE, REDIS_CON_STRING,
};
use diesel;
use diesel::r2d2;
use dotenv::Error;
use r2d2_redis::RedisConnectionManager;
use std::env;
use std::time::Duration;

pub fn redis_pool() -> Result<r2d2::Pool<RedisConnectionManager>, Error> {
    let url = env::var(REDIS_CON_STRING).expect("Can't get environment string");
    let manager =
        RedisConnectionManager::new(url).expect("Failed to create Redis connection manager");
    let pool = r2d2::Pool::builder()
        .max_size(CACHE_POOL_MAX_OPEN)
        .max_lifetime(Some(Duration::from_secs(CACHE_POOL_EXPIRE_SECONDS)))
        .min_idle(Some(CACHE_POOL_MIN_IDLE))
        .build(manager)
        .expect("Failed to create Redis connection pool");
    Ok(pool)
}
