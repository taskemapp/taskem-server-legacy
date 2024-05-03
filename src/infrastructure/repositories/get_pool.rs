use std::sync::Arc;

use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel::PgConnection;

use crate::domain::error::Error;
use crate::domain::error::Result;
use crate::infrastructure::databases::postgresql::DBConn;

pub trait GetPool {
    fn get_pool(pool: &Arc<DBConn>) -> Result<PooledConnection<ConnectionManager<PgConnection>>> {
        match pool.get() {
            Ok(value) => Ok(value),
            Err(_) => Err(Error::GetPoolError),
        }
    }
}
