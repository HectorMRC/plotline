use crate::name::Name;

#[derive(Debug, Clone)]
pub enum PropertyValue<Edge> {
    String(String),
    Edge(Edge),
}

/// Represents an arbitrary value associated to a name.
#[derive(Debug, Clone)]
pub struct Property<Edge> {
    /// The [Name] of the property.
    pub name: Name<Self>,
    /// The value of the property.
    pub value: PropertyValue<Edge>,
}
