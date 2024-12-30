//! Trigger helpers.

use std::{any::TypeId, marker::PhantomData};

use crate::id::Identify;

use super::{
    transaction::{Context, Ctx},
    Result,
};

/// Represents a trigger that can be executed under a [`Context`].
pub trait Trigger<T, Args>
where
    T: Identify,
{
    /// Executes the trigger.
    fn execute(&self, ctx: &Context<'_, T>) -> Result<()>;
}

#[macro_export]
macro_rules! impl_trigger {
    ($($args:tt),*) => {
        impl<_T, _F, $($args),*> Trigger<_T, ($($args,)*)> for _F
        where
            _T: Identify,
            _F: Fn(Ctx<_T>, $($args),*) -> Result<()>,
            $($args: for<'a> From<&'a Context<'a, _T>>),*
        {
            fn execute(&self, _ctx: &Context<'_, _T>) -> Result<()> {
                (self)(_ctx.into(), $($args::from(_ctx)),*)
            }
        }
    };
}

pub use impl_trigger;

impl_trigger!();
impl_trigger!(A);
impl_trigger!(A, B);
impl_trigger!(A, B, C);
impl_trigger!(A, B, C, D);
impl_trigger!(A, B, C, D, E);
impl_trigger!(A, B, C, D, E, F);
impl_trigger!(A, B, C, D, E, F, G);
impl_trigger!(A, B, C, D, E, F, G, H);

/// A set of arbitrary triggers.
pub struct TriggerSet<T> {
    triggers: Vec<(TypeId, Box<dyn Trigger<T, ()>>)>,
    _node: PhantomData<T>,
}

impl<T> Default for TriggerSet<T> {
    fn default() -> Self {
        Self {
            triggers: Default::default(),
            _node: Default::default(),
        }
    }
}

impl<T> TriggerSet<T> {
    /// Schedules a new trigger.
    pub fn with_trigger<S, Args>(mut self, _: S, trigger: impl Trigger<T, Args> + 'static) -> Self
    where
        T: 'static + Identify,
        S: 'static,
        Args: 'static,
    {
        let trigger: Box<dyn Trigger<T, ()>> = Box::new(ArglessTrigger::from(trigger));
        self.triggers.push((TypeId::of::<S>(), trigger));

        self
    }

    /// Returns an iterator over the triggers scheduled for the given type.
    pub fn select<S>(&self) -> impl Iterator<Item = &dyn Trigger<T, ()>>
    where
        S: 'static,
    {
        self.triggers
            .iter()
            .filter_map(|(type_id, trigger)| (&TypeId::of::<S>() == type_id).then_some(trigger))
            .map(AsRef::as_ref)
    }
}

/// Wraps the trigger's [`CommandRef`] into an argless implementation of [`CommandRef`].
///
/// This wraper is useful when downcasting triggers from `Box<dyn Any>`.
/// It allows selecting all the triggers for a specific context and error type, no matter the arguments.
pub(crate) struct ArglessTrigger<T, M> {
    trigger: T,
    _meta: PhantomData<M>,
}

impl<T, M> From<T> for ArglessTrigger<T, M> {
    fn from(trigger: T) -> Self {
        Self {
            trigger,
            _meta: PhantomData,
        }
    }
}

impl<T, Tr, Args> Trigger<T, ()> for ArglessTrigger<Tr, Args>
where
    T: Identify,
    Tr: Trigger<T, Args>,
{
    fn execute(&self, ctx: &Context<'_, T>) -> Result<()> {
        self.trigger.execute(ctx)
    }
}

#[cfg(test)]
mod tests {
    use std::marker::PhantomData;

    use crate::{
        graph::Graph,
        id::fixtures::IndentifyMock,
        schema::{
            transaction::{Context, Ctx},
            Result, Schema,
        },
    };

    #[test]
    fn triggers_downcasting() {
        type Node = IndentifyMock<'static, usize>;

        struct Foo;
        impl From<&Context<'_, Node>> for Foo {
            fn from(_: &Context<'_, Node>) -> Self {
                Foo
            }
        }

        struct Bar<'a>(PhantomData<&'a ()>);
        impl<'a> From<&Context<'_, Node>> for Bar<'a> {
            fn from(_: &Context<'_, Node>) -> Self {
                Bar(PhantomData)
            }
        }

        fn a_trigger(_: Ctx<Node>, _: Foo) -> Result<()> {
            Ok(())
        }

        fn another_trigger<'a>(_: Ctx<Node>, _: Foo, _: Bar) -> Result<()> {
            Ok(())
        }

        struct Schedule1;
        struct Schedule2;

        let schema = Schema::from(Graph::<IndentifyMock<usize>>::default())
            .with_trigger(Schedule1, a_trigger)
            .with_trigger(Schedule1, another_trigger)
            .with_trigger(Schedule2, another_trigger);

        assert_eq!(schema.triggers().select::<Schedule1>().count(), 2);
        assert_eq!(schema.triggers().select::<Schedule2>().count(), 1);

        assert_eq!(schema.triggers().select::<usize>().count(), 0);
    }
}
