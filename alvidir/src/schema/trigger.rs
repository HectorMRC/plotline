//! Trigger helpers.

use std::{any::Any, marker::PhantomData};

use crate::{command::CommandRef, id::Identify};

use super::Schema;

/// A set of arbitrary triggers.
#[derive(Debug, Default)]
pub struct TriggerSet {
    triggers: Vec<Box<dyn Any>>,
}

impl TriggerSet {
    /// Registers the given command as a trigger.
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

    /// Returns an iterator over the triggers implementing the corresponding command.
    pub fn select<Ctx, Err>(&self) -> impl Iterator<Item = &dyn CommandRef<'static, Ctx, Err = Err>>
    where
        Ctx: 'static,
        Err: 'static,
    {
        self.triggers
            .iter()
            .filter_map(|trigger| trigger.downcast_ref::<Box<dyn CommandRef<Ctx, Err = Err>>>())
            .map(AsRef::as_ref)
    }
}

/// Wraps the trigger's [`CommandRef`] into an argless implementation of [`CommandRef`].
///
/// This wraper is useful when downcasting triggers from `Box<dyn Any>`.
/// It allows selecting all the triggers for a specific context and error type, no matter the arguments.
pub(crate) struct Trigger<Cmd, M> {
    command: Cmd,
    _meta: PhantomData<M>,
}

impl<Cmd, M> From<Cmd> for Trigger<Cmd, M> {
    fn from(command: Cmd) -> Self {
        Self {
            command,
            _meta: PhantomData,
        }
    }
}

impl<'a, Cmd, Ctx, Args, Err> CommandRef<'a, Ctx> for Trigger<Cmd, (Ctx, Args, Err)>
where
    Cmd: CommandRef<'a, Ctx, Args, Err = Err>,
{
    type Err = Err;

    fn execute(&self, ctx: &'a Ctx) -> Result<(), Self::Err> {
        self.command.execute(ctx)
    }
}

/// Allows to register a trigger under a pre-selected context.
pub struct OnContext<T, Ctx>
where
    T: Identify,
{
    schema: Schema<T>,
    context: PhantomData<Ctx>,
}

impl<T, Ctx> From<Schema<T>> for OnContext<T, Ctx>
where
    T: Identify,
{
    fn from(schema: Schema<T>) -> Self {
        Self {
            schema,
            context: PhantomData,
        }
    }
}

impl<T, Ctx> OnContext<T, Ctx>
where
    T: 'static + Identify,
    Ctx: 'static,
{
    /// Registers the given command as a trigger.
    pub fn trigger<Args, Err>(
        self,
        trigger: impl CommandRef<'static, Ctx, Args, Err = Err> + 'static,
    ) -> Schema<T>
    where
        Args: 'static,
        Err: 'static,
    {
        self.schema.with_trigger::<Ctx, _, _>(trigger)
    }
}

/// A helper type that allows T to improve its trigger API.
pub struct WithTrigger<T> {
    pub inner: T,
}

impl<T> From<T> for WithTrigger<T> {
    fn from(inner: T) -> Self {
        WithTrigger { inner }
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

        assert_eq!(
            schema.triggers().select::<ContextFoo, Infallible>().count(),
            3
        );
        assert_eq!(
            schema.triggers().select::<ContextBar, Infallible>().count(),
            1
        );

        // There is no trigger taking usize as context.
        assert_eq!(schema.triggers().select::<usize, Infallible>().count(), 0);
    }
}
