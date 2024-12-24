//! Trigger helpers.

use std::marker::PhantomData;

use crate::{command::CommandRef, id::Identify};

use super::Schema;

/// A helper type that allows T to improve its trigger API.
pub struct WithTrigger<T> {
    pub inner: T,
}

impl<T> From<T> for WithTrigger<T> {
    fn from(inner: T) -> Self {
        WithTrigger { inner }
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

/// Allows to register a trigger into the schema under a pre-selected context.
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
    /// Registers the given command as a trigger of the schema.
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
