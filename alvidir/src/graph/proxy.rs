//! A proxy for nodes in a graph.

use crate::{id::Identify, name::Name};

use super::{edge::Edge, Graph, Node};

/// An access control layer for a node that may, or may not, exist in a [`Graph`].
#[derive(Debug)]
pub struct NodeProxy<'a, T: Identify> {
    /// The graph in which the id potentially exists.
    pub graph: &'a Graph<T>,
    /// The id of the node.
    pub id: T::Id,
}

impl<'a, T> Clone for NodeProxy<'a, T>
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

impl<'a, T> Identify for NodeProxy<'a, T>
where
    T: Identify,
{
    type Id = T::Id;

    fn id(&self) -> &Self::Id {
        &self.id
    }
}

impl<'a, T> Node for NodeProxy<'a, T>
where
    T: Identify + Node<Edge = Edge<T::Id>>,
    T::Id: Ord,
{
    type Edge = Edge<Self>;

    fn edges(&self) -> Vec<Self::Edge> {
        let Some(node) = self.value() else {
            return Vec::default();
        };

        node.edges()
            .into_iter()
            .map(|edge| Edge::new(self.graph.node(edge.node)).with_name(edge.name.map(Name::from)))
            .collect()
    }
}

impl<'a, T> NodeProxy<'a, T>
where
    T: Identify,
    T::Id: Ord,
{
    /// Returns the content of the node if, and only if, the node is not virtual.
    pub fn value(&self) -> Option<&T> {
        self.graph.nodes.get(&self.id)
    }

    /// Returns true if, and only if, the node does not exist in the graph.
    pub fn is_virtual(&self) -> bool {
        !self.graph.nodes.contains_key(&self.id)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::{
        graph::{
            edge::Edge,
            fixtures::{node_mock, NodeMock},
            Graph, Node,
        },
        name::Name,
    };

    #[tokio::test]
    async fn only_non_existent_nodes_must_be_virtual() {
        let graph = Graph::default().with_node(node_mock!(
            "node_1",
            Edge::new("node_1"),
            Edge::new("node_2")
        ));

        let node_1 = graph.node("node_1");
        assert!(
            !node_1.is_virtual(),
            "an existing node in the graph must not be virtual"
        );

        let edges_1 = node_1.edges();
        assert_eq!(edges_1.len(), 2);
        assert!(
            !edges_1[0].node.is_virtual(),
            "an existing refered node in the graph must not be virtual"
        );
        assert!(
            edges_1[1].node.is_virtual(),
            "a non-existent refered node in the graph must be virtual"
        );

        assert!(
            graph.node("node_3").is_virtual(),
            "a non-existent node in the graph must be virtual"
        )
    }

    #[tokio::test]
    async fn graph_must_be_traversable() {
        let graph = Graph::from_iter(vec![
            node_mock!("node_1", Edge::new("node_2")),
            node_mock!("node_2", Edge::new("node_3")),
            node_mock!(
                "node_3",
                Edge::new("node_1").with_name(Some(Name::from_str("next").unwrap())),
                Edge::new("node_2").with_name(Some(Name::from_str("previous").unwrap()))
            ),
        ]);

        let edges_1 = graph.node("node_1").edges();
        assert_eq!(edges_1.len(), 1);
        assert!(edges_1[0].name.is_none());
        assert_eq!(edges_1[0].node.id, "node_2");

        let edges_2 = edges_1[0].node.edges();
        assert_eq!(edges_2.len(), 1);
        assert_eq!(edges_2[0].node.id, "node_3");

        let edges_3 = edges_2[0].node.edges();
        assert_eq!(edges_3.len(), 2);
        assert_eq!(edges_3[0].name.as_ref().unwrap().as_str(), "next");
        assert_eq!(edges_3[0].node.id, "node_1");
        assert_eq!(edges_3[1].name.as_ref().unwrap().as_str(), "previous");
        assert_eq!(edges_3[1].node.id, "node_2");
    }
}
