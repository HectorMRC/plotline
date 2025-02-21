//! Schema representation.

mod error;
pub use error::{Error, Result};
pub mod guard;
pub mod ops;
pub mod plugin;
pub mod resource;
pub mod transaction;
pub mod trigger;

use std::sync::RwLock;

use guard::{SchemaReadGuard, SchemaWriteGuard};
use plugin::Plugin;
use resource::ResourceSet;
use transaction::Background;
use trigger::{Trigger, TriggerSet};

use crate::{graph::Graph, id::Identify};

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
    triggers: TriggerSet<T>,
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
        plugin.install(self)
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

    /// Schedules the given trigger in this schema.
    pub fn with_trigger<S, Args>(
        mut self,
        scheduler: S,
        trigger: impl Trigger<T, Args> + 'static,
    ) -> Self
    where
        T: 'static,
        S: 'static,
        Args: 'static,
    {
        self.triggers = self.triggers.with_trigger(scheduler, trigger);
        self
    }

    /// Returns the resource set of this schema.
    pub fn resources(&self) -> &ResourceSet {
        &self.resources
    }

    /// Returns the trigger set of this schema.
    pub fn triggers(&self) -> &TriggerSet<T> {
        &self.triggers
    }

    /// Returns a new transaction background.
    #[inline]
    pub fn transaction(&self) -> Background<'_, T> {
        self.into()
    }

    #[inline]
    pub fn read(&self) -> SchemaReadGuard<'_, T> {
        self.into()
    }

    #[inline]
    pub fn write(&self) -> SchemaWriteGuard<'_, T> {
        self.into()
    }
}
