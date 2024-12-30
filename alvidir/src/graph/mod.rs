//! Graph related definitions.

use std::collections::{btree_map::Values, BTreeMap};

use crate::id::Identify;

mod proxy;
pub use proxy::*;

/// An arbitrary graph.
#[derive(Debug)]
pub struct Graph<T>
where
    T: Identify,
{
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

impl<'a, T> IntoIterator for &'a Graph<T>
where
    T: Identify,
{
    type Item = &'a T;

    type IntoIter = Values<'a, T::Id, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.nodes.values()
    }
}

impl<T> Source for Graph<T>
where
    T: Identify + Clone,
    T::Id: Ord,
{
    type Node = T;

    fn get(&self, id: &<Self::Node as Identify>::Id) -> Option<Self::Node> {
        self.nodes.get(id).cloned()
    }

    fn contains(&self, id: &<Self::Node as Identify>::Id) -> bool {
        self.nodes.contains_key(id)
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

    /// Inserts the given node into the graph, returning the previous node with that same id, if any.
    pub fn insert(&mut self, node: T) -> Option<T> {
        self.nodes.insert(node.id().clone(), node)
    }
}

impl<T> Graph<T>
where
    T: Identify + Clone,
    T::Id: Ord,
{
    /// Returns the [`NodeProxy`] for the given id.
    pub fn node(&self, id: T::Id) -> NodeProxy<'_, Self> {
        NodeProxy::new(self, id)
    }
}

impl<T> Graph<T>
where
    T: Identify,
    T::Id: Ord,
{
    /// Removes the node with the given id from the graph, returning it, if any.
    pub fn remove(&mut self, node_id: &T::Id) -> Option<T> {
        self.nodes.remove(node_id)
    }
}

#[cfg(any(test, feature = "fixtures"))]
pub mod fixtures {
    use crate::{id::Identify, property::Property};

    /// A fake node type.
    #[derive(Debug, Default, Clone)]
    pub struct FakeNode<Id> {
        pub id: Option<Id>,
        pub edges: Option<Vec<Id>>,
    }

    impl<Id> Identify for FakeNode<Id> {
        type Id = Id;

        fn id(&self) -> &Self::Id {
            self.id.as_ref().expect("id should be set")
        }
    }

    impl<Id> FakeNode<Id> {
        pub fn with_id(mut self, id: Id) -> Self {
            self.id = Some(id);
            self
        }

        pub fn with_edges(mut self, edges: Vec<Id>) -> Self {
            self.edges = Some(edges);
            self
        }
    }

    /// A fake edge type.
    pub struct FakeEdge<T> {
        pub id: T,
    }

    impl<T> From<T> for FakeEdge<T> {
        fn from(id: T) -> Self {
            FakeEdge { id }
        }
    }

    impl<T> Property<FakeNode<T>> for FakeEdge<T>
    where
        T: Copy,
    {
        fn all(source: &FakeNode<T>) -> Vec<Self>
        where
            Self: Sized,
        {
            source
                .edges
                .as_ref()
                .expect("edges method should be set")
                .iter()
                .cloned()
                .map(FakeEdge::from)
                .collect()
        }
    }

    impl<T> Identify for FakeEdge<T> {
        type Id = T;

        fn id(&self) -> &Self::Id {
            &self.id
        }
    }

    #[allow(unused_macros)]
    macro_rules! fake_node {
        ($id:tt $(,$edges:tt)*) => {
            FakeNode::default()
                .with_id($id)
                .with_edges(vec![$($edges)*])
        };
    }

    #[allow(unused_imports)]
    pub(crate) use fake_node;
}
