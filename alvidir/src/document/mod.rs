use syntactic_tree::SyntacticTreeNode;

use crate::{graph::Node, id::Identify, name::Name, property::Property, tag::Tag};

pub mod proxy;
pub mod syntactic_tree;

mod error;
pub use error::*;

/// A private alias for internal usage.
type DocumentId = <Document as Identify>::Id;

/// Represents a named syntactic tree.
pub struct Document {
    /// The name of the document.
    name: Name<Self>,
    /// The syntactic tree representing the document's content.
    root: SyntacticTreeNode,
}

impl Document {
    /// Returns a new document with the given name.
    pub fn new(name: Name<Self>) -> Self {
        Self {
            name,
            root: Default::default(),
        }
    }

    /// Sets the given [SyntacticTreeNode] as the root of the document.
    pub fn with_root(mut self, root: SyntacticTreeNode) -> Self {
        self.root = root;
        self
    }
}

impl Identify for Document {
    type Id = Name<Self>;

    fn id(&self) -> Self::Id {
        self.name.clone()
    }
}

impl Node for Document {
    type Edge = <Self as Identify>::Id;

    async fn tags(&self) -> Vec<Tag> {
        self.root.tags()
    }

    async fn properties(&self) -> Vec<Property<Self::Edge>> {
        self.root.properties()
    }

    async fn edges(&self) -> Vec<Self::Edge> {
        self.root.references()
    }
}
