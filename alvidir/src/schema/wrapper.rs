//! Wrapper representation.

/// A helper type that allows T to improve its API based on constraints.
pub struct Wrapper<T> {
    pub(super) inner: T,
}

impl<T> From<T> for Wrapper<T> {
    fn from(inner: T) -> Self {
        Wrapper { inner }
    }
}
