//! An edge representation for the graph.

use crate::name::Name;

/// A relation pointing to a node in a graph.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Edge<Node> {
    /// The name of the edge.
    pub name: Option<Name<Self>>,
    /// The node the edge is refering to.
    pub node: Node,
}

impl<Node> Edge<Node> {
    pub fn new(node: Node) -> Self {
        Self {
            name: Default::default(),
            node,
        }
    }

    pub fn with_name(mut self, name: Option<Name<Self>>) -> Self {
        self.name = name;
        self
    }
}
