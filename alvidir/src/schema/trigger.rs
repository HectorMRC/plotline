//! Trigger helpers.

/// A helper type that allows T to improve its trigger API.
pub struct WithTrigger<T> {
    pub inner: T,
}

impl<T> From<T> for WithTrigger<T> {
    fn from(inner: T) -> Self {
        WithTrigger { inner }
    }
}
