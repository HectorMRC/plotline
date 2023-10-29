use crate::{entity::EntityID, event::EventID};

/// An Experience is the result of a specific event in an entity.
pub struct Experience {
    entity: EntityID,
    event: EventID,
}
