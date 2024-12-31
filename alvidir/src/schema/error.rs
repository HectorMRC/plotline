//! Error definition.

use std::fmt::Display;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Determines that an operation has no effect.
    #[error("nothing to apply")]
    Noop,
    #[error("{0}")]
    Msg(String),
}

impl Error {
    /// Returns an error with he given message as cause.
    pub fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        Self::Msg(msg.to_string())
    }
}
