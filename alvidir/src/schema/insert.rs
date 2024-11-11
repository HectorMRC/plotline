//! Insertion request.

use std::cell::RefCell;

use crate::{command::Command, graph::Graph, id::Identify};

use super::Schema;

/// An insertion transaction for a node into a schema.
pub struct Insert<T, B, A>
where
    T: Identify,
{
    /// The node being inserted into the schema.
    pub node: T,
    /// The command to execute before inserting the node.
    ///
    /// If this command fails the whole transaction is aborted.
    pub before: B,
    /// The command to execute once the insertion has been performed.
    ///
    /// If this command fails the transaction IS NOT rollbacked. But the resulting error is still retrived as the transaction's result.
    pub after: A,
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
                    tracing::error!("posioned graph has been recovered");
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
            node: inserted_id,
        };

        self.after.execute(&payload)
    }
}

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

/// The context of the after-insertion triggers.
pub struct InsertedNode<'a, T>
where
    T: Identify,
{
    /// The schema in which the node has been inserted.
    pub schema: &'a Schema<T>,
    /// The id of the inserted node.
    pub node: T::Id,
}
