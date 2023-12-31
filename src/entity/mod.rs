pub mod application;
#[cfg(feature = "cli")]
pub mod cli;
#[cfg(feature = "fmt")]
pub mod fmt;
#[cfg(feature = "in_memory")]
pub mod repository;

mod error;
pub use error::*;

use crate::id::{Id, Identifiable};
use crate::name::Name;
use serde::{Deserialize, Serialize};

/// An Entity is anything which to interact with.
#[derive(Debug, Clone, Eq, Serialize, Deserialize)]
pub struct Entity {
    id: Id<Self>,
    name: Name<Self>,
}

impl PartialEq for Entity {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Identifiable for Entity {
    type Id = Id<Entity>;

    fn id(&self) -> Self::Id {
        self.id
    }
}

impl Entity {
    /// Creates a new entity with the given id and name.
    pub fn new(id: Id<Self>, name: Name<Self>) -> Self {
        Self { id, name }
    }

    /// Returns a reference to the name of self.
    pub fn name(&self) -> &Name<Self> {
        &self.name
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::Entity;
    use crate::id::Id;

    impl Entity {
        pub fn fixture() -> Self {
            Entity {
                id: Default::default(),
                name: "fixture".to_string().try_into().unwrap(),
            }
        }

        pub fn with_id(mut self, id: Id<Entity>) -> Self {
            self.id = id;
            self
        }
    }
}
