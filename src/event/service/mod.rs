mod create;
pub use create::*;

mod add_entity;
pub use add_entity::*;

use super::{error::Result, Event};
use crate::{guard::Tx, id::Id, interval::Interval};
use std::sync::Arc;

pub trait EventRepository {
    type Interval: Interval;
    type Tx: Tx<Event<Self::Interval>>;

    fn create(&self, event: &Event<Self::Interval>) -> Result<()>;
    fn find(&self, id: Id<Event<Self::Interval>>) -> Result<Self::Tx>;
}

pub struct EventService<R, E> {
    pub event_repo: Arc<R>,
    pub entity_repo: Arc<E>,
}
