use std::error::Error;
use std::fmt::{Debug, Display};

#[derive(Debug)]
pub struct ResidueError<T, E> {
    pub error: E,
    pub inner: T,
}

impl<T: Debug, E: Error> std::error::Error for ResidueError<T, E> {}

impl<T, E: Error> Display for ResidueError<T, E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.error)
    }
}

impl<T, E> ResidueError<T, E> {
    pub fn new(guard: T, cause: E) -> Self {
        ResidueError {
            inner: guard,
            error: cause,
        }
    }

    pub fn into_inner(self) -> T {
        self.inner
    }
}
