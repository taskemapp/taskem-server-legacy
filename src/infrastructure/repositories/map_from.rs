use diesel::QueryResult;

use crate::common::Error;
use crate::common::Result;

pub trait MapFrom {
    fn map_from<T, A: From<T>>(&self, query: QueryResult<Vec<T>>) -> Result<Vec<A>> {
        Ok(query
            .map_err(|e| Error::MapFrom)?
            .into_iter()
            .map(|val| A::from(val))
            .collect())
    }
}
