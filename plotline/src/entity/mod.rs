pub mod application;
#[cfg(feature = "in_memory")]
pub mod repository;

mod error;
pub use error::*;

use crate::id::{Id, Indentify};
use crate::name::Name;
use serde::{Deserialize, Serialize};

/// An Entity is anything which to interact with.
#[derive(Debug, Default, Clone, Eq, Serialize, Deserialize)]
pub struct Entity {
    /// The id of the entity.
    pub id: Id<Self>,
    /// The name of the entity.
    pub name: Name<Self>,
}

impl PartialEq for Entity {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Indentify for Entity {
    type Id = Id<Self>;

    fn id(&self) -> Self::Id {
        self.id
    }
}

impl Entity {
    /// Creates a new entity with the given id and name.
    pub fn new(id: Id<Self>, name: Name<Self>) -> Self {
        Self { id, name }
    }

    pub fn with_id(mut self, id: Id<Self>) -> Self {
        self.id = id;
        self
    }

    pub fn with_name(mut self, name: Name<Self>) -> Self {
        self.name = name;
        self
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use std::str::FromStr;
    use crate::name::Name;
    use super::Entity;

    impl Entity {
        pub fn fixture() -> Self {
            Entity {
                id: Default::default(),
                name: Name::from_str("fixture").unwrap(),
            }
        }
    }
}
