mod save;
pub use save::*;

use super::{error::Result, Event};
use crate::{experience::Experience, id::Id, interval::Interval, transaction::Tx};
use std::sync::Arc;

pub trait ExperienceRepository {
    type Interval: Interval;
    type Tx: Tx<Experience<Self::Interval>>;

    fn create(&self, event: &Event<Self::Interval>) -> Result<()>;
    fn find(&self, id: Id<Event<Self::Interval>>) -> Result<Self::Tx>;
}

pub struct EventService<EventRepo> {
    pub event_repo: Arc<EventRepo>,
}
