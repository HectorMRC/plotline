//! The application service.

mod save;
pub use save::*;

mod filter;
pub use filter::*;

mod find;
pub use find::*;

use super::error::Result;
use crate::{
    experience::Experience,
    id::{Id, Indentify},
    interval::Interval,
    plugin::{Plugin, PluginGroup, PluginId},
    transaction::Tx,
};
use std::sync::Arc;

#[trait_variant::make]
pub trait ExperienceRepository {
    type Intv: Interval;
    type Tx: Tx<Experience<Self::Intv>>;

    async fn find(&self, id: Id<Experience<Self::Intv>>) -> Result<Self::Tx>;
    async fn filter(&self, filter: &ExperienceFilter<Self::Intv>) -> Result<Vec<Self::Tx>>;
    async fn create(&self, experience: &Experience<Self::Intv>) -> Result<()>;
    async fn delete(&self, id: Id<Experience<Self::Intv>>) -> Result<()>;
}

pub trait BeforeSaveExperience<'a, Intv>: Indentify<Id = PluginId> + Plugin<()> {
    fn with_subject(self, subject: &'a Experience<Intv>) -> Self;
    fn with_timeline(self, timeline: &'a [&Experience<Intv>]) -> Self;
}

pub trait PluginFactory {
    type Intv: Interval;
    type BeforeSaveExperience<'a>: BeforeSaveExperience<'a, Self::Intv>
    where
        Self: 'a;

    fn before_save_experience(&self) -> PluginGroup<Self::BeforeSaveExperience<'_>>;
}

pub struct ExperienceApplication<ExperienceRepo, EntityRepo, EventRepo, PluginFactory> {
    pub experience_repo: Arc<ExperienceRepo>,
    pub entity_repo: Arc<EntityRepo>,
    pub event_repo: Arc<EventRepo>,
    pub plugin_factory: Arc<PluginFactory>,
}
