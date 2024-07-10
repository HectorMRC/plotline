use crate::{property::Property, tag::Tag};

pub mod application;
pub mod directed;

mod error;
pub use error::*;

/// Represents an arbitrary node
#[trait_make::make]
pub trait Node {
    /// The type to reference other nodes.
    type Edge;

    /// Returns all tags of the node.
    async fn tags(&self) -> Vec<Tag>;
    /// Returns all properties of the node.
    async fn properties(&self) -> Vec<Property<Self::Edge>>;
    /// Returns all edges of the nodes.
    async fn edges(&self) -> Vec<Self::Edge>;
}

#[cfg(any(test, features = "fixtures"))]
pub mod fixtures {
    use std::str::FromStr;

    use crate::{
        id::Identify,
        name::Name,
        property::{Property, PropertyValue},
        tag::Tag,
    };

    use super::Node;

    pub struct FakeNode(pub String);

    impl Identify for FakeNode {
        type Id = String;

        fn id(&self) -> Self::Id {
            self.0.clone()
        }
    }

    impl Node for FakeNode {
        type Edge = <Self as Identify>::Id;

        async fn tags(&self) -> Vec<Tag> {
            vec![Tag::from_str(&self.0).unwrap()]
        }

        async fn properties(&self) -> Vec<Property<Self::Edge>> {
            vec![Property {
                name: Name::from_str(&self.0).unwrap(),
                value: PropertyValue::String(self.0.clone()),
            }]
        }

        async fn edges(&self) -> Vec<Self::Edge> {
            vec![self.0.clone()]
        }
    }
}
