use std::fmt::Debug;

use uuid::Uuid;

use crate::domain::error::Result;

pub trait RedisSessionRepository: Send + Sync + Debug {
    fn create(&self, user_id: &Uuid) -> Result<String>;
    fn validate(&self, session_id: &str) -> Result<String>;
    fn session_expand(&self, session_id: &str) -> Result<()>;
    fn remove_session(&self, session_id: &str) -> Result<()>;
}
