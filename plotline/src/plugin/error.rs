use super::PluginId;
use std::fmt::{Debug, Display};

pub type Result<T> = std::result::Result<T, Error>;

/// An Error that specifies the error source.
#[derive(Debug, thiserror::Error, PartialEq, Eq, Clone)]
pub enum Error {
    #[error("{plugin}: {error}")]
    Execution {
        plugin: PluginId,
        error: ExecutionError,
    },
    #[error("{plugin}: {error}")]
    Output {
        plugin: PluginId,
        error: OutputError,
    },
    #[error("{0}")]
    Stack(ErrorStack),
}

impl Error {
    /// Returns a a builder function of [Error::Execution] for an specific
    /// [PluginId].
    pub fn execution<T>(plugin: PluginId) -> impl FnOnce(T) -> Self
    where
        T: Into<ExecutionError>,
    {
        |value: T| Self::Execution {
            plugin,
            error: value.into(),
        }
    }

    /// Joins self and tail within a single error instance.
    pub fn join(self, tail: Self) -> Self {
        Error::Stack(match (self, tail) {
            (Error::Stack(mut head_errors), Error::Stack(tail_errors)) => {
                head_errors.0.extend(tail_errors.0);
                head_errors
            }
            (Error::Stack(mut head_errors), tail_error) => {
                head_errors.0.push(tail_error);
                head_errors
            }
            (head_error, Error::Stack(tail_errors)) => {
                let mut tmp = vec![head_error];
                tmp.extend(tail_errors.0);
                ErrorStack(tmp)
            }
            (head_error, tail_error) => ErrorStack(vec![head_error, tail_error]),
        })
    }
}

/// ErrorStack is a collection of errors.
#[derive(PartialEq, Eq, Clone)]
pub struct ErrorStack(Vec<Error>);

impl Debug for ErrorStack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <Self as Display>::fmt(self, f)
    }
}

impl Display for ErrorStack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join("; ")
        )
    }
}

/// An ExecutionError may occurs when executing a plugin (e.g. it panics).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecutionError(String);

impl<T: AsRef<str>> From<T> for ExecutionError {
    fn from(value: T) -> Self {
        Self(value.as_ref().to_string())
    }
}

impl Display for ExecutionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// An Error that may be returned as the output of a plugin.
#[derive(Clone, Eq)]
pub struct OutputError {
    pub code: String,
    pub message: String,
}

impl PartialEq for OutputError {
    fn eq(&self, other: &Self) -> bool {
        self.code == other.code
    }
}

impl Debug for OutputError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <Self as Display>::fmt(self, f)
    }
}

impl Display for OutputError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl OutputError {
    pub fn new(code: impl AsRef<str>) -> Self {
        Self {
            code: code.as_ref().to_string(),
            message: Default::default(),
        }
    }

    pub fn with_message(mut self, msg: impl AsRef<str>) -> Self {
        self.message = msg.as_ref().to_string();
        self
    }
}
