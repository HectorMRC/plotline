//! Insertion transaction.

use std::cell::RefCell;

use alvidir_macros::with_trigger;

use crate::{
    chain::LiFoChain,
    command::{Command, NoopCommand},
    graph::Graph,
    id::Identify,
};

use super::{trigger::WithTrigger, Schema};

/// The context for the before-insertion triggers.
pub struct NodeToInsert<'a, T>
where
    T: Identify,
{
    /// The graph in which the node is being inserted.
    pub graph: &'a Graph<T>,
    /// The node being inserted into the schema.
    pub node: RefCell<T>,
}

/// The context for the after-insertion triggers.
pub struct InsertedNode<'a, T>
where
    T: Identify,
{
    /// The schema in which the node has been inserted.
    pub schema: &'a Schema<T>,
    /// The id of the inserted node.
    pub node_id: T::Id,
}

/// An insertion transaction for a node into a schema.
#[with_trigger]
pub struct Insert<T>
where
    T: Identify,
{
    /// The node being inserted into the schema.
    pub node: T,
}

impl<T, B, A, E> Command<Schema<T>> for Insert<T, B, A>
where
    T: 'static + Identify,
    T::Id: Ord + Clone,
    B: for<'b> Command<NodeToInsert<'b, T>, Err = E>,
    A: for<'a> Command<InsertedNode<'a, T>, Err = E>,
{
    type Err = E;

    fn execute(self, schema: &Schema<T>) -> Result<(), Self::Err> {
        let inserted_id = {
            let mut graph = match schema.graph.write() {
                Ok(graph) => graph,
                Err(poisoned) => {
                    tracing::error!(error = poisoned.to_string(), "posioned graph");
                    poisoned.into_inner()
                }
            };

            let final_node = {
                let payload = NodeToInsert {
                    graph: &graph,
                    node: RefCell::new(self.node),
                };

                self.before.execute(&payload)?;
                payload.node
            }
            .into_inner();

            let inserted_id = final_node.id().clone();
            graph.insert(final_node);

            inserted_id
        };

        let payload = InsertedNode {
            schema,
            node_id: inserted_id,
        };

        self.after.execute(&payload)
    }
}

impl<T> Insert<T, NoopCommand, NoopCommand>
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
