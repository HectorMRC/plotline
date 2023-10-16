#[cfg(feature = "cli")]
pub mod cli;
pub mod error;
pub mod repository;
pub mod service;

/// Represents the unique name of an [Entity].
#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct EntityName(String);

impl TryFrom<String> for EntityName {
    type Error = error::Error;

    fn try_from(value: String) -> error::Result<Self> {
        if value.is_empty() {
            return Err(error::Error::NotAnEntityName);
        }

        Ok(Self(value))
    }
}

/// An Entity is anything which to interact with.
#[derive(Serialize, Deserialize, Hash, PartialEq, Eq, Clone)]
pub struct Entity {
    name: EntityName,
}

impl Entity {
    pub fn new(name: EntityName) -> Self {
        Self { name }
    }
}
