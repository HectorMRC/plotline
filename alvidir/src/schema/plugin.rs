//! Plugin definition.

use crate::id::Identify;

use super::Schema;

/// A pluggin to be installed into a schema.
pub trait Plugin<T> {
    /// Allows the plugin to get initialized.
    ///
    /// This method is called during the installation of the plugin in that
    /// schema given as parameter.
    fn setup(&self, schema: Schema<T>) -> Schema<T>
    where
        T: Identify;
}
