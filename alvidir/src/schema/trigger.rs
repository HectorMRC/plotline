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

/// asdfasdfa
#[macro_export]
macro_rules! trigger {
    ($ctx:ty) => {
        trigger!($ctx, (), ())
    };
    ($ctx:ty, $err:ty) => {
        trigger!($ctx, (), $err)
    };
    ($ctx:ty, $args:ty, $err:ty) => {
        Box<dyn Command<$ctx, $args, Err = $err>>
    };
}

pub use trigger;
