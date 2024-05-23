use std::fmt::Display;

use derive_more::From;

mod app_error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, From)]
pub enum Error {
    Repository,
    Redis,
    File,
    Checksum,
    GetPool,
    MapFrom,
    CreateApp(app_error::Error),
}

impl Display for Error {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(fmt, "{self:?}")
    }
}
