mod save;
pub use save::*;

mod add_entity;
pub use add_entity::*;

use super::{error::Result, Event};
use crate::{transaction::Tx, id::Id, interval::Interval};
use std::sync::Arc;

pub trait EventRepository {
    type Interval: Interval;
    type Tx: Tx<Event<Self::Interval>>;

    fn create(&self, event: &Event<Self::Interval>) -> Result<()>;
    fn find(&self, id: Id<Event<Self::Interval>>) -> Result<Self::Tx>;
}

pub struct EventService<EventRepo, EntityRepo> {
    pub event_repo: Arc<EventRepo>,
    pub entity_repo: Arc<EntityRepo>,
}
