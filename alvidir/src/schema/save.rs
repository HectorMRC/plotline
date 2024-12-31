//! Save transaction.

use crate::id::Identify;

use super::{transaction::Transaction, Result};

pub struct BeforeSave;

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
        {
            let ctx = tx.begin().with_target(self.node);
            ctx.triggers()
                .select::<BeforeSave>()
                .try_for_each(|trigger| trigger.execute(&ctx))?;

            ctx.target().with(|node| ctx.save(node.clone()));

            ctx.triggers()
                .select::<AfterSave>()
                .try_for_each(|trigger| trigger.execute(&ctx))?;
        }

        tx.commit();
        Ok(())
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
