use std::collections::HashMap;

use crate::{
    id::Identify,
    name::Name,
    property::{Property, PropertyValue},
    tag::Tag,
};

use super::Node;

/// Represents an arbitrary directed graph.
pub struct DirectedGraph<T: Identify> {
    /// All the nodes in the graph.
    nodes: HashMap<T::Id, T>,
}

impl<T: Identify> FromIterator<T> for DirectedGraph<T> {
    fn from_iter<U: IntoIterator<Item = T>>(nodes: U) -> Self {
        Self {
            nodes: HashMap::from_iter(nodes.into_iter().map(|node| (node.id(), node))),
        }
    }
}

impl<T: Identify> DirectedGraph<T> {
    /// Inserts the given node into the graph, overwriting any previous value with the same id.
    pub fn with_node(mut self, node: T) -> Self {
        self.nodes.insert(node.id(), node);
        self
    }
}

impl<T: Identify> DirectedGraph<T> {
    /// Returns an iterator over all the [DirectedNode]s in the graph.
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
pub struct DirectedNode<'a, T: Identify> {
    graph: &'a DirectedGraph<T>,
    id: T::Id,
}

impl<'a, T: Identify> Identify for DirectedNode<'a, T> {
    type Id = T::Id;

    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}

impl<'a, T> Node for DirectedNode<'a, T>
where
    T: Identify + Node<Edge = T::Id>,
{
    type Edge = Self;

    async fn tags(&self) -> Vec<Tag> {
        let Some(node) = self.value() else {
            return Vec::default();
        };

        node.tags().await
    }

    async fn properties(&self) -> Vec<Property<Self::Edge>> {
        let Some(node) = self.value() else {
            return Vec::default();
        };

        node.properties()
            .await
            .into_iter()
            .map(|property| Property::<Self> {
                name: Name::from(property.name),
                value: match property.value {
                    PropertyValue::Edge(id) => PropertyValue::Edge(self.graph.node(id)),
                    PropertyValue::String(s) => PropertyValue::String(s),
                },
            })
            .collect()
    }

    async fn edges(&self) -> Vec<Self::Edge> {
        let Some(node) = self.value() else {
            return Vec::default();
        };

        node.edges()
            .await
            .into_iter()
            .map(|id| self.graph.node(id))
            .collect()
    }
}

impl<'a, T: Identify> DirectedNode<'a, T> {
    /// Returns the content of the node if, and only if, the node is not virtual.
    pub fn value(&self) -> Option<&T> {
        self.graph.nodes.get(&self.id)
    }

    /// Returns true if, and only if, the node does not exists in the graph.
    pub fn is_virtual(&self) -> bool {
        self.value().is_none()
    }
}
