use std::fmt::Debug;
use std::ops::Deref;
use std::sync::Arc;

use derive_new::new;
use r2d2_redis::r2d2::Pool;
use r2d2_redis::redis::Commands;
use r2d2_redis::RedisConnectionManager;
use tracing::{debug, error};
use uuid::Uuid;

use crate::common::Error;
use crate::common::Result;
use crate::domain::constants::ONE_DAY;
use crate::domain::repositories::session::RedisSessionRepository;

#[derive(new, Clone, Debug)]
pub struct RedisSessionRepositoryImpl {
    pool: Arc<Pool<RedisConnectionManager>>,
}

impl RedisSessionRepository for RedisSessionRepositoryImpl {
    fn create(&self, user_id: &Uuid) -> Result<String> {
        let binding = self.pool.clone();
        let client = binding.deref();
        let mut connection = client.get().unwrap();

        match self.validate(&format!("session_id:{}", &user_id)) {
            Ok(key) => Ok(key),
            Err(_) => {
                let exp = ONE_DAY * 3;
                let key = format!("session_id:{}", user_id);
                connection
                    .set::<&str, String, ()>(
                        &format!("session_id:{}", user_id),
                        user_id.to_string(),
                    )
                    .expect("Failed to set session data");
                connection
                    .expire::<&str, ()>(&key, exp as usize)
                    .expect("Failed to set expiration time");
                Ok(user_id.to_string())
            }
        }
    }

    fn validate(&self, session_id: &str) -> Result<String> {
        debug!("Session validate id: {}", session_id);
        let binding = self.pool.clone();
        let client = binding.deref();
        let mut connection = client.get().unwrap();
        match connection.exists::<&str, i64>(session_id) {
            Ok(value) => {
                if value > 0 {
                    match connection.get::<&str, String>(session_id) {
                        Ok(_) => {
                            let split_session = session_id.split(':').collect::<Vec<&str>>()[1];
                            Ok(split_session.to_string())
                        }
                        Err(_) => Err(Error::Redis),
                    }
                } else {
                    Err(Error::Redis)
                }
            }
            Err(e) => {
                error!("{}", e);
                Err(Error::Redis)
            }
        }
    }

    fn session_expand(&self, session_id: &str) -> Result<()> {
        if self.validate(session_id).is_err() {
            return Err(Error::Redis);
        }

        let binding = self.pool.clone();
        let client = binding.deref();
        let mut connection = client.get().unwrap();
        let exp = ONE_DAY * 3;
        connection
            .expire::<&str, ()>(session_id, exp as usize)
            .map_err(|e| {
                error!("{:?}", e);
            })
            .expect("Failed to set expiration time");
        Ok(())
    }

    fn remove_session(&self, session_id: &str) -> Result<()> {
        let binding = self.pool.clone();
        let client = binding.deref();
        let mut connection = client.get().unwrap();

        match connection.del::<&str, ()>(session_id) {
            Ok(_) => Ok(()),
            Err(e) => Err(Error::Redis),
        }
    }
}
