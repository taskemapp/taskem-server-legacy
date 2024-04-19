use crate::error::postgresql::PostgresPoolError;
use crate::pool::postgresql::PostgresDBConn;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel::PgConnection;
use std::sync::Arc;

pub trait GetPool {
    fn get_pool(
        pool: &Arc<PostgresDBConn>,
    ) -> Result<PooledConnection<ConnectionManager<PgConnection>>, PostgresPoolError> {
        match pool.get() {
            Ok(value) => Ok(value),
            Err(_) => Err(PostgresPoolError {
                message: "Failed to get postgres connection pool".to_string(),
            }),
        }
    }
}
