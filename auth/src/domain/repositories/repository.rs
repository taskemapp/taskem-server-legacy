use crate::domain::repositories::error::RepositoryError;

pub type RepositoryResult<T> = Result<T, RepositoryError>;
