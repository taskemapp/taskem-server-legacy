use crate::domain::repositories::error::RepositoryError;
use crate::domain::repositories::repository::RepositoryResult;
use std::error::Error;
use tracing::error;

pub trait MapFrom {
    fn map_from<T, A: From<T>>(
        &self,
        query: Result<Vec<T>, Box<dyn Error>>,
    ) -> RepositoryResult<Vec<A>> {
        match query {
            Ok(vec) => Ok(vec.into_iter().map(|val| A::from(val)).collect()),
            Err(e) => {
                error!("{:?}", e);
                Err(RepositoryError {
                    message: e.to_string(),
                })
            }
        }
    }
}
