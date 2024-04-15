pub const POSTGRESQL_DB_URI: &str = "DATABASE_URL";

pub const REDIS_CON_STRING: &str = "REDIS_URL";

pub const MIDDLEWARE_AUTH_SESSION_KEY: &str = "authorization";

pub const CACHE_POOL_MAX_OPEN: u32 = 16;
pub const CACHE_POOL_MIN_IDLE: u32 = 8;
// pub const CACHE_POOL_TIMEOUT_SECONDS: u64 = 1;
pub const CACHE_POOL_EXPIRE_SECONDS: u64 = 60;

pub const EMAIL: &str = r"^(([^<>()\[\]\\.,;:\s@”]+(\.[^<>()\[\]\\.,;:\s@”]+)*)|(“.+”))@((\[[0–9]{1,3}\.[0–9]{1,3}\.[0–9]{1,3}\.[0–9]{1,3}])|(([a-zA-Z\-0–9]+\.)+[a-zA-Z]{2,}))$";
// pub const BASE64: &str = r"^[A-Fa-f0-9]{8,64}$";

pub const ONE_DAY: i64 = 60 * 60 * 24;
