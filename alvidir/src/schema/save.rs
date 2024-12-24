//! Save transaction.

use alvidir_macros::with_trigger;

use crate::{
    chain::LiFoChain,
    command::{Command, NoopCommand},
    graph::Graph,
    id::Identify,
};

use super::{trigger::WithTrigger, Schema};

/// The context for triggers before saving.
pub struct NodeToSave<'a, T>
where
    T: Identify,
{
    /// The graph the node is being saved into.
    pub graph: &'a Graph<T>,
    /// The node being saved into the schema.
    pub node: T,
}

/// The context for triggers after saving.
pub struct SavedNode<'a, T>
where
    T: Identify,
{
    /// The schema the node has been saved into.
    pub schema: &'a Schema<T>,
    /// The id of the saved node.
    pub node_id: T::Id,
}

/// A save transaction for a node into a schema.
#[with_trigger]
pub struct Save<T>
where
    T: Identify,
{
    /// The node being saved into the schema.
    pub node: T,
}

impl<T, B, A, E, BArgs, AArgs> Command<'_, Schema<T>, (BArgs, AArgs)> for Save<T, B, A>
where
    T: 'static + Identify,
    T::Id: Ord + Clone,
    B: for<'b> Command<'b, NodeToSave<'b, T>, BArgs, Err = E>,
    A: for<'a> Command<'a, SavedNode<'a, T>, AArgs, Err = E>,
{
    type Err = E;

    /// Executes the [`Save`] transaction.
    ///
    /// ### Before
    /// Before performing the save this transaction executes the before command.
    /// If the before command fails, the whole transaction is aborted and the trigger's error is returned as the transaction's result.
    ///
    /// ### After
    /// Once the save has been completed, this transaction executes the after command.
    /// If the after command fails the transaction __DOES NOT__ rollback, but the resulting error is retrived as the transaction's result.
    fn execute(self, schema: &Schema<T>) -> Result<(), Self::Err> {
        let inserted_id: <T as Identify>::Id = {
            let mut graph = schema.write();

            let final_node = {
                let payload = NodeToSave {
                    graph: &graph,
                    node: self.node,
                };

                self.before.execute(&payload)?;
                payload.node
            };

            let inserted_id = final_node.id().clone();
            graph.insert(final_node);

            inserted_id
        };

        let payload = SavedNode {
            schema,
            node_id: inserted_id,
        };

        self.after.execute(&payload)
    }
}

impl<T> Save<T, NoopCommand, NoopCommand>
where
    T: Identify,
{
    pub fn new(node: T) -> Self {
        Self {
            node,
            before: NoopCommand,
            after: NoopCommand,
        }
    }
}
