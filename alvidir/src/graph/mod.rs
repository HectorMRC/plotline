pub mod directed;
pub mod edge;

/// Represents an arbitrary node
#[trait_make::make]
pub trait Node {
    /// The type to reference other nodes.
    type Edge;

    /// Returns all the edges of the node.
    async fn edges(&self) -> Vec<Self::Edge>;
}

#[cfg(any(test, feature = "fixtures"))]
pub mod fixtures {
    use crate::id::Identify;

    use super::{edge::Edge, Node};

    /// A mock implementation for the [Node] trait.
    #[derive(Debug, Default)]
    pub struct NodeMock<Id> {
        pub id_fn: Option<fn() -> Id>,
        pub edges_fn: Option<fn() -> Vec<Edge<Id>>>,
    }

    impl<Id> Identify for NodeMock<Id> {
        type Id = Id;

        fn id(&self) -> Self::Id {
            if let Some(id_fn) = self.id_fn {
                return id_fn();
            }

            unimplemented!()
        }
    }

    impl<Id> Node for NodeMock<Id> {
        type Edge = Edge<<Self as Identify>::Id>;

        async fn edges(&self) -> Vec<Self::Edge> {
            self.edges_fn.expect("edges method must be set")()
        }
    }

    impl<Id> NodeMock<Id> {
        pub fn with_id_fn(mut self, f: fn() -> Id) -> Self {
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
                .with_id_fn(|| $id)
                .with_edges_fn(|| vec![$($edges)*])
        };
    }

    pub(crate) use node_mock;
}
