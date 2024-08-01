use crate::name::Name;

/// Represents an unidirectional relation between two nodes in a graph.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Edge<T> {
    /// The name of the edge.
    pub name: Option<Name<Self>>,
    /// The node the edge is refering to.
    pub node: T,
}

impl<T> Edge<T> {
    pub fn new(value: T) -> Self {
        Self {
            name: Default::default(),
            node: value,
        }
    }

    pub fn with_name(mut self, name: Option<Name<Self>>) -> Self {
        self.name = name;
        self
    }
}
