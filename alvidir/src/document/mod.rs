use syntactic_tree::SyntacticTreeNode;

use crate::{graph::Node, id::Identify, name::Name};

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

impl Identify for Document {
    type Id = Name<Self>;

    fn id(&self) -> Self::Id {
        self.name.clone()
    }
}

impl Node for Document {
    type Edge = <Self as Identify>::Id;

    async fn edges(&self) -> Vec<Self::Edge> {
        self.root.references()
    }
}

impl From<Name<Self>> for Document {
    fn from(name: Name<Self>) -> Self {
        Self::new(name)
    }
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
