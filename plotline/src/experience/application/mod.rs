//! The application service.

mod save;
pub use save::*;

mod filter;
pub use filter::*;

mod find;
pub use find::*;

use super::error::Result;
use crate::{experience::Experience, id::Id, interval::Interval, transaction::Tx};
use std::sync::Arc;

pub trait ExperienceRepository {
    type Interval: Interval;
    type Tx: Tx<Experience<Self::Interval>>;

    fn find(&self, id: Id<Experience<Self::Interval>>) -> Result<Self::Tx>;
    fn filter(&self, filter: &ExperienceFilter<Self::Interval>) -> Result<Vec<Self::Tx>>;
    fn create(&self, experience: &Experience<Self::Interval>) -> Result<()>;
    fn delete(&self, id: Id<Experience<Self::Interval>>) -> Result<()>;
}

pub struct ExperienceApplication<ExperienceRepo, EntityRepo, EventRepo> {
    pub experience_repo: Arc<ExperienceRepo>,
    pub entity_repo: Arc<EntityRepo>,
    pub event_repo: Arc<EventRepo>,
}
