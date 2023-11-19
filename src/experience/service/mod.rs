mod save;
pub use save::*;

use super::error::Result;
use crate::{experience::Experience, interval::Interval, transaction::Tx, entity::Entity, event::Event, id::Id};
use std::sync::Arc;

pub trait ExperienceRepository {
    type Interval: Interval;
    type Tx: Tx<Experience<Self::Interval>>;

    fn create(&self, experience: &Experience<Self::Interval>) -> Result<()>;
    fn find_by_entity_and_event(&self, entity: Id<Entity>, event: Id<Event<Self::Interval>>) -> Result<Self::Tx>;
}

pub struct EventService<EventRepo> {
    pub event_repo: Arc<EventRepo>,
}
