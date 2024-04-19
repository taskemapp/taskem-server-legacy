use crate::config::{
    CACHE_POOL_EXPIRE_SECONDS, CACHE_POOL_MAX_OPEN, CACHE_POOL_MIN_IDLE, REDIS_CON_STRING,
};
use crate::error::redis::RedisPoolError;
use diesel;
use r2d2_redis::{r2d2, RedisConnectionManager};
use std::env;
use std::time::Duration;

pub type RedisPool = r2d2::Pool<RedisConnectionManager>;

pub fn redis_pool() -> r2d2::Pool<RedisConnectionManager> {
    let url = env::var(REDIS_CON_STRING).expect("Can't get environment string");
    let manager =
        RedisConnectionManager::new(url).expect("Failed to create Redis connection manager");

    r2d2::Pool::builder()
        .max_size(CACHE_POOL_MAX_OPEN)
        .max_lifetime(Some(Duration::from_secs(CACHE_POOL_EXPIRE_SECONDS)))
        .min_idle(Some(CACHE_POOL_MIN_IDLE))
        .build(manager)
        .map_err(|e| RedisPoolError {
            message: format!("Failed to create Redis pool: {}", e),
        })
        .unwrap()
}
