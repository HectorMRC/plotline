//! Delete transaction.

use crate::{deref::TryDeref, id::Identify};

use super::{transaction::Transaction, Result};

pub struct BeforeDelete;

pub struct AfterDelete;

/// A delete transaction for a node from a schema.
pub struct Delete<T>
where
    T: Identify,
{
    /// The id of the node being deleted from the schema.
    pub node_id: T::Id,
}

impl<T> Delete<T>
where
    T: Identify + Clone,
    T::Id: Ord + Clone,
{
    /// Executes the [`Delete`] transaction.
    pub fn execute(self, tx: impl Transaction<Target = T>) -> Result<()> {
        {
            let ctx = tx.begin();
            let Some(node) = ctx.node(self.node_id.clone()).try_deref().cloned() else {
                tracing::warn_span!("node does not exists");
                return Ok(());
            };

            let ctx = ctx.with_target(node);
            ctx.triggers()
                .select::<BeforeDelete>()
                .try_for_each(|trigger| trigger.execute(&ctx))?;

            ctx.delete(self.node_id);
            ctx.triggers()
                .select::<AfterDelete>()
                .try_for_each(|trigger| trigger.execute(&ctx))?;
        }

        tx.commit();
        Ok(())
    }
}

impl<T> Delete<T>
where
    T: Identify,
{
    pub fn new(node_id: T::Id) -> Self {
        Self { node_id }
    }
}
