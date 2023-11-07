use std::sync::{Mutex, MutexGuard, PoisonError, Arc};

/// A Guard holds a copy of T while keeping locked the original value, ensuring its
/// consistency during transactions.
pub trait Guard<'a, T>: AsRef<T> + AsMut<T> {
    /// Commits the changes into the locked value, releasing the resource at the end.
    fn commit(self);
}

/// Guarded is an implementation of [Guard].
pub struct Guarded<'a, T> {
    sandbox: T,
    guard: MutexGuard<'a, T>,
    mu: Arc<Mutex<T>>,
}

impl<'a, T> AsRef<T> for Guarded<'a, T> {
    fn as_ref(&self) -> &T {
        &self.sandbox
    }
}

impl<'a, T> AsMut<T> for Guarded<'a, T> {
    fn as_mut(&mut self) -> &mut T {
        &mut self.sandbox
    }
}

impl<'a, T> TryFrom<Arc<Mutex<T>>> for Guarded<'a, T>
where T: Clone {
    type Error = PoisonError<MutexGuard<'a, T>>;

    fn try_from(mu: Arc<Mutex<T>>) -> Result<Self, Self::Error> {
        mu.clone().lock().map(|guard| Self {
            sandbox: guard.clone(),
            guard,
            mu,
        })
    }
}

impl<'a, T> Guard<'a, T> for Guarded<'a, T> {
    fn commit(mut self) {
        *self.guard = self.sandbox;
    }
}