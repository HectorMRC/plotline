mod error;
pub use error::*;

use crate::entity::EntityID;

/// An AttributeID uniquely identifies an attribute within a [Profile].
#[derive(PartialEq, Eq)]
pub struct AttributeID(String);

impl TryFrom<String> for AttributeID {
    type Error = error::Error;

    /// An AttributeID must consist of a non-empty string.
    fn try_from(value: String) -> error::Result<Self> {
        if value.is_empty() {
            return Err(Error::NotAnAttributeID);
        }

        Ok(Self(value))
    }
}

/// An AttributeValue is the information associated to an [AttributeID].
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
    id: AttributeID,
    value: AttributeValue,
}

impl Attribute {
    /// Creates an attribute with the given [AttributeID] and [AttributeValue].
    pub fn new(id: AttributeID, value: AttributeValue) -> Self {
        Self { id, value }
    }
}

/// A Profile collects all the necessary attributes to describe an individual entity.
pub struct Profile {
    entity: EntityID,
    attributes: Vec<Attribute>,
}

impl Profile {
    /// Creates an empty profiles for the given [EntityID].
    pub fn new(entity_id: EntityID) -> Self {
        Self {
            entity: entity_id,
            attributes: Default::default(),
        }
    }
}
