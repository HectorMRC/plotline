use std::vec::IntoIter;

use crate::name::Name;
use crate::property::Property;
use crate::tag::Tag;

use super::{Document, DocumentId, Error, Result};

/// Represents a set of [SyntacticTreeNode]s belonging to the same line in the [Document].
#[derive(Default)]
pub struct Line {
    /// The [SyntacticTreeNode]s contained in the line.
    nodes: Vec<SyntacticTreeNode>,
}

impl IntoIterator for Line {
    type Item = SyntacticTreeNode;
    type IntoIter = IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.nodes.into_iter()
    }
}

impl Line {
    fn with_node(mut self, node: SyntacticTreeNode) -> Result<Self> {
        if !node.is_terminal() {
            return Err(Error::NotALineNode);
        }

        self.nodes.push(node);
        Ok(self)
    }

    fn tags(&self) -> Vec<Tag> {
        self.nodes
            .iter()
            .map(SyntacticTreeNode::tags)
            .flatten()
            .collect()
    }

    fn properties(&self) -> Vec<Property<DocumentId>> {
        self.nodes
            .iter()
            .map(SyntacticTreeNode::properties)
            .flatten()
            .collect()
    }

    fn references(&self) -> Vec<Name<Document>> {
        self.nodes
            .iter()
            .map(SyntacticTreeNode::references)
            .flatten()
            .collect()
    }
}

/// Represents a set of [SyntacticTreeNode]s that shares the same scope in the [Document].
#[derive(Default)]
pub struct Section {
    /// The [Name] of the section, if any.
    name: Option<Name<Self>>,
    /// The [SyntacticTreeNode]s contained in the section.
    nodes: Vec<SyntacticTreeNode>,
}

impl Section {
    fn with_name(mut self, name: Name<Self>) -> Self {
        self.name = Some(name);
        self
    }

    fn with_node(mut self, node: SyntacticTreeNode) -> Self {
        self.nodes.push(node);
        self
    }

    fn tags(&self) -> Vec<Tag> {
        self.nodes
            .iter()
            .map(SyntacticTreeNode::tags)
            .flatten()
            .collect()
    }

    fn properties(&self) -> Vec<Property<DocumentId>> {
        self.nodes
            .iter()
            .map(SyntacticTreeNode::properties)
            .flatten()
            .collect()
    }

    fn references(&self) -> Vec<Name<Document>> {
        self.nodes
            .iter()
            .map(SyntacticTreeNode::references)
            .flatten()
            .collect()
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
    Property(Property<DocumentId>),
    Reference(DocumentId),
    // The section node ensures the syntactic tree is standalone, which means an arbitrary tree can
    // be built without other definitions but its own.
    Section(Section),
    String(String),
    Tag(Tag),
}

impl Default for SyntacticTreeNode {
    /// The default [SyntacticTreeNode] is an empty [Section].
    fn default() -> Self {
        Self::Section(Default::default())
    }
}

impl SyntacticTreeNode {
    /// Returns true if, and only if, the node does not support children nodes, becoming a leaf of
    /// the syntactic tree.
    fn is_terminal(&self) -> bool {
        // using a match forces the developer to always determine if an arm is terminal or not when
        // adding it
        match self {
            Self::Property(_) | Self::Reference(_) | Self::String(_) | Self::Tag(_) => true,
            Self::Section(_) | Self::Line(_) => false,
        }
    }

    pub(super) fn tags(&self) -> Vec<Tag> {
        match self {
            Self::Line(internal_node) => internal_node.tags(),
            Self::Section(internal_node) => internal_node.tags(),
            Self::Tag(tag) => vec![tag.clone()],
            _ => vec![],
        }
    }

    pub(super) fn properties(&self) -> Vec<Property<DocumentId>> {
        match self {
            Self::Line(internal_node) => internal_node.properties(),
            Self::Section(internal_node) => internal_node.properties(),
            Self::Property(property) => vec![property.clone()],
            _ => vec![],
        }
    }

    pub(super) fn references(&self) -> Vec<DocumentId> {
        match self {
            Self::Line(internal_node) => internal_node.references(),
            Self::Section(internal_node) => internal_node.references(),
            Self::Reference(name) => vec![name.clone()],
            _ => vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::document::{
        syntactic_tree::{Line, Section, SyntacticTreeNode},
        Error, Result,
    };

    #[test]
    fn line_with_node() {
        struct Test {
            name: &'static str,
            node: SyntacticTreeNode,
            result: Result<()>,
        }

        vec![
            Test {
                name: "non terminal node (section) should fail",
                node: SyntacticTreeNode::Section(Section::default()),
                result: Err(Error::NotALineNode),
            },
            Test {
                name: "non terminal node (line) should fail",
                node: SyntacticTreeNode::Line(Line::default()),
                result: Err(Error::NotALineNode),
            },
            Test {
                name: "terminal node should not fail",
                node: SyntacticTreeNode::String(String::default()),
                result: Ok(()),
            },
        ]
        .into_iter()
        .for_each(|test| {
            assert_eq!(
                Line::default().with_node(test.node).map(|_| ()),
                test.result,
                "{}",
                test.name
            );
        })
    }
}
