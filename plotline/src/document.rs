use std::vec::IntoIter;

use crate::name::Name;
use crate::property::Property;
use crate::tag::Tag;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("a line cannot contain non-terminal nodes")]
    NotALineNode,
}

/// A Line represents a set of [SyntacticTreeNode]s belonging to the same line
/// in the [Document].
#[derive(Default)]
pub struct Line {
    /// The [SyntacticTreeNode]s contained in the line.
    pub nodes: Vec<SyntacticTreeNode>,
}

impl Line {
    pub fn with_node(mut self, node: SyntacticTreeNode) -> Result<Self> {
        if !node.is_terminal() {
            return Err(Error::NotALineNode);
        }

        self.nodes.push(node);
        Ok(self)
    }
}

impl IntoIterator for Line {
    type Item = SyntacticTreeNode;
    type IntoIter = IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.nodes.into_iter()
    }
}

/// A Section represents a set of [SyntacticTreeNode]s that shares the same
/// scope in the [Document].
#[derive(Default)]
pub struct Section {
    /// The [Name] of the section, if any.
    pub name: Option<Name<Self>>,
    /// The [SyntacticTreeNode]s contained in the section.
    pub nodes: Vec<SyntacticTreeNode>,
}

impl Section {
    pub fn with_name(mut self, name: Name<Self>) -> Self {
        self.name = Some(name);
        self
    }

    pub fn with_node(mut self, node: SyntacticTreeNode) -> Self {
        self.nodes.push(node);
        self
    }
}

impl IntoIterator for Section {
    type Item = SyntacticTreeNode;
    type IntoIter = IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.nodes.into_iter()
    }
}

/// Represents a node in a syntactic tree.
pub enum SyntacticTreeNode {
    Line(Line),
    Property(Property),
    Reference(Name<Document>),
    Section(Section),
    String(String),
    Tag(Tag),
}

impl SyntacticTreeNode {
    /// Returns true if, and only if, the node does not support children nodes,
    /// becoming a leaf of the syntactic tree.
    pub fn is_terminal(&self) -> bool {
        match self {
            Self::Property(_) | Self::Reference(_) | Self::String(_) | Self::Tag(_) => true,
            Self::Section(_) | Self::Line(_) => false,
        }
    }
}

/// A Document is the minimum form of information.
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
