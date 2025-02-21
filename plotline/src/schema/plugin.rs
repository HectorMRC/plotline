//! Plugin definition.

use crate::id::Identify;

use super::Schema;

/// A pluggin to be installed into a schema.
pub trait Plugin<T> {
    /// Allows the plugin to be initialized in the given schema.
    fn install(self, schema: Schema<T>) -> Schema<T>
    where
        T: Identify;
}
