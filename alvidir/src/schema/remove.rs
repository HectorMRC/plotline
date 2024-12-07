//! Remove transaction.

use alvidir_macros::with_trigger;

use crate::{
    chain::LiFoChain,
    command::{Command, NoopCommand},
    graph::Graph,
    id::Identify,
};

use super::{trigger::WithTrigger, Schema};

/// The context for trigers before removal.
pub struct NodeToRemove<'a, T>
where
    T: Identify,
{
    /// The graph the node is being removed from.
    pub graph: &'a Graph<T>,
    /// The id of the node to remove.
    pub node_id: &'a T::Id,
}

/// The context for triggers after removal.
pub struct RemovedNode<'a, T>
where
    T: Identify,
{
    /// The schema the node has been removed from.
    pub schema: &'a Schema<T>,
    /// The removed node.
    pub node: T,
}

/// A remove transaction for a node from a schema.
#[with_trigger]
pub struct Remove<T>
where
    T: Identify,
{
    /// The id of the node being removed from the schema.
    pub node_id: T::Id,
}

impl<T, B, A, E, BArgs, AArgs> Command<Schema<T>, (BArgs, AArgs)> for Remove<T, B, A>
where
    T: 'static + Identify,
    T::Id: Ord,
    B: for<'b> Command<NodeToRemove<'b, T>, BArgs, Err = E>,
    A: for<'a> Command<RemovedNode<'a, T>, AArgs, Err = E>,
{
    type Err = E;

    fn execute(self, schema: &Schema<T>) -> Result<(), Self::Err> {
        let removed_node = {
            let mut graph = schema.write();

            {
                let payload = NodeToRemove {
                    graph: &graph,
                    node_id: &self.node_id,
                };

                self.before.execute(&payload)?;
            }

            graph.remove(&self.node_id)
        };

        let Some(node) = removed_node else {
            return Ok(());
        };

        let payload = RemovedNode { schema, node };

        self.after.execute(&payload)
    }
}

impl<T> Remove<T, NoopCommand, NoopCommand>
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
