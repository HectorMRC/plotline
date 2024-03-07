use super::{
    application::{ExperienceFilter, ExperienceRepository},
    Error, Experience, Result,
};
use crate::{
    id::Id,
    interval::Interval,
    resource::{Resource, ResourceMap},
    serde::{from_rwlock, into_rwlock},
    transaction::Tx,
};
use serde::{Deserialize, Serialize};
use std::sync::RwLock;

#[derive(Default, Serialize, Deserialize)]
#[serde(default)]
pub struct InMemoryExperienceRepository<Intv>
where
    Intv: Interval + Serialize + for<'a> Deserialize<'a>,
{
    #[serde(
        serialize_with = "from_rwlock",
        deserialize_with = "into_rwlock",
        default
    )]
    experiences: RwLock<ResourceMap<Experience<Intv>>>,
}

impl<Intv> ExperienceRepository for InMemoryExperienceRepository<Intv>
where
    Intv: Interval + Serialize + for<'a> Deserialize<'a>,
{
    type Interval = Intv;
    type Tx = Resource<Experience<Intv>>;

    fn find(&self, id: Id<Experience<Intv>>) -> Result<Self::Tx> {
        self.experiences
            .read()
            .map_err(Error::from)?
            .get(&id)
            .cloned()
            .map(Resource::from)
            .ok_or(Error::NotFound)
    }

    fn filter(&self, filter: &ExperienceFilter<Intv>) -> Result<Vec<Self::Tx>> {
        Ok(self
            .experiences
            .read()
            .map_err(Error::from)?
            .values()
            .filter(|&entity| filter.matches(&entity.clone().read()))
            .cloned()
            .collect())
    }

    fn create(&self, experience: &Experience<Intv>) -> Result<()> {
        let mut experiences = self.experiences.write().map_err(Error::from)?;

        if experiences.contains_key(&experience.id) {
            return Err(Error::AlreadyExists);
        }

        experiences.insert(experience.id, experience.clone().into());
        Ok(())
    }

    fn delete(&self, id: Id<Experience<Intv>>) -> Result<()> {
        let mut experiences = self.experiences.write().map_err(Error::from)?;

        if experiences.remove(&id).is_none() {
            return Err(Error::NotFound);
        }

        Ok(())
    }
}
