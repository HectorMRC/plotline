use std::fmt::{Display, Debug};
use std::error::Error;

#[derive(Debug)]
pub struct PoisonError<T, E> {
    pub error: E,
    pub inner: T,
}

impl<T: Debug, E: Error> std::error::Error for PoisonError<T, E> {}

impl<T, E: Error> Display for PoisonError<T, E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.error)
    }
}

impl<T, E> PoisonError<T, E> {
    pub fn new(guard: T, cause: E) -> Self {
        PoisonError { inner: guard, error: cause }
    }

    pub fn into_inner(self) -> T {
        self.inner
    }
}