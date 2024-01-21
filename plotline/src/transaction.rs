use std::ops::{Deref, DerefMut};

/// Tx represents a resource to be manipulated transactionally.
pub trait Tx<T> {
    type Guard: TxGuard<T>;

    /// Acquires the resource, blocking the current thread until it is available
    /// to do so.
    fn begin(self) -> Self::Guard;
}

/// A TxGuard holds a copy of T while keeping locked the original value,
/// ensuring its consistency between transactions.
pub trait TxGuard<T>: Deref<Target = T> + DerefMut {
    /// Releases the resource right after updating its content with the
    /// manipulated data.
    fn commit(self);
}
