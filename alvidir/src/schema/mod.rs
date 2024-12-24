//! Schema representation.

pub mod delete;
pub mod guard;
pub mod plugin;
pub mod resource;
pub mod save;
pub mod trigger;

use std::{
    any::{Any, TypeId},
    collections::BTreeMap,
    sync::RwLock,
};

use guard::{SchemaReadGuard, SchemaWriteGuard};
use plugin::Plugin;
use trigger::{OnContext, Trigger};

use crate::{command::CommandRef, graph::Graph, id::Identify};

/// A graph that is subject to a set of rules.
pub struct Schema<T>
where
    T: Identify,
{
    /// The graph being orchestrated by this schema.
    graph: RwLock<Graph<T>>,
    /// All the resources of this schema.
    resources: BTreeMap<TypeId, Box<dyn Any>>,
    /// All the triggers in the schema.
    triggers: Vec<Box<dyn Any>>,
}

impl<T> From<Graph<T>> for Schema<T>
where
    T: Identify,
{
    fn from(graph: Graph<T>) -> Self {
        Self {
            graph: RwLock::new(graph),
            resources: Default::default(),
            triggers: Default::default(),
        }
    }
}

impl<T> Schema<T>
where
    T: Identify,
{
    /// Installs the given plugin in the schema.
    pub fn install<P>(self, plugin: P) -> Self
    where
        P: Plugin<T> + 'static,
    {
        plugin.setup(self)
    }

    /// Adds the given resource into the schema.
    ///
    /// If the resource already exists, the old value is overwritten.
    pub fn with_resource<R>(mut self, resource: R) -> Self
    where
        R: 'static,
    {
        let type_id = TypeId::of::<R>();
        self.resources.insert(type_id, Box::new(resource));
        self
    }

    /// Registers the given command as a trigger of this schema.
    ///
    /// This method works out of the box if, and only if, the trigger implements command for a single context.
    /// Otherwise it requires to specify the context for which the trigger is being registered.
    /// See [`Self::on_context`] for a better user experience.
    pub fn with_trigger<Ctx, Args, Err>(
        mut self,
        trigger: impl CommandRef<'static, Ctx, Args, Err = Err> + 'static,
    ) -> Self
    where
        Ctx: 'static,
        Args: 'static,
        Err: 'static,
    {
        let trigger: Box<dyn CommandRef<Ctx, Err = Err>> = Box::new(Trigger::from(trigger));
        self.triggers.push(Box::new(trigger));
        self
    }

    /// Pre-selects a context for which a trigger is going to be registered.
    ///
    /// This is the the two-steps equivalent of [`Self::with_trigger`].
    /// Its main purpose is to enhance the Schema's API by reducing placeholders, which improves readability.
    ///
    /// ```ignore
    /// // Both forms are equivalent.
    /// schema.with_trigger<MyCtx, _, _>(MyTrigger);
    /// schema.on_context<MyCtx>().trigger(MyTrigger);
    /// ```
    pub fn on_context<Ctx>(self) -> OnContext<T, Ctx> {
        self.into()
    }

    /// Returns an iterator over the triggers in the schema implementing the corresponding command.
    pub fn triggers<Ctx, Err>(
        &self,
    ) -> impl Iterator<Item = &dyn CommandRef<'static, Ctx, Err = Err>>
    where
        Ctx: 'static,
        Err: 'static,
    {
        self.triggers
            .iter()
            .filter_map(|trigger| trigger.downcast_ref::<Box<dyn CommandRef<Ctx, Err = Err>>>())
            .map(AsRef::as_ref)
    }

    /// Returns a read-only access to the schema.
    #[inline]
    pub fn read(&self) -> SchemaReadGuard<'_, T> {
        self.into()
    }

    /// Returns a read-write access to the schema.
    #[inline]
    pub fn write(&self) -> SchemaWriteGuard<'_, T> {
        self.into()
    }
}

#[cfg(test)]
mod tests {
    use std::{convert::Infallible, marker::PhantomData};

    use crate::{graph::Graph, id::fixtures::IndentifyMock, schema::Schema};

    #[test]
    fn triggers_downcasting() {
        struct Foo;
        struct ContextFoo;
        impl From<&ContextFoo> for Foo {
            fn from(_: &ContextFoo) -> Self {
                Foo
            }
        }

        struct Bar<'a>(PhantomData<&'a ()>);
        struct ContextBar<'a>(PhantomData<&'a ()>);
        impl<'a> From<&ContextBar<'a>> for Bar<'a> {
            fn from(_: &ContextBar) -> Self {
                Bar(PhantomData)
            }
        }

        impl<'a> From<&ContextFoo> for Bar<'a> {
            fn from(_: &ContextFoo) -> Self {
                Bar(PhantomData)
            }
        }

        struct Qux;
        struct ContextQux;
        impl From<&ContextQux> for Qux {
            fn from(_: &ContextQux) -> Self {
                Qux
            }
        }

        fn a_trigger(_: Foo) -> Result<(), Infallible> {
            Ok(())
        }

        fn another_trigger(_: Bar) -> Result<(), Infallible> {
            Ok(())
        }

        /// Is a trigger because Foo and Bar implement Command for the same context.
        fn even_another_trigger(_: Foo, _: Bar) -> Result<(), Infallible> {
            Ok(())
        }

        // Is not a trigger because Foo and Qux implement Command for different contexts.
        fn _not_a_trigger(_: Foo, _: Qux) -> Result<(), Infallible> {
            Ok(())
        }

        let schema = Schema::from(Graph::<IndentifyMock<usize>>::default())
            // .with_trigger(_not_a_trigger)
            .with_trigger(a_trigger)
            .on_context::<ContextBar>()
            .trigger(another_trigger)
            .on_context::<ContextFoo>()
            .trigger(another_trigger)
            .with_trigger(even_another_trigger);

        assert_eq!(schema.triggers::<ContextFoo, Infallible>().count(), 3);
        assert_eq!(schema.triggers::<ContextBar, Infallible>().count(), 1);

        // There is no trigger taking usize as context.
        assert_eq!(schema.triggers::<usize, Infallible>().count(), 0);
    }
}
