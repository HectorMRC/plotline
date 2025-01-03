//! Trigger helpers.

use std::{any::TypeId, collections::BTreeMap, marker::PhantomData};

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
            fn execute(&self, ctx: &Context<'_, _T>) -> Result<()> {
                (self)(ctx.into(), $($args::from(ctx)),*)
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

/// Implements the [`Trigger`] trait for a selection of triggers.
pub struct TriggerSelect<'a, T> {
    triggers: Option<&'a [Box<dyn Trigger<T, ()>>]>,
}

impl<I> Default for TriggerSelect<'_, I> {
    fn default() -> Self {
        Self {
            triggers: Default::default(),
        }
    }
}

impl<'a, T> Trigger<T, ()> for TriggerSelect<'a, T>
where
    T: 'a + Identify,
{
    fn execute(&self, ctx: &Context<'_, T>) -> Result<()> {
        let Some(triggers) = self.triggers else {
            return Ok(());
        };

        triggers.iter().try_for_each(|trigger| trigger.execute(ctx))
    }
}

/// A set of arbitrary triggers.
pub struct TriggerSet<T> {
    triggers: BTreeMap<TypeId, Vec<Box<dyn Trigger<T, ()>>>>,
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

impl<T> TriggerSet<T>
where
    T: Identify,
{
    /// Schedules a new trigger.
    pub fn with_trigger<S, Args>(mut self, _: S, trigger: impl Trigger<T, Args> + 'static) -> Self
    where
        T: 'static,
        S: 'static,
        Args: 'static,
    {
        let trigger: Box<dyn Trigger<T, ()>> = Box::new(ArglessTrigger::from(trigger));
        let scheduler = TypeId::of::<S>();

        match self.triggers.get_mut(&scheduler) {
            Some(triggers) => triggers.push(trigger),
            None => {
                self.triggers.insert(scheduler, vec![trigger]);
            }
        };

        self
    }

    /// Returns an iterator over the triggers scheduled for the given type.
    pub fn select<S>(&self, _: S) -> TriggerSelect<'_, T>
    where
        S: 'static,
    {
        let Some(triggers) = self.triggers.get(&TypeId::of::<S>()) else {
            return TriggerSelect::default();
        };

        TriggerSelect {
            triggers: Some(triggers.as_slice()),
        }
    }
}

/// Wraps a trigger into an argless implementation of [`Trigger`].
struct ArglessTrigger<T, M> {
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
    use std::{
        marker::PhantomData,
        sync::atomic::{AtomicUsize, Ordering},
    };

    use crate::{
        graph::Graph,
        id::fixtures::IndentifyMock,
        prelude::Transaction,
        schema::{
            transaction::{Context, Ctx},
            trigger::Trigger,
            Result, Schema,
        },
    };

    #[test]
    fn triggers_downcasting() {
        static COUNT: AtomicUsize = AtomicUsize::new(0);

        type Node = IndentifyMock<'static, usize>;

        struct Foo;
        impl From<&Context<'_, Node>> for Foo {
            fn from(_: &Context<'_, Node>) -> Self {
                Foo
            }
        }

        struct Bar<'a>(PhantomData<&'a ()>);
        impl From<&Context<'_, Node>> for Bar<'_> {
            fn from(_: &Context<'_, Node>) -> Self {
                Bar(PhantomData)
            }
        }

        fn a_trigger(_: Ctx<Node>, _: Foo) -> Result<()> {
            COUNT.fetch_add(1, Ordering::Relaxed);
            Ok(())
        }

        fn another_trigger(_: Ctx<Node>, _: Foo, _: Bar) -> Result<()> {
            COUNT.fetch_add(1, Ordering::Relaxed);
            Ok(())
        }

        struct Schedule1;
        struct Schedule2;

        let schema = Schema::from(Graph::<IndentifyMock<usize>>::default())
            .with_trigger(Schedule1, a_trigger)
            .with_trigger(Schedule1, another_trigger)
            .with_trigger(Schedule2, another_trigger);

        schema
            .transaction()
            .with(|ctx| schema.triggers().select(Schedule1).execute(&ctx))
            .expect("transaction should not fail");

        assert_eq!(
            COUNT.load(Ordering::Relaxed),
            2,
            "all scheduled triggers should be executed"
        );

        COUNT.store(0, Ordering::Relaxed);
        schema
            .transaction()
            .with(|ctx| schema.triggers().select(Schedule2).execute(&ctx))
            .expect("transaction should not fail");

        assert_eq!(
            COUNT.load(Ordering::Relaxed),
            1,
            "only scheduled triggers should be executed"
        );
    }
}
