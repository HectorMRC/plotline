//! Deletion transaction.

use alvidir_macros::with_trigger;

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
#[with_trigger]
pub struct Delete<T>
where
    T: Identify,
{
    /// The id of the node being deleted from the schema.
    pub node_id: T::Id,
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
                    tracing::error!(error = poisoned.to_string(), "posioned graph");
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
