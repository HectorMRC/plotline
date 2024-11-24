//! Schema representation.

pub mod delete;
pub mod insert;
pub mod trigger;

use std::sync::{RwLock, RwLockWriteGuard};

use crate::{graph::Graph, id::Identify};

/// A graph that is subject to a set of rules.
pub struct Schema<T>
where
    T: Identify,
{
    /// The graph being orchestrated by this schema.
    graph: RwLock<Graph<T>>,
}

impl<T> From<Graph<T>> for Schema<T>
where
    T: Identify,
{
    fn from(graph: Graph<T>) -> Self {
        Self {
            graph: RwLock::new(graph),
        }
    }
}

impl<T> Schema<T>
where
    T: Identify,
{
    /// Returns a [`RwLockWriteGuard`] of the inner graph even if the [`RwLock`] was poisoned.
    pub fn write(&self) -> RwLockWriteGuard<'_, Graph<T>> {
        match self.graph.write() {
            Ok(graph) => graph,
            Err(poisoned) => {
                tracing::error!(error = poisoned.to_string(), "posioned graph");
                poisoned.into_inner()
            }
        }
    }
}
