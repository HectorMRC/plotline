mod error;
use std::collections::HashMap;

pub use error::*;

use crate::{id::Identify, property::Property, tag::Tag};

#[trait_make::make]
pub trait DirectedGraphNode: Identify {
    /// Retrives all the tags in the node.
    async fn tags(&self) -> Vec<Tag>;
    /// Retrives all the properties of the node.
    async fn properties(&self) -> Vec<Property<Self::Id>>;
    /// Retrives all the references to other nodes.
    async fn references(&self) -> Vec<Self::Id>;
}

/// Represents an arbitrary directed graph.
struct DirectedGraph<Node>
where 
    Node: DirectedGraphNode
{
    /// All the nodes in the graph.
    nodes: HashMap<Node::Id, Node>,
}

impl<Node> DirectedGraph<Node>
where 
    Node: DirectedGraphNode,
{
    pub fn with_node(mut self, node: Node) -> Self {
        self.nodes.insert(node.id(), node);
        self
    }
}

#[cfg(any(test, features = "fixtures"))]
pub mod fixtures {
    use crate::id::Identify;

    use super::DirectedGraphNode;

    pub struct FakeDirectedGraphNode(usize);

    impl Identify for FakeDirectedGraphNode {
        type Id = usize;

        fn id(&self) -> Self::Id {
            self.0
        }
    }

    impl DirectedGraphNode for FakeDirectedGraphNode {
        async fn tags(&self) -> Vec<crate::tag::Tag> {
            todo!()
        }

        async fn properties(&self) -> Vec<crate::property::Property<Self::Id>> {
            todo!()
        }

        async fn references(&self) -> Vec<Self::Id> {
            todo!()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::graph::{fixtures::FakeDirectedGraphNode, Result};

    #[test]
    fn add_node() {
        struct Test {
            name: &'static str,
            node: FakeDirectedGraphNode,
            result: Result<()>,
        }
    }
}
