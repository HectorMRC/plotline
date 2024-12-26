//! Resources from the schema.

use std::{
    any::{Any, TypeId},
    collections::BTreeMap,
    marker::PhantomData,
    sync::{RwLock, RwLockReadGuard, RwLockWriteGuard},
};

use crate::deref::{TryDeref, TryDerefMut};

/// Represents a set of arbitrary resources.
#[derive(Debug, Default)]
pub struct ResourceSet {
    resources: BTreeMap<TypeId, RwLock<Box<dyn Any>>>,
}

impl ResourceSet {
    /// Registers the given resource.
    ///
    /// This methos overwrites any older value for the same resource type.
    pub fn with_resource<R>(mut self, resource: R) -> Self
    where
        R: 'static,
    {
        let type_id = TypeId::of::<R>();
        self.resources
            .insert(type_id, RwLock::new(Box::new(resource)));
        self
    }
}

/// A read-only access to a resource.
pub struct Read<'a, R> {
    guard: Option<RwLockReadGuard<'a, Box<dyn Any>>>,
    _type: PhantomData<R>,
}

impl<R> TryDeref for Read<'_, R>
where
    R: 'static,
{
    type Target = R;

    fn try_deref(&self) -> Option<&Self::Target> {
        self.guard.as_ref().and_then(|res| res.downcast_ref())
    }
}

impl<'a, R> From<&'a ResourceSet> for Read<'a, R>
where
    R: 'static,
{
    fn from(set: &'a ResourceSet) -> Self {
        Self {
            guard: set
                .resources
                .get(&TypeId::of::<R>())
                .and_then(|lock| lock.read().ok()),
            _type: PhantomData,
        }
    }
}

/// A read-write access to a resource.
pub struct Write<'a, R> {
    guard: Option<RwLockWriteGuard<'a, Box<dyn Any>>>,
    _type: PhantomData<R>,
}

impl<R> TryDeref for Write<'_, R>
where
    R: 'static,
{
    type Target = R;

    fn try_deref(&self) -> Option<&Self::Target> {
        self.guard.as_ref().and_then(|res| res.downcast_ref())
    }
}

impl<R> TryDerefMut for Write<'_, R>
where
    R: 'static,
{
    fn try_deref_mut(&mut self) -> Option<&mut Self::Target> {
        self.guard.as_mut().and_then(|res| res.downcast_mut())
    }
}

impl<'a, R> From<&'a ResourceSet> for Write<'a, R>
where
    R: 'static,
{
    fn from(set: &'a ResourceSet) -> Self {
        Self {
            guard: set
                .resources
                .get(&TypeId::of::<R>())
                .and_then(|lock| lock.write().ok()),
            _type: PhantomData,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        deref::{TryDeref, TryDerefMut},
        graph::Graph,
        id::fixtures::IndentifyMock,
        schema::{
            resource::{Read, Write},
            Schema,
        },
    };

    #[test]
    fn read_resource() {
        struct Foo;
        struct Bar;

        let schema = Schema::from(Graph::<IndentifyMock<usize>>::default()).with_resource(Foo);

        let read_foo = Read::<Foo>::from(schema.resources());
        assert!(
            read_foo.try_deref().is_some(),
            "resource from the schema should be dereferenced"
        );

        let read_bar = Read::<Bar>::from(schema.resources());

        assert!(
            read_bar.try_deref().is_none(),
            "unregistered resource should not be dereferenced"
        );
    }

    #[test]
    fn write_resource() {
        struct Foo(usize);

        let schema = Schema::from(Graph::<IndentifyMock<usize>>::default()).with_resource(Foo(0));

        {
            let mut write_foo = Write::<Foo>::from(schema.resources());
            let foo = write_foo
                .try_deref_mut()
                .expect("resource from the schema should be dereferenced");
            foo.0 = 1;
        }

        let read_foo = Read::<Foo>::from(schema.resources());
        let foo = read_foo
            .try_deref()
            .map(|foo| foo.0)
            .expect("resource from the schema should be dereferenced");

        assert_eq!(
            foo, 1,
            "previous value of the resource should be overwritten"
        );
    }
}
