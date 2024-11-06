//! Resource definition.

use crate::deref::TryDeref;

/// A value in a source.
pub trait Resource<Src> {
    /// Retrives all the ocurrences of self in the source.
    fn all(source: &Src) -> Vec<Self>
    where
        Self: Sized;
}

impl<T, U> Resource<U> for T
where
    T: Resource<U::Target>,
    U: TryDeref,
{
    fn all(source: &U) -> Vec<Self>
    where
        Self: Sized,
    {
        source.try_deref().map(T::all).unwrap_or_default()
    }
}
