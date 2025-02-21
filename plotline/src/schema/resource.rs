//! Resources from the schema.

use std::{
    any::{Any, TypeId},
    collections::BTreeMap,
    marker::PhantomData,
    sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard},
};

use crate::{
    deref::{ReadOnly, ReadWrite, TryDeref, TryDerefMut, With},
    id::Identify,
};

use super::transaction::Context;

/// Represents a set of arbitrary resources.
#[derive(Debug, Default)]
pub struct ResourceSet {
    resources: BTreeMap<TypeId, Arc<RwLock<Box<dyn Any>>>>,
}

impl ResourceSet {
    /// Registers the given resource.
    ///
    /// This methos overwrites any older value for the same resource type.
    #[allow(clippy::arc_with_non_send_sync)]
    pub fn with_resource<R>(mut self, resource: R) -> Self
    where
        R: 'static,
    {
        let type_id = TypeId::of::<R>();
        self.resources
            .insert(type_id, Arc::new(RwLock::new(Box::new(resource))));
        self
    }
}

/// A resource that may, or may not, exist in the schema.
pub struct Res<T> {
    lock: Option<Arc<RwLock<Box<dyn Any>>>>,
    _type: PhantomData<T>,
}

/// Holds a read-only access to a resource.
pub struct ResReadGuard<'a, T> {
    guard: Option<RwLockReadGuard<'a, Box<dyn Any>>>,
    _type: PhantomData<T>,
}

impl<T> Default for ResReadGuard<'_, T> {
    fn default() -> Self {
        Self {
            guard: Default::default(),
            _type: PhantomData,
        }
    }
}

impl<T> TryDeref for ResReadGuard<'_, T>
where
    T: 'static,
{
    type Target = T;

    fn try_deref(&self) -> Option<&Self::Target> {
        self.guard.as_ref()?.downcast_ref()
    }
}

impl<T> ReadOnly for Res<T>
where
    T: 'static,
{
    type Target = T;
    type Guard<'a> = ResReadGuard<'a, T>;

    fn read(&self) -> Self::Guard<'_> {
        let Some(lock) = self.lock.as_ref() else {
            return Default::default();
        };

        match lock.read() {
            Ok(guard) => ResReadGuard {
                guard: Some(guard),
                _type: PhantomData,
            },
            Err(err) => {
                tracing::error!(error = err.to_string(), type_id = ?TypeId::of::<T>(), "accessing resource");
                Default::default()
            }
        }
    }
}

/// Holds a read-write access to a resource.
pub struct ResWriteGuard<'a, T> {
    guard: Option<RwLockWriteGuard<'a, Box<dyn Any>>>,
    _type: PhantomData<T>,
}

impl<T> Default for ResWriteGuard<'_, T> {
    fn default() -> Self {
        Self {
            guard: Default::default(),
            _type: PhantomData,
        }
    }
}

impl<T> TryDeref for ResWriteGuard<'_, T>
where
    T: 'static,
{
    type Target = T;

    fn try_deref(&self) -> Option<&Self::Target> {
        self.guard.as_ref()?.downcast_ref()
    }
}

impl<T> TryDerefMut for ResWriteGuard<'_, T>
where
    T: 'static,
{
    fn try_deref_mut(&mut self) -> Option<&mut Self::Target> {
        self.guard.as_mut()?.downcast_mut()
    }
}

impl<T> ReadWrite for Res<T>
where
    T: 'static,
{
    type Target = T;
    type Guard<'a> = ResWriteGuard<'a, T>;

    fn write(&self) -> Self::Guard<'_> {
        let Some(lock) = self.lock.as_ref() else {
            return Default::default();
        };

        match lock.write() {
            Ok(guard) => ResWriteGuard {
                guard: Some(guard),
                _type: PhantomData,
            },
            Err(err) => {
                tracing::error!(error = err.to_string(), type_id = ?TypeId::of::<T>(), "accessing resource");
                Default::default()
            }
        }
    }
}

impl<T> Res<T>
where
    T: 'static,
{
    /// Returns true if, and only if, the resource exists.
    pub fn exists(&self) -> bool {
        self.with(|_| true).unwrap_or_default()
    }
}

impl<T> From<&ResourceSet> for Res<T>
where
    T: 'static,
{
    fn from(set: &ResourceSet) -> Self {
        Self {
            lock: set.resources.get(&TypeId::of::<T>()).cloned(),
            _type: PhantomData,
        }
    }
}

impl<T, R> From<&Context<'_, T>> for Res<R>
where
    T: Identify,
    R: 'static,
{
    fn from(ctx: &Context<T>) -> Self {
        ctx.resources().into()
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        deref::{With, WithMut},
        graph::Graph,
        id::fixtures::IndentifyMock,
        schema::{resource::Res, Schema},
    };

    #[test]
    fn resource_exists() {
        struct Foo;
        struct Bar;

        let schema = Schema::from(Graph::<IndentifyMock<usize>>::default()).with_resource(Foo);

        let res = Res::<Foo>::from(schema.resources());
        assert!(res.exists(), "resource from the schema should exists");

        let no_res = Res::<Bar>::from(schema.resources());

        assert!(!no_res.exists(), "unregistered resource should not exists");
    }

    #[test]
    fn resource_read_write() {
        struct Foo(usize);

        let schema = Schema::from(Graph::<IndentifyMock<usize>>::default()).with_resource(Foo(0));

        let res = Res::<Foo>::from(schema.resources());
        res.with_mut(|foo| {
            foo.0 = 1;
        })
        .expect("resource from the schema should exists");

        res.with(|foo| {
            assert_eq!(
                foo.0, 1,
                "previous value of the resource should be overwritten"
            );
        })
        .expect("resource from the schema should exists");
    }
}
