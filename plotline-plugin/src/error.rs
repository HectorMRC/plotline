use std::fmt::{Debug, Display};
use plotline::plugin::ExecutionError;

/// RawResult represents the output of aplugin in binary format.
pub type RawResult = std::result::Result<Vec<u8>, RawError>;

/// RawError represents an error message comming from a [RawPlugin]
/// implementation.
pub struct RawError(String);

impl From<String> for RawError {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl Display for RawError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.0, f)
    }
}

impl Into<ExecutionError> for RawError {
    fn into(self) -> ExecutionError {
        self.0.into()
    }
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("plugin not found")]
    NotFound,
    #[error("plugin already exists ")]
    AlreadyExists,
    #[error("plugin is not of the expected kind")]
    WrongKind,
    #[error("field is missing: {0}")]
    MissingField(&'static str),
    #[error("{0}")]
    Version(#[from] crate::version::Error),
    #[error("{0}")]
    Id(#[from] plotline::id::Error),
    #[error("{0}")]
    Name(#[from] plotline::name::Error),
    #[error("{0}")]
    Interval(String),
}
