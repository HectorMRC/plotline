use crate::error::ResidueError;
use std::sync::PoisonError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, PartialEq, thiserror::Error)]
pub enum Error {
    // repo
    #[error("experience already exists")]
    AlreadyExists,
    #[error("experience not found")]
    NotFound,
    // application
    #[error("{0}: must to be set")]
    MandatoryField(&'static str),
    // domain
    #[error("an entity cannot result from the same experience more than once")]
    RepeatedEntity,
    // foreign
    #[error("{0}")]
    Entity(#[from] crate::entity::Error),
    #[error("{0}")]
    Event(#[from] crate::event::Error),
    #[error("{0}")]
    Plugin(String),
    #[error("{0}")]
    Lock(String),
}

impl<T, E> From<ResidueError<T, E>> for Error
where
    E: Into<Error>,
{
    fn from(value: ResidueError<T, E>) -> Self {
        value.error.into()
    }
}

impl<T> From<PoisonError<T>> for Error {
    fn from(value: PoisonError<T>) -> Self {
        Self::Lock(value.to_string())
    }
}
