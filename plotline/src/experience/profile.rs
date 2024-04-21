use crate::{entity::Entity, id::Indentify};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A Profile describes an [Entity] during the time being between two periods
/// of time.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Profile {
    /// The entity being described by this profile.
    pub entity: Entity,
    /// The key-value attributes of the entity.
    pub values: HashMap<String, String>,
}

impl Indentify for Profile {
    type Id = <Entity as Indentify>::Id;

    fn id(&self) -> Self::Id {
        self.entity.id()
    }
}

impl Profile {
    pub fn new(entity: Entity) -> Self {
        Self {
            entity,
            values: HashMap::new(),
        }
    }

    pub fn values(&self) -> impl Iterator<Item = (&str, &str)> {
        self.values
            .iter()
            .map(|(key, value)| (key.as_str(), value.as_str()))
    }
}
