pub mod directed;
pub mod edge;

/// Represents an arbitrary node
#[trait_make::make]
pub trait Node {
    /// The type to reference other nodes.
    type Edge;

    /// Returns all the edges of the nodes.
    async fn edges(&self) -> Vec<Self::Edge>;
}

#[cfg(any(test, feature = "fixtures"))]
pub mod fixtures {
    use crate::id::Identify;

    use super::{edge::Edge, Node};

    #[derive(Debug)]
    pub struct FakeNode {
        pub id: <Self as Identify>::Id,
        pub edges: Vec<Edge<<Self as Identify>::Id>>,
    }

    impl Identify for FakeNode {
        type Id = &'static str;

        fn id(&self) -> Self::Id {
            self.id
        }
    }

    impl Node for FakeNode {
        type Edge = Edge<<Self as Identify>::Id>;

        async fn edges(&self) -> Vec<Self::Edge> {
            self.edges.clone()
        }
    }
}
