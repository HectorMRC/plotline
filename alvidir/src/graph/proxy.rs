//! A proxy for nodes in a graph.

use crate::{deref::TryDeref, id::Identify, resource::Resource};

use super::Graph;

/// A preliminary representation of a node that may, or may not, exist in a [`Graph`].
#[derive(Debug)]
pub struct NodeProxy<'a, T: Identify> {
    /// The graph in which the id potentially exists.
    pub graph: &'a Graph<T>,
    /// The id of the node.
    pub id: T::Id,
}

impl<T> Clone for NodeProxy<'_, T>
where
    T: Identify,
    T::Id: Clone,
{
    fn clone(&self) -> Self {
        Self {
            graph: self.graph,
            id: self.id.clone(),
        }
    }
}

impl<T> Identify for NodeProxy<'_, T>
where
    T: Identify,
{
    type Id = T::Id;

    fn id(&self) -> &Self::Id {
        &self.id
    }
}

impl<T> TryDeref for NodeProxy<'_, T>
where
    T: Identify,
    T::Id: Ord,
{
    type Target = T;

    fn try_deref(&self) -> Option<&Self::Target> {
        self.graph.nodes.get(&self.id)
    }
}

impl<T> NodeProxy<'_, T>
where
    T: Identify,
    T::Id: Ord + Clone,
{
    /// Returns a list of all the nodes pointed by the current one.
    pub fn successors<Edge>(&self) -> Vec<Self>
    where
        Edge: Resource<T> + Identify<Id = T::Id>,
    {
        let Some(node) = self.try_deref() else {
            return Vec::default();
        };

        Edge::all(node)
            .into_iter()
            .map(|edge| Self {
                graph: self.graph,
                id: edge.id().clone(),
            })
            .collect()
    }
}

impl<T> NodeProxy<'_, T>
where
    T: Identify,
    T::Id: Ord,
{
    /// Returns true if, and only if, the node does not exist in the graph.
    pub fn is_virtual(&self) -> bool {
        !self.graph.nodes.contains_key(&self.id)
    }
}

#[cfg(test)]
mod tests {
    use crate::graph::{
        fixtures::{fake_node, FakeEdge, FakeNode},
        Graph,
    };

    #[test]
    fn non_existent_nodes_must_be_virtual() {
        let graph = Graph::<FakeNode<usize>>::default();
        let virtual_node = graph.node(0);

        assert!(
            virtual_node.is_virtual(),
            "a non-existent node in the graph must be virtual"
        );
    }

    #[test]
    fn existent_nodes_must_be_non_virtual() {
        let graph = Graph::default().with_node(fake_node!(0));

        let non_virtual_node = graph.node(0);
        assert!(
            !non_virtual_node.is_virtual(),
            "an existing node in the graph must not be virtual"
        );
    }

    #[test]
    fn graph_must_be_traversable() {
        let graph = Graph::from_iter(vec![fake_node!(1, 2), fake_node!(2, 1)]);

        let edges_1 = graph.node(1).successors::<FakeEdge<i8>>();
        assert_eq!(edges_1.len(), 1);
        assert_eq!(edges_1[0].id, 2);

        let edges_2 = edges_1[0].successors::<FakeEdge<i8>>();
        assert_eq!(edges_2.len(), 1);
        assert_eq!(edges_2[0].id, 1);
    }
}
