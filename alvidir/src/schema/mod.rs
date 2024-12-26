//! Schema representation.

pub mod delete;
pub mod guard;
pub mod plugin;
pub mod resource;
pub mod save;
pub mod trigger;

use std::sync::RwLock;

use guard::{SchemaReadGuard, SchemaWriteGuard};
use plugin::Plugin;
use resource::ResourceSet;
use trigger::{OnContext, TriggerSet};

use crate::{command::CommandRef, graph::Graph, id::Identify};

/// A graph that is subject to a set of rules.
pub struct Schema<T>
where
    T: Identify,
{
    /// The graph being orchestrated by this schema.
    graph: RwLock<Graph<T>>,
    /// All the resources in this schema.
    resources: ResourceSet,
    /// All the triggers in the schema.
    triggers: TriggerSet,
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
        self.resources = self.resources.with_resource(resource);
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
        self.triggers = self.triggers.with_trigger(trigger);
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

    /// Returns the resource set of this schema.
    pub fn resources(&self) -> &ResourceSet {
        &self.resources
    }

    /// Returns the trigger set of this schema.
    pub fn triggers(&self) -> &TriggerSet {
        &self.triggers
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
