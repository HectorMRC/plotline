//! A proxy for nodes in a graph.

use std::sync::OnceLock;

use crate::{deref::TryDeref, id::Identify, property::Property};

/// Represents a source of nodes.
///
/// This trait allows [`NodeProxy`] to be graph-agnostic.
pub trait Source {
    type Node: Identify;

    /// Provides the node with the given id, if any.
    //
    // TODO: try to return Option<&Self::Node>.
    //       This method was previously returning a reference, but the transaction
    //       feature required this to become a value.
    fn get(&self, id: &<Self::Node as Identify>::Id) -> Option<Self::Node>;
    /// Returns true if, and only if, a node with the given id exist in the source.
    /// Otherwise returns false.
    fn contains(&self, id: &<Self::Node as Identify>::Id) -> bool;
}

/// A preliminary representation of a node that may, or may not, exist in a [`Graph`].
pub struct NodeProxy<'a, S>
where
    S: Source,
    S::Node: Identify,
{
    /// The source in which the id potentially exists.
    pub source: &'a S,
    /// The id of the node.
    pub id: <S::Node as Identify>::Id,
    /// The actual node.
    value: OnceLock<Option<S::Node>>,
}

impl<S> Clone for NodeProxy<'_, S>
where
    S: Source,
    S::Node: Identify + Clone,
    <S::Node as Identify>::Id: Clone,
{
    fn clone(&self) -> Self {
        Self {
            source: self.source,
            id: self.id.clone(),
            value: self.value.clone(),
        }
    }
}

impl<S> Identify for NodeProxy<'_, S>
where
    S: Source,
    S::Node: Identify,
{
    type Id = <S::Node as Identify>::Id;

    fn id(&self) -> &Self::Id {
        &self.id
    }
}

impl<S> TryDeref for NodeProxy<'_, S>
where
    S: Source,
    S::Node: Identify,
    <S::Node as Identify>::Id: Ord,
{
    type Target = S::Node;

    fn try_deref(&self) -> Option<&Self::Target> {
        self.value
            .get_or_init(|| self.source.get(&self.id))
            .as_ref()
    }
}

impl<S> NodeProxy<'_, S>
where
    S: Source,
    S::Node: Identify,
    <S::Node as Identify>::Id: Ord + Clone,
{
    /// Returns a list of all the nodes pointed by the current one.
    pub fn successors<Edge>(&self) -> Vec<Self>
    where
        Edge: Property<S::Node> + Identify<Id = <S::Node as Identify>::Id>,
    {
        let Some(node) = self.try_deref() else {
            return Vec::default();
        };

        Edge::all(node)
            .into_iter()
            .map(|edge| Self {
                source: self.source,
                id: edge.id().clone(),
                value: Default::default(),
            })
            .collect()
    }
}

impl<'a, S> NodeProxy<'a, S>
where
    S: Source,
    S::Node: Identify,
    <S::Node as Identify>::Id: Ord,
{
    pub fn new(source: &'a S, id: <S::Node as Identify>::Id) -> Self {
        NodeProxy {
            source,
            id,
            value: Default::default(),
        }
    }

    /// Returns true if, and only if, the node does not exist in the graph.
    pub fn is_virtual(&self) -> bool {
        !self.source.contains(&self.id)
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
