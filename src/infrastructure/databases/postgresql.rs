use crate::domain::constants::POSTGRESQL_DB_URI;
use diesel;
use diesel::pg::PgConnection;
use diesel::r2d2;
use diesel::r2d2::ConnectionManager;

use std::env;

pub type Pool<T> = r2d2::Pool<ConnectionManager<T>>;
pub type PostgresPool = Pool<PgConnection>;
pub type DBConn = PostgresPool;

pub fn db_pool() -> DBConn {
    let database_url = env::var(POSTGRESQL_DB_URI)
        .unwrap_or_else(|_| panic!("{value} must be set", value = POSTGRESQL_DB_URI));
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder()
        .build(manager)
        .expect("Failed to create pool")
}
