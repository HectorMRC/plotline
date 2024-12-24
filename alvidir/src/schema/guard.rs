//! Schema guard definition.

use std::{
    ops::{Deref, DerefMut},
    sync::{RwLockReadGuard, RwLockWriteGuard},
};

use crate::{graph::Graph, id::Identify};

use super::Schema;

/// A read-only access to a schema.
pub struct SchemaReadGuard<'a, T>
where
    T: Identify,
{
    guard: RwLockReadGuard<'a, Graph<T>>,
}

impl<T> Deref for SchemaReadGuard<'_, T>
where
    T: Identify,
{
    type Target = Graph<T>;

    fn deref(&self) -> &Self::Target {
        &self.guard
    }
}

impl<'a, T> From<&'a Schema<T>> for SchemaReadGuard<'a, T>
where
    T: Identify,
{
    fn from(schema: &'a Schema<T>) -> Self {
        SchemaReadGuard {
            guard: match schema.graph.read() {
                Ok(graph) => graph,
                Err(poisoned) => {
                    tracing::error!(error = poisoned.to_string(), "posioned graph");
                    poisoned.into_inner()
                }
            },
        }
    }
}

/// A read-write access to a schema.
pub struct SchemaWriteGuard<'a, T>
where
    T: Identify,
{
    guard: RwLockWriteGuard<'a, Graph<T>>,
}

impl<T> Deref for SchemaWriteGuard<'_, T>
where
    T: Identify,
{
    type Target = Graph<T>;

    fn deref(&self) -> &Self::Target {
        &self.guard
    }
}

impl<T> DerefMut for SchemaWriteGuard<'_, T>
where
    T: Identify,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.guard
    }
}

impl<'a, T> From<&'a Schema<T>> for SchemaWriteGuard<'a, T>
where
    T: Identify,
{
    fn from(schema: &'a Schema<T>) -> Self {
        SchemaWriteGuard {
            guard: match schema.graph.write() {
                Ok(graph) => graph,
                Err(poisoned) => {
                    tracing::error!(error = poisoned.to_string(), "posioned graph");
                    poisoned.into_inner()
                }
            },
        }
    }
}
