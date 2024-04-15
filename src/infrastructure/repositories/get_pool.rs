use std::sync::Arc;

use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel::PgConnection;

use crate::domain::error::RepositoryError;
use crate::domain::repositories::repository::RepositoryResult;
use crate::infrastructure::databases::postgresql::DBConn;

pub trait GetPool {
    fn get_pool(
        pool: &Arc<DBConn>,
    ) -> RepositoryResult<PooledConnection<ConnectionManager<PgConnection>>> {
        match pool.get() {
            Ok(value) => Ok(value),
            Err(_) => Err(RepositoryError {
                message: "Failed to get postgres connection pool".to_string(),
            }),
        }
    }
}
