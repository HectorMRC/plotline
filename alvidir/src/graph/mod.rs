//! A graph representation.

use std::collections::BTreeMap;

use crate::id::Identify;

mod edge;
pub use edge::*;

mod proxy;
pub use proxy::*;

/// An arbitrary node.
pub trait Node {
    /// The type to reference other nodes.
    type Edge;

    /// Returns all the edges of the node.
    fn edges(&self) -> Vec<Self::Edge>;
}

/// An entity which value depends on the state of a [`Graph`].
pub trait FromGraph<T>
where
    T: Identify,
{
    /// Returns an instance of self resulting from the given [`Graph`].
    fn from_graph(graph: &Graph<T>) -> Self;
}

impl<T, U> FromGraph<T> for U
where
    T: Identify,
    U: for<'a> From<&'a Graph<T>>,
{
    fn from_graph(graph: &Graph<T>) -> Self {
        Self::from(graph)
    }
}

/// An arbitrary graph.
#[derive(Debug)]
pub struct Graph<T: Identify> {
    /// All the nodes in the graph.
    nodes: BTreeMap<T::Id, T>,
}

impl<T: Identify> Default for Graph<T> {
    fn default() -> Self {
        Self {
            nodes: Default::default(),
        }
    }
}

impl<T> FromIterator<T> for Graph<T>
where
    T: Identify,
    T::Id: Ord + Clone,
{
    /// Returns the [`Graph`] resulting from all the nodes in the given iterator.
    ///
    /// This method does not check if there are repeated ids. In front of collisions
    /// only the latest node will remain.
    fn from_iter<V: IntoIterator<Item = T>>(nodes: V) -> Self {
        Self {
            nodes: BTreeMap::from_iter(nodes.into_iter().map(|node| (node.id().clone(), node))),
        }
    }
}

impl<T> Graph<T>
where
    T: Identify,
    T::Id: Ord + Clone,
{
    /// Inserts the given node into the graph, overwriting any previous value with the same id.
    pub fn with_node(mut self, node: T) -> Self {
        self.nodes.insert(node.id().clone(), node);
        self
    }
}

impl<T: Identify> Graph<T> {
    /// Returns the [`NodeProxy`] with the given id.
    pub fn node(&self, id: T::Id) -> NodeProxy<'_, T> {
        NodeProxy { graph: self, id }
    }
}

#[cfg(any(test, feature = "fixtures"))]
pub mod fixtures {
    use crate::id::Identify;

    use super::{edge::Edge, Node};

    /// A mock implementation for the [Node] trait.
    #[derive(Debug, Default)]
    pub struct NodeMock<Id: 'static> {
        pub id_fn: Option<fn() -> &'static Id>,
        pub edges_fn: Option<fn() -> Vec<Edge<Id>>>,
    }

    impl<Id> Identify for NodeMock<Id> {
        type Id = Id;

        fn id(&self) -> &Self::Id {
            self.id_fn.expect("id method must be set")()
        }
    }

    impl<Id> Node for NodeMock<Id> {
        type Edge = Edge<<Self as Identify>::Id>;

        fn edges(&self) -> Vec<Self::Edge> {
            self.edges_fn.expect("edges method must be set")()
        }
    }

    impl<Id> NodeMock<Id> {
        pub fn with_id_fn(mut self, f: fn() -> &'static Id) -> Self {
            self.id_fn = Some(f);
            self
        }

        pub fn with_edges_fn(mut self, f: fn() -> Vec<Edge<Id>>) -> Self {
            self.edges_fn = Some(f);
            self
        }
    }

    macro_rules! node_mock {
        ($id:tt, $($edges:tt)*) => {
            NodeMock::default()
                .with_id_fn(|| &$id)
                .with_edges_fn(|| vec![$($edges)*])
        };
    }

    pub(crate) use node_mock;
}
