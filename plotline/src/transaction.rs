use std::ops::{Deref, DerefMut};

/// Tx represents a resource to be manipulated transactionally.
pub trait Tx<T> {
    type ReadGuard<'a>: TxReadGuard<T>
    where
        Self: 'a;

    type WriteGuard<'a>: TxWriteGuard<T>
    where
        Self: 'a;

    /// Acquires the resource, blocking the current thread until it is available
    /// to do so.
    async fn read(&self) -> Self::ReadGuard<'_>;

    /// Acquires the resource, blocking the current thread until it is available
    /// to do so.
    async fn write(&self) -> Self::WriteGuard<'_>;
}

/// A TxReadGuard holds T ensuring its consistency between transactions.
pub trait TxReadGuard<T>: Deref<Target = T> {
    /// Releases the resource.
    fn release(self);
}

/// A TxWriteGuard holds a copy of T while keeping locked the original value,
/// ensuring its consistency between transactions.
pub trait TxWriteGuard<T>: Deref<Target = T> + DerefMut {
    /// Releases the resource right after updating its content with the
    /// manipulated data.
    fn commit(self);

    /// Releases the resource, discarting any possible update its content
    /// may had.
    fn rollback(self);
}
