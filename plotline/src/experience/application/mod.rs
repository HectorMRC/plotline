//! The application service.

mod save;
pub use save::*;

mod filter;
pub use filter::*;

mod find;
pub use find::*;

use super::error::Result;
use crate::{
    command::Command, experience::Experience, id::Id, interval::Interval, transaction::Tx,
};
use std::sync::Arc;

pub trait ExperienceRepository {
    type Intv: Interval;
    type Tx: Tx<Experience<Self::Intv>>;

    fn find(&self, id: Id<Experience<Self::Intv>>) -> Result<Self::Tx>;
    fn filter(&self, filter: &ExperienceFilter<Self::Intv>) -> Result<Vec<Self::Tx>>;
    fn create(&self, experience: &Experience<Self::Intv>) -> Result<()>;
    fn delete(&self, id: Id<Experience<Self::Intv>>) -> Result<()>;
}

pub trait BeforeSaveExperience<Intv>: Command<Result = Result<()>> {
    fn with_subject(self, subject: &Experience<Intv>) -> Self;
    fn with_timeline(self, timeline: &[&Experience<Intv>]) -> Self;
}

pub trait PluginFactory {
    type Intv: Interval;
    type BeforeSaveExperience: BeforeSaveExperience<Self::Intv>;

    fn before_save_experience(&self) -> Self::BeforeSaveExperience;
}

pub struct ExperienceApplication<ExperienceRepo, EntityRepo, EventRepo, PluginFactory> {
    pub experience_repo: Arc<ExperienceRepo>,
    pub entity_repo: Arc<EntityRepo>,
    pub event_repo: Arc<EventRepo>,
    pub plugin_factory: Arc<PluginFactory>,
}
