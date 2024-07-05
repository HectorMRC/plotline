use crate::name::Name;

#[derive(Debug, Clone)]
pub enum PropertyValue<Ref> {
    String(String),
    Reference(Ref),
}

/// Represents an arbitrary value associated to a name.
#[derive(Debug, Clone)]
pub struct Property<Ref> {
    /// The [Name] of the property.
    pub name: Name<Self>,
    /// The value of the property.
    pub value: PropertyValue<Ref>,
}
