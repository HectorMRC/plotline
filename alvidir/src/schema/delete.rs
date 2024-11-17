//! Deletion transaction.

use crate::{
    chain::LiFoChain,
    command::{Command, NoopCommand},
    graph::Graph,
    id::Identify,
};

use super::{trigger::WithTrigger, Schema};

/// The context for the before-deletion triggers.
pub struct NodeToDelete<'a, T>
where
    T: Identify,
{
    /// The graph from which the node is being deleted.
    pub graph: &'a Graph<T>,
    /// The id of the node to delete.
    pub node_id: &'a T::Id,
}

/// The context for the after-deletion triggers.
pub struct DeletedNode<'a, T>
where
    T: Identify,
{
    /// The schema in which the node has been inserted.
    pub schema: &'a Schema<T>,
    /// The deleted node.
    pub node: T,
}

/// A deletion transaction for a node from a schema.
pub struct Delete<T, B, A>
where
    T: Identify,
{
    /// The id of the node being deleted from the schema.
    pub node_id: T::Id,
    /// The command to execute before deleting the node.
    ///
    /// If this command fails the whole transaction is aborted.
    pub before: B,
    /// The command to execute once the deletion has been performed.
    ///
    /// If this command fails the transaction IS NOT rollbacked, but the resulting error is retrived as the transaction's result.
    pub after: A,
}

impl<T, B, A, E> Command<Schema<T>> for Delete<T, B, A>
where
    T: 'static + Identify,
    T::Id: Ord,
    B: for<'b> Command<NodeToDelete<'b, T>, Err = E>,
    A: for<'a> Command<DeletedNode<'a, T>, Err = E>,
{
    type Err = E;

    fn execute(self, schema: &Schema<T>) -> Result<(), Self::Err> {
        let deleted_node = {
            let mut graph = match schema.graph.write() {
                Ok(graph) => graph,
                Err(poisoned) => {
                    tracing::error!("posioned graph has been recovered");
                    poisoned.into_inner()
                }
            };

            {
                let payload = NodeToDelete {
                    graph: &graph,
                    node_id: &self.node_id,
                };

                self.before.execute(&payload)?;
            }

            graph.remove(&self.node_id)
        };

        let Some(node) = deleted_node else {
            return Ok(());
        };

        let payload = DeletedNode { schema, node };

        self.after.execute(&payload)
    }
}

impl<T> Delete<T, NoopCommand, NoopCommand>
where
    T: Identify,
{
    pub fn new(node_id: T::Id) -> Self {
        Self {
            node_id,
            before: NoopCommand,
            after: NoopCommand,
        }
    }
}

impl<T, B, A> Delete<T, B, A>
where
    T: Identify,
{
    /// Configure triggers for this transaction.
    pub fn with_trigger(self) -> WithTrigger<Self> {
        self.into()
    }
}

impl<T, B, A> WithTrigger<Delete<T, B, A>>
where
    T: Identify,
{
    /// Configures the given command as a before insertion trigger.
    pub fn before<C>(self, command: C) -> Delete<T, LiFoChain<C, B>, A> {
        Delete {
            node_id: self.inner.node_id,
            before: LiFoChain {
                head: self.inner.before,
                value: command,
            },
            after: self.inner.after,
        }
    }
}

impl<T, B, A> WithTrigger<Delete<T, B, A>>
where
    T: Identify,
{
    /// Configures the given command as an after insertion trigger.
    pub fn after<C>(self, command: C) -> Delete<T, B, LiFoChain<C, A>> {
        Delete {
            node_id: self.inner.node_id,
            before: self.inner.before,
            after: LiFoChain {
                head: self.inner.after,
                value: command,
            },
        }
    }
}
