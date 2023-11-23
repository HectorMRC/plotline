use parking_lot::{lock_api::ArcMutexGuard, Mutex, RawMutex};
use serde::{Deserialize, Serialize};
use std::{
    ops::{Deref, DerefMut},
    sync::Arc,
};

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

/// Resource implements the [Tx] trait for any piece of data.
#[derive(Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Resource<T> {
    mu: Arc<Mutex<T>>,
}

impl<T> From<T> for Resource<T> {
    fn from(value: T) -> Self {
        Resource {
            mu: Arc::new(Mutex::new(value)),
        }
    }
}

impl<T> Tx<T> for Resource<T>
where
    T: Clone,
{
    type Guard = ResourceGuard<T>;

    fn begin(self) -> Self::Guard {
        let guard = self.mu.lock_arc();
        ResourceGuard {
            data: guard.clone(),
            guard,
        }
    }
}

impl<T> From<Arc<Mutex<T>>> for Resource<T> {
    fn from(value: Arc<Mutex<T>>) -> Self {
        Self { mu: value }
    }
}

/// ResourceGuard is the [TxGuard] implementation for [Resource].
pub struct ResourceGuard<T> {
    guard: ArcMutexGuard<RawMutex, T>,
    data: T,
}

impl<T> Deref for ResourceGuard<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T> DerefMut for ResourceGuard<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

impl<T> TxGuard<T> for ResourceGuard<T> {
    fn commit(mut self) {
        *self.guard = self.data;
    }
}
