use std::{collections::HashMap, hash::Hash};

use crate::{id::Identify, name::Name};

use super::{edge::Edge, Node};

/// Represents an arbitrary directed graph.
#[derive(Debug)]
pub struct DirectedGraph<T: Identify> {
    /// All the nodes in the graph.
    nodes: HashMap<T::Id, T>,
}

impl<T: Identify> Default for DirectedGraph<T> {
    fn default() -> Self {
        Self {
            nodes: Default::default(),
        }
    }
}

impl<T> FromIterator<T> for DirectedGraph<T>
where 
    T: Identify,
    T::Id: Eq + Hash,
{
    /// Returns a [DirectedGraph] resulting from all the nodes in the given iterator.
    ///
    /// Notice how this method does not check if there are repeated ids. In the case of
    /// collisions only the last node with the same id will remain.
    fn from_iter<U: IntoIterator<Item = T>>(nodes: U) -> Self {
        Self {
            nodes: HashMap::from_iter(nodes.into_iter().map(|node| (node.id(), node))),
        }
    }
}

impl<T> DirectedGraph<T> 
where 
    T: Identify,
    T::Id: Eq + Hash,
{
    /// Inserts the given node into the graph, overwriting any previous value with the same id.
    pub fn with_node(mut self, node: T) -> Self {
        self.nodes.insert(node.id(), node);
        self
    }
}

impl<T> DirectedGraph<T>
where
    T: Identify,
    T::Id: Clone,
{
    /// Returns an iterator over all the existing [DirectedNode]s in the graph.
    pub fn nodes<'a>(&'a self) -> impl Iterator<Item = DirectedNode<'a, T>> {
        self.nodes.keys().cloned().map(|id| self.node(id))
    }
}

impl<T: Identify> DirectedGraph<T> {
    /// Returns a [DirectedNode] with the given id associated to the graph.
    ///
    /// Notice how this method does not ensures the given id does exists in the graph. If it does
    /// not, the returned node is virtual.
    pub fn node<'a>(&'a self, id: T::Id) -> DirectedNode<'a, T> {
        DirectedNode { graph: self, id }
    }
}

/// Represents a node in a [DirectedGraph].
#[derive(Debug)]
pub struct DirectedNode<'a, T: Identify> {
    graph: &'a DirectedGraph<T>,
    id: T::Id,
}

impl<'a, T> Identify for DirectedNode<'a, T>
where
    T: Identify,
    T::Id: Clone,
{
    type Id = T::Id;

    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}

impl<'a, T> Node for DirectedNode<'a, T>
where
    T: Identify + Node<Edge = Edge<T::Id>>,
    T::Id: Eq + Hash,
{
    type Edge = Edge<Self>;

    async fn edges(&self) -> Vec<Self::Edge> {
        let Some(node) = self.value() else {
            return Vec::default();
        };

        node.edges()
            .await
            .into_iter()
            .map(|edge| Edge::new(self.graph.node(edge.node)).with_name(edge.name.map(Name::from)))
            .collect()
    }
}

impl<'a, T> DirectedNode<'a, T> 
where 
    T: Identify,
    T::Id: Eq + Hash,
{
    /// Returns the content of the node if, and only if, the node is not virtual.
    pub fn value(&self) -> Option<&T> {
        self.graph.nodes.get(&self.id)
    }

    /// Returns true if, and only if, the node does not exists in the graph.
    pub fn is_virtual(&self) -> bool {
        !self.graph.nodes.contains_key(&self.id)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::{
        graph::{
            directed::DirectedGraph,
            edge::Edge,
            fixtures::{node_mock, NodeMock},
            Node,
        },
        name::Name,
    };

    #[tokio::test]
    async fn only_non_existent_nodes_must_be_virtual() {
        let graph = DirectedGraph::default().with_node(node_mock!(
            "node_1",
            Edge::new("node_1"),
            Edge::new("node_2")
        ));

        let node_1 = graph.node("node_1");
        assert!(
            !node_1.is_virtual(),
            "an existing node in the graph must not be virtual"
        );

        let edges_1 = node_1.edges().await;
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
    async fn directed_graph_must_be_traversable() {
        let graph = DirectedGraph::from_iter(vec![
            node_mock!("node_1", Edge::new("node_2")),
            node_mock!("node_2", Edge::new("node_3")),
            node_mock!(
                "node_3",
                Edge::new("node_1").with_name(Some(Name::from_str("next").unwrap())),
                Edge::new("node_2").with_name(Some(Name::from_str("previous").unwrap()))
            ),
        ]);

        let edges_1 = graph.node("node_1").edges().await;
        assert_eq!(edges_1.len(), 1);
        assert!(edges_1[0].name.is_none());
        assert_eq!(edges_1[0].node.id, "node_2");

        let edges_2 = edges_1[0].node.edges().await;
        assert_eq!(edges_2.len(), 1);
        assert_eq!(edges_2[0].node.id, "node_3");

        let edges_3 = edges_2[0].node.edges().await;
        assert_eq!(edges_3.len(), 2);
        assert_eq!(edges_3[0].name.clone().unwrap(), "next");
        assert_eq!(edges_3[0].node.id, "node_1");
        assert_eq!(edges_3[1].name.clone().unwrap(), "previous");
        assert_eq!(edges_3[1].node.id, "node_2");
    }
}
