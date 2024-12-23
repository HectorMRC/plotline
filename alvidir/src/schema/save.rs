//! Save transaction.

use std::{cell::RefCell, marker::PhantomData};

use alvidir_macros::with_trigger;

use crate::{
    chain::LiFoChain,
    command::{Command, CommandRef, NoopCommand},
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
    pub node: RefCell<T>,
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
pub struct Save<T> {
    /// The node being saved into the schema.
    pub node: T,
}

impl<T, B, A, E, BArgs, AArgs> Command<Schema<T>, (BArgs, AArgs)> for Save<T, B, A>
where
    T: 'static + Identify,
    T::Id: Ord + Clone,
    B: for<'b> Command<NodeToSave<'b, T>, BArgs, Err = E>,
    A: for<'a> Command<SavedNode<'a, T>, AArgs, Err = E>,
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

        let payload = SavedNode {
            schema,
            node_id: inserted_id,
        };

        self.after.execute(&payload)
    }
}

/// A placeholder for types that are yet to be difined.
pub struct Unknown<T>(PhantomData<T>);

impl<T> Default for Save<Unknown<T>, NoopCommand, NoopCommand> {
    fn default() -> Self {
        Save::new(Unknown(PhantomData))
    }
}

impl<T, B, A> Save<Unknown<T>, B, A> {
    /// Converts self into a save transaction for the given node. 
    pub fn with_node(self, node: T) -> Save<T, B, A> {
        Save {
            node,
            before: self.before,
            after: self.after,
        }
    }
}

impl<T> Save<T, NoopCommand, NoopCommand> {
    /// Builds a save command for the given node.
    pub fn new(node: T) -> Self {
        Self {
            node,
            before: NoopCommand,
            after: NoopCommand,
        }
    }
}

impl<'a, T, Err> From<&'a Schema<T>> for Save<Unknown<T>, Vec<&'a dyn CommandRef<NodeToSave<'static, T>, (), Err = Err>>, Vec<&'a dyn CommandRef<SavedNode<'static, T>, (), Err = Err>>>
where 
    T: Identify,
    Err: 'static,
{
    fn from(schema: &'a Schema<T>) -> Self {
        let before: Vec<_> = schema.triggers::<NodeToSave<'_, T>, Err>().collect();
        let after: Vec<_> = schema.triggers::<SavedNode<'_, T>, Err>().collect();

        Self {
            node: Unknown(PhantomData),
            before,
            after
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{convert::Infallible, default};

    use crate::{graph::{fixtures::FakeNode, Graph}, id::{fixtures::IndentifyMock, Identify}, schema::Schema, command::{Command, CommandRef}};

    use super::{NodeToSave, Save};

    #[test]
    fn save_from_schema_should_be_command() {
        let schema = Schema::from(Graph::<IndentifyMock<usize>>::default());
        let r: Result<(), Infallible> = Save::from(&schema)
            .with_node(IndentifyMock::new(1))
            .execute();
    }
}