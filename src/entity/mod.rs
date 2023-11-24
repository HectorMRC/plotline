#[cfg(feature = "cli")]
pub mod cli;
#[cfg(feature = "fmt")]
pub mod fmt;
#[cfg(feature = "in_memory")]
pub mod repository;
pub mod application;

mod error;
pub use error::*;

use crate::id::{Id, Identifiable};
use crate::name::Name;
use serde::{Deserialize, Serialize};

/// An Entity is anything which to interact with.
#[derive(Clone, Serialize, Deserialize)]
pub struct Entity {
    id: Id<Self>,
    name: Name<Self>,
}

impl Eq for Entity {}
impl PartialEq for Entity {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Identifiable<Entity> for Entity {
    fn id(&self) -> Id<Self> {
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
