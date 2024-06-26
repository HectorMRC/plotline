use futures::future;

use super::{ExperienceApplication, ExperienceRepository};
use crate::{
    entity::Entity,
    event::{application::EventRepository, Event},
    experience::{Experience, Result},
    id::Id,
    transaction::Tx,
};
use std::sync::Arc;

/// Implements the filter query, through which zero o more experiences may be
/// retrived.
pub struct ExperienceFilter<Intv> {
    pub id: Option<Id<Experience<Intv>>>,
    /// Determines the [Entity] involved in the experience.
    pub entity: Option<Id<Entity>>,
    /// Determines the [Event] causing the [Experience].
    pub event: Option<Id<Event<Intv>>>,
}

impl<Intv> Default for ExperienceFilter<Intv> {
    fn default() -> Self {
        Self {
            id: Default::default(),
            entity: Default::default(),
            event: Default::default(),
        }
    }
}

impl<Intv> ExperienceFilter<Intv> {
    pub fn with_entity(mut self, id: Option<Id<Entity>>) -> Self {
        self.entity = id;
        self
    }

    pub fn with_event(mut self, id: Option<Id<Event<Intv>>>) -> Self {
        self.event = id;
        self
    }
}

/// Implements the filter query, through which zero o more experiences may be
/// retrived.
pub struct FilterExperiences<ExperienceRepo>
where
    ExperienceRepo: ExperienceRepository,
{
    experience_repo: Arc<ExperienceRepo>,
    filter: ExperienceFilter<ExperienceRepo::Intv>,
}

impl<ExperienceRepo> FilterExperiences<ExperienceRepo>
where
    ExperienceRepo: ExperienceRepository,
{
    pub async fn execute(self) -> Result<Vec<Experience<ExperienceRepo::Intv>>> {
        let mut experiences = future::join_all(
            self.experience_repo
                .filter(&self.filter)
                .await?
                .into_iter()
                .map(|entity_tx| async move { entity_tx.read().await.clone() }),
        )
        .await;

        experiences.sort();
        Ok(experiences)
    }
}

impl<ExperienceRepo> FilterExperiences<ExperienceRepo>
where
    ExperienceRepo: ExperienceRepository,
{
    pub fn with_filter(mut self, filter: ExperienceFilter<ExperienceRepo::Intv>) -> Self {
        self.filter = filter;
        self
    }
}

impl<ExperienceRepo, EntityRepo, EventRepo, PluginFcty>
    ExperienceApplication<ExperienceRepo, EntityRepo, EventRepo, PluginFcty>
where
    ExperienceRepo: ExperienceRepository<Intv = EventRepo::Intv>,
    EventRepo: EventRepository,
{
    pub fn filter_experiences(&self) -> FilterExperiences<ExperienceRepo> {
        FilterExperiences {
            experience_repo: self.experience_repo.clone(),
            filter: Default::default(),
        }
    }
}
