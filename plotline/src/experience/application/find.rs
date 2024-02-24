use super::{ExperienceApplication, ExperienceFilter, ExperienceRepository};
use crate::{
    event::application::EventRepository,
    experience::{Error, Experience, Result},
    id::Identifiable,
    interval::Interval,
    transaction::Tx,
};
use std::sync::Arc;

/// Implements the find query, through which one, and exactly one, entity must
/// be retrived.
#[derive(Default)]
pub struct FindExperience<ExperienceRepo, Intv> {
    experience_repo: Arc<ExperienceRepo>,
    id: <Experience<Intv> as Identifiable>::Id,
}

impl<ExperienceRepo, Intv> FindExperience<ExperienceRepo, Intv>
where
    ExperienceRepo: ExperienceRepository<Interval = Intv>,
    Intv: Interval,
{
    /// Executes the find query, through which one, and exactly one, experience
    /// must be retrived.
    pub fn execute(self) -> Result<Experience<Intv>> {
        self.experience_repo
            .filter(
                &ExperienceFilter::default()
                    .with_entity(Some(self.id.0))
                    .with_event(Some(self.id.1)),
            )?
            .into_iter()
            .next()
            .map(Tx::read)
            .as_deref()
            .cloned()
            .ok_or(Error::NotFound)
    }
}

impl<ExperienceRepo, EntityRepo, EventRepo, CnstFactory>
    ExperienceApplication<ExperienceRepo, EntityRepo, EventRepo, CnstFactory>
where
    ExperienceRepo: ExperienceRepository<Interval = EventRepo::Interval>,
    EventRepo: EventRepository,
{
    pub fn find_experience(
        &self,
        id: <Experience<EventRepo::Interval> as Identifiable>::Id,
    ) -> FindExperience<ExperienceRepo, EventRepo::Interval> {
        FindExperience {
            experience_repo: self.experience_repo.clone(),
            id,
        }
    }
}
