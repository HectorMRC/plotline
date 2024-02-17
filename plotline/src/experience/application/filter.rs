use super::{ExperienceApplication, ExperienceRepository};
use crate::{
    entity::Entity,
    event::{application::EventRepository, Event},
    experience::{Experience, Result},
    id::Id,
    macros::equals_or_return,
    transaction::Tx,
};
use std::sync::Arc;

/// Implements the filter query, through which zero o more experiences may be
/// retrived.
pub struct ExperienceFilter<Intv> {
    /// Determines the [Entity] involved in the experience, no matter it is
    /// before or after.
    pub(crate) entity: Option<Id<Entity>>,
    /// Determines the [Event] causing the [Experience].
    pub(crate) event: Option<Id<Event<Intv>>>,
}

impl<Intv> Default for ExperienceFilter<Intv> {
    fn default() -> Self {
        Self {
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

    pub fn filter(&self, experience: &Experience<Intv>) -> bool {
        equals_or_return!(self.event, &experience.event);
        self.filter_by_entity(experience)
    }

    /// An [Experience] not only belongs to the [Entity] associated with it,
    /// but also to each one owning a [Profile] in that experience.  
    fn filter_by_entity(&self, experience: &Experience<Intv>) -> bool {
        let Some(entity_id) = self.entity else {
            return true; // no filter by entity has been set
        };

        entity_id == experience.entity
            || experience
                .profiles
                .iter()
                .any(|profile| profile.entity == entity_id)
    }
}

/// Implements the filter query, through which zero o more experiences may be
/// retrived.
pub struct FilterExperiences<ExperienceRepo>
where
    ExperienceRepo: ExperienceRepository,
{
    experience_repo: Arc<ExperienceRepo>,
    filter: ExperienceFilter<ExperienceRepo::Interval>,
}

impl<ExperienceRepo> FilterExperiences<ExperienceRepo>
where
    ExperienceRepo: ExperienceRepository,
{
    pub fn execute(self) -> Result<Vec<Experience<ExperienceRepo::Interval>>> {
        Ok(self
            .experience_repo
            .filter(&self.filter)?
            .into_iter()
            .map(Tx::begin)
            .map(|experience| experience.clone())
            .collect())
    }
}

impl<ExperienceRepo> FilterExperiences<ExperienceRepo>
where
    ExperienceRepo: ExperienceRepository,
{
    pub fn with_filter(mut self, filter: ExperienceFilter<ExperienceRepo::Interval>) -> Self {
        self.filter = filter;
        self
    }
}

impl<ExperienceRepo, EntityRepo, EventRepo, CnstFactory>
    ExperienceApplication<ExperienceRepo, EntityRepo, EventRepo, CnstFactory>
where
    ExperienceRepo: ExperienceRepository<Interval = EventRepo::Interval>,
    EventRepo: EventRepository,
{
    pub fn filter_experiences(&self) -> FilterExperiences<ExperienceRepo> {
        FilterExperiences {
            experience_repo: self.experience_repo.clone(),
            filter: Default::default(),
        }
    }
}