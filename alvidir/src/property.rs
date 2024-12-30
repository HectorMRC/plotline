//! Resource definition.

use crate::deref::TryDeref;

/// A value in a source.
pub trait Property<Src> {
    /// Retrives all the ocurrences of self in the source.
    fn all(source: &Src) -> Vec<Self>
    where
        Self: Sized;
}

impl<T, U> Property<U> for T
where
    T: Property<U::Target>,
    U: TryDeref,
{
    fn all(source: &U) -> Vec<Self>
    where
        Self: Sized,
    {
        source.try_deref().map(T::all).unwrap_or_default()
    }
}
