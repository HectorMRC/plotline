mod error;
pub use error::*;

use crate::{property::Property, tag::Tag};

pub mod application;
pub mod directed;

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

    pub struct FakeNode(usize);

    impl Identify for FakeNode {
        type Id = usize;

        fn id(&self) -> Self::Id {
            self.0
        }
    }

    impl Node for FakeNode {
        type Edge = <Self as Identify>::Id;

        async fn tags(&self) -> Vec<Tag> {
            vec![Tag::from_str("fake").unwrap()]
        }

        async fn properties(&self) -> Vec<Property<Self::Edge>> {
            vec![Property {
                name: Name::from_str("fake").unwrap(),
                value: PropertyValue::String("fake".to_string()),
            }]
        }

        async fn edges(&self) -> Vec<Self::Edge> {
            vec![0]
        }
    }
}
