//! Deref definition.

/// A type that may be immutably dereferenced.
pub trait TryDeref {
    type Target;

    /// Tries to dereferences the value.
    fn try_deref(&self) -> Option<&Self::Target>;
}

/// A type that may be mutably dereferenced.
pub trait TryDerefMut: TryDeref {
    /// Tries to dereferences the value.
    fn try_deref_mut(&mut self) -> Option<&mut Self::Target>;
}
