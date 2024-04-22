use diesel;
use diesel::pg::PgConnection;
use diesel::r2d2;
use diesel::r2d2::ConnectionManager;

use crate::config::POSTGRESQL_DB_URI;
use std::env;

pub type Pool<T> = r2d2::Pool<ConnectionManager<T>>;
pub type PostgresPool = Pool<PgConnection>;
pub type PostgresDBConn = PostgresPool;

pub fn postgres_pool() -> PostgresDBConn {
    let database_url = env::var(POSTGRESQL_DB_URI)
        .unwrap_or_else(|_| panic!("{value} must be set", value = POSTGRESQL_DB_URI));
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder()
        .build(manager)
        .expect("Failed to create pool")
}
