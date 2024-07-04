mod syntactic_tree;
pub use syntactic_tree::*;

use crate::{graph::DirectedGraphNode, id::Identify, name::Name, property::Property, tag::Tag};

// A private alias for internal usage.
type DocumentId = Name<Document>;

/// Represents a named syntactic tree.
pub struct Document {
    /// The name of the document.
    pub name: Name<Self>,
    /// The syntactic tree representing the document's content.
    pub root: Option<SyntacticTreeNode>,
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
        self.root = Some(root);
        self
    }
}

impl Identify for Document {
    type Id = Name<Self>;

    fn id(&self) -> &Self::Id {
        &self.name
    }
}

impl DirectedGraphNode for Document {
    fn tags(&self) -> Vec<Tag> {
        self.root
            .as_ref()
            .map(SyntacticTreeNode::tags)
            .unwrap_or_default()
    }

    fn properties(&self) -> Vec<Property<Self::Id>> {
        self.root
            .as_ref()
            .map(SyntacticTreeNode::properties)
            .unwrap_or_default()
    }

    fn references(&self) -> Vec<Self::Id> {
        self.root
            .as_ref()
            .map(SyntacticTreeNode::references)
            .unwrap_or_default()
    }
}
