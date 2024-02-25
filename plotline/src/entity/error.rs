use std::sync::PoisonError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(PartialEq, thiserror::Error, Debug)]
pub enum Error {
    // repo
    #[error("entity already exists")]
    AlreadyExists,
    #[error("entity not found")]
    NotFound,
    // application
    #[error("{0}: must to be set")]
    MandatoryField(&'static str),
    // foreign
    #[error("{0}")]
    Name(#[from] crate::name::Error),
    #[error("{0}")]
    Id(#[from] crate::id::Error),
    #[error("{0}")]
    Lock(String),
}

impl<T> From<PoisonError<T>> for Error {
    fn from(value: PoisonError<T>) -> Self {
        Self::Lock(value.to_string())
    }
}
