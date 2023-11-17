use crate::{entity::Entity, id::Id};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A Profile describes an [Entity] during the time being between two periods of time.
#[derive(Clone, Serialize, Deserialize)]
pub struct Profile {
    id: Id<Self>,
    entity: Id<Entity>,
    values: HashMap<String, String>,
}
