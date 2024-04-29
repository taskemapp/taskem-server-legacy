use diesel::QueryResult;

use crate::domain::error::Error;
use crate::domain::error::Result;

pub trait MapFrom {
    fn map_from<T, A: From<T>>(&self, query: QueryResult<Vec<T>>) -> Result<Vec<A>> {
        Ok(query
            .map_err(|e| Error::MapFromError)?
            .into_iter()
            .map(|val| A::from(val))
            .collect())
    }
}
