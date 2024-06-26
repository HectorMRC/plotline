use super::{ExperienceApplication, ExperienceRepository};
use crate::{
    event::application::EventRepository,
    experience::{Experience, Result},
    id::Indentify,
    interval::Interval,
    transaction::Tx,
};
use std::sync::Arc;

/// Implements the find query, through which one, and exactly one, entity must
/// be retrived.
#[derive(Default)]
pub struct FindExperience<ExperienceRepo, Intv> {
    experience_repo: Arc<ExperienceRepo>,
    id: <Experience<Intv> as Indentify>::Id,
}

impl<ExperienceRepo, Intv> FindExperience<ExperienceRepo, Intv>
where
    ExperienceRepo: ExperienceRepository<Intv = Intv>,
    Intv: Interval,
{
    /// Executes the find query, through which one, and exactly one, experience
    /// must be retrived.
    pub async fn execute(self) -> Result<Experience<Intv>> {
        Ok(self
            .experience_repo
            .find(self.id)
            .await?
            .read()
            .await
            .clone())
    }
}

impl<ExperienceRepo, EntityRepo, EventRepo, PluginFcty>
    ExperienceApplication<ExperienceRepo, EntityRepo, EventRepo, PluginFcty>
where
    ExperienceRepo: ExperienceRepository<Intv = EventRepo::Intv>,
    EventRepo: EventRepository,
{
    pub fn find_experience(
        &self,
        id: <Experience<EventRepo::Intv> as Indentify>::Id,
    ) -> FindExperience<ExperienceRepo, EventRepo::Intv> {
        FindExperience {
            experience_repo: self.experience_repo.clone(),
            id,
        }
    }
}
