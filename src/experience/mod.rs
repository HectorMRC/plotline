use crate::{entity::EntityId, event::EventId};

/// An Experience is the result of a specific event in an entity.
pub struct Experience {
    entity: EntityId,
    event: EventId,
}
