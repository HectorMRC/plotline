//! Resources from the schema.

use std::{
    any::{Any, TypeId},
    collections::BTreeMap,
    marker::PhantomData,
    sync::{Arc, RwLock},
};

use crate::id::Identify;

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

impl<T> Res<T>
where
    T: 'static,
{
    /// Gets a read-only access to the resource and executes the given closure.
    pub fn with<F, R>(&self, f: F) -> Option<R>
    where
        F: FnOnce(&T) -> R,
    {
        match self.lock.as_ref()?.read() {
            Ok(res) => Some(f(res.downcast_ref()?)),
            Err(err) => {
                tracing::error!(error = err.to_string(), type_id = ?TypeId::of::<T>(), "accessing resource");
                None
            }
        }
    }

    /// Gets a read-write access to the resouce and executes the given closure.
    pub fn with_mut<F, R>(&self, f: F) -> Option<R>
    where
        F: FnOnce(&mut T) -> R,
    {
        match self.lock.as_ref()?.write() {
            Ok(mut res) => Some(f(res.downcast_mut()?)),
            Err(err) => {
                tracing::error!(error = err.to_string(), type_id = ?TypeId::of::<T>(), "accessing resource");
                None
            }
        }
    }

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
