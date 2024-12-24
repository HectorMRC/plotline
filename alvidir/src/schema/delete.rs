//! Delete transaction.

use alvidir_macros::with_trigger;

use crate::{
    chain::LiFoChain,
    command::{Command, NoopCommand},
    deref::TryDeref,
    graph::Graph,
    id::Identify,
};

use super::{trigger::WithTrigger, Schema};

/// The context for trigers before delete.
pub struct NodeToDelete<'a, T>
where
    T: Identify,
{
    /// The graph the node is being deleted from.
    pub graph: &'a Graph<T>,
    /// The id of the node to delete.
    pub node: &'a T,
}

/// The context for triggers after delete.
pub struct DeletedNode<'a, T>
where
    T: Identify,
{
    /// The schema the node has been deleted from.
    pub schema: &'a Schema<T>,
    /// The deleted node.
    pub node: T,
}

/// A delete transaction for a node from a schema.
#[with_trigger]
pub struct Delete<T>
where
    T: Identify,
{
    /// The id of the node being deleted from the schema.
    pub node_id: T::Id,
}

impl<T, B, A, E, BArgs, AArgs> Command<'_, Schema<T>, (BArgs, AArgs)> for Delete<T, B, A>
where
    T: 'static + Identify,
    T::Id: Ord + Clone,
    B: for<'b> Command<'b, NodeToDelete<'b, T>, BArgs, Err = E>,
    A: for<'a> Command<'a, DeletedNode<'a, T>, AArgs, Err = E>,
{
    type Err = E;

    /// Executes the [`Delete`] transaction.
    ///
    /// ### Before
    /// Before performing the delete this transaction executes the before command.
    /// If the before command fails, the whole transaction is aborted and the trigger's error is returned as the transaction's result.
    ///
    /// ### After
    /// Once the delete has been completed, this transaction executes the after command.
    /// If the after command fails the transaction __DOES NOT__ rollback, but the resulting error is retrived as the transaction's result.
    fn execute(self, schema: &Schema<T>) -> Result<(), Self::Err> {
        let deleted_node = {
            let mut graph = schema.write();

            let node = graph.node(self.node_id.clone());
            let Some(node) = node.try_deref() else {
                return Ok(());
            };

            {
                let payload = NodeToDelete {
                    graph: &graph,
                    node,
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
