use std::sync::{Arc, Weak};

use crate::{id::Identify, property::Property, tag::Tag};

pub trait DirectedGraphNode: Identify {
    /// Retrives all the tags in the node.
    fn tags(&self) -> Vec<Tag>;
    /// Retrives all the properties of the node.
    fn properties(&self) -> Vec<Property<Self::Id>>;
    /// Retrives all the references to other nodes.
    fn references(&self) -> Vec<Self::Id>;
}

/// Represents a subset of a [DirectedGraph].
trait DirectedGraphView {
    type Node;

    fn new_node(&self, node: Weak<Self::Node>);
}

struct DirectedGraph<Node> {
    /// The nodes in the graph.
    nodes: Vec<Arc<Node>>,
    /// The views associated to the graph.
    views: Vec<()>,
}
