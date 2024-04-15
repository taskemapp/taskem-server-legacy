use crate::domain::error::RedisError;
use std::fmt::Debug;
use uuid::Uuid;

pub trait RedisSessionRepository: Send + Sync + Debug {
    fn create(&self, user_id: &Uuid) -> Result<String, RedisError>;
    fn validate(&self, session_id: &str) -> Result<String, RedisError>;
    fn session_expand(&self, session_id: &str) -> Result<(), RedisError>;
    fn remove_session(&self, session_id: &str) -> Result<(), RedisError>;
}
