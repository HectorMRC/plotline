mod error;
pub use error::*;

use crate::{entity::EntityId, id::Id, name::Name};

/// AttributeName determines an instance of [Name] belongs to an [Attribute].
pub struct AttributeName;

/// An AttributeValue is the information associated to an [AttributeName].
pub struct AttributeValue(String);

impl TryFrom<String> for AttributeValue {
    type Error = Error;

    fn try_from(value: String) -> Result<Self> {
        if value.is_empty() {
            return Err(Error::NotAnAttributeValue);
        }

        Ok(Self(value))
    }
}

/// An Attribute gives some information about an entity.
pub struct Attribute {
    name: Name<AttributeName>,
    value: AttributeValue,
}

impl Attribute {
    /// Creates an attribute with the given [AttributeName] and [AttributeValue].
    pub fn new(name: Name<AttributeName>, value: AttributeValue) -> Self {
        Self { name, value }
    }
}

/// A Profile collects all the necessary attributes to describe an individual entity.
pub struct Profile {
    entity: Id<EntityId>,
    attributes: Vec<Attribute>,
}

impl Profile {
    /// Creates an empty profiles for the given [EntityId].
    pub fn new(entity_id: Id<EntityId>) -> Self {
        Self {
            entity: entity_id,
            attributes: Default::default(),
        }
    }
}
