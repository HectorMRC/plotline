//! Save transaction.

use crate::{
    id::Identify,
    prelude::Transaction,
    schema::{trigger::Trigger, Result},
};

/// Schedules a trigger before a save is performed.
pub struct BeforeSave;

/// Schedules a trigger after a save is performed.
pub struct AfterSave;

/// A save transaction for a node into a schema.
pub struct Save<T> {
    /// The node being saved into the schema.
    pub node: T,
}

impl<T> Save<T> {
    /// Executes the [`Save`] transaction.
    pub fn execute(self, tx: impl Transaction<Target = T>) -> Result<()>
    where
        T: Identify + Clone,
    {
        tx.with(|ctx| {
            let ctx = ctx.with_target(self.node);

            ctx.triggers().select(BeforeSave).execute(&ctx)?;

            ctx.target().with(|node| ctx.save(node.clone()));

            ctx.triggers().select(AfterSave).execute(&ctx)?;

            Ok(())
        })
    }
}

impl<T> Save<T>
where
    T: Identify,
{
    pub fn new(node: T) -> Self {
        Self { node }
    }
}
