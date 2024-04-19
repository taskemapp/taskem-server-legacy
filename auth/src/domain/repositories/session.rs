use crate::domain::repositories::error::SessionRepositoryError;
use std::fmt::Debug;
use uuid::Uuid;

pub trait SessionRepository: Send + Sync + Debug {
    fn create(&self, user_id: &Uuid) -> Result<String, SessionRepositoryError>;
    fn validate(&self, session_id: &str) -> Result<String, SessionRepositoryError>;
    fn session_expand(&self, session_id: &str) -> Result<(), SessionRepositoryError>;
    fn remove_session(&self, session_id: &str) -> Result<(), SessionRepositoryError>;
}
