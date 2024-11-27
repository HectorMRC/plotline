//! Insert transaction.

use std::cell::RefCell;

use alvidir_macros::with_trigger;

use crate::{
    chain::LiFoChain,
    command::{Command, NoopCommand},
    graph::Graph,
    id::Identify,
};

use super::{trigger::WithTrigger, Schema};

/// The context for triggers before insertion.
pub struct NodeToInsert<'a, T>
where
    T: Identify,
{
    /// The graph the node is being inserted to.
    pub graph: &'a Graph<T>,
    /// The node being inserted into the schema.
    pub node: RefCell<T>,
}

/// The context for triggers after insertion.
pub struct InsertedNode<'a, T>
where
    T: Identify,
{
    /// The schema the node has beem inserted to.
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

impl<T, B, A, E, BArgs, AArgs> Command<Schema<T>, (BArgs, AArgs)> for Insert<T, B, A>
where
    T: 'static + Identify,
    T::Id: Ord + Clone,
    B: for<'b> Command<NodeToInsert<'b, T>, BArgs, Err = E>,
    A: for<'a> Command<InsertedNode<'a, T>, AArgs, Err = E>,
{
    type Err = E;

    /// Executes the [`Insert`] transaction.
    ///
    /// ### Before
    /// Before performing the insertion this transaction executes the before command.
    /// If the before command fails, the whole transaction is aborted and the trigger's error is returned as the transaction's result.
    ///
    /// ### After
    /// Once the insertion has been completed, this transaction executes the after command.
    /// If the after command fails the transaction __DOES NOT__ rollback, but the resulting error is retrived as the transaction's result.
    fn execute(self, schema: &Schema<T>) -> Result<(), Self::Err> {
        let inserted_id = {
            let mut graph = schema.write();

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

// impl<T> Schema<T>
// where
//     T: Identify
// {
//     pub fn insert<B, A>(&self, node: T) -> Insert<T, B, A> {
//         let insert = Insert::new(node);
//         self.triggers.iter().filter_map(|trigger| trigger.downcast_ref::<>())
//         insert
//     }
// }
