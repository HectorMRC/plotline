pub mod application;
pub mod constraint;
pub mod query;
#[cfg(feature = "in_memory")]
pub mod repository;

mod error;
pub use error::*;

use crate::{
    entity::Entity,
    event::Event,
    id::{Id, Identifiable},
    interval::Interval,
};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

/// A Profile describes an [Entity] during the time being between two periods
/// of time.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Profile {
    entity: Id<Entity>,
    values: HashMap<String, String>,
}

impl Identifiable for Profile {
    type Id = <Entity as Identifiable>::Id;

    fn id(&self) -> Self::Id {
        self.entity
    }
}

impl Profile {
    pub fn new(entity: Id<Entity>) -> Self {
        Self {
            entity,
            values: HashMap::new(),
        }
    }

    pub fn values(&self) -> impl Iterator<Item = (&str, &str)> {
        self.values
            .iter()
            .map(|(key, value)| (key.as_str(), value.as_str()))
    }
}

/// An Experience represents the change caused by an [Event] on an [Entity].
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Experience<Intv> {
    /// The id of the entity involved in the experience.
    entity: Id<Entity>,
    /// The id of the event causing the experience.
    event: Id<Event<Intv>>,
    /// The profiles resulting from the experience.
    profiles: Vec<Profile>,
}

impl<Intv> Identifiable for Experience<Intv> {
    type Id = (Id<Entity>, Id<Event<Intv>>);

    fn id(&self) -> Self::Id {
        (self.entity, self.event)
    }
}

impl<Intv> Experience<Intv> {
    pub fn with_profiles(mut self, profiles: Vec<Profile>) -> Self {
        self.profiles = profiles;
        self
    }

    pub fn profiles(&self) -> &[Profile] {
        &self.profiles
    }
}

/// ExperienceBuilder makes sure an [Experience] is created if, and only if,
/// all of its requirements are meet.
pub struct ExperienceBuilder<'a, Intv> {
    entity: &'a Entity,
    event: &'a Event<Intv>,
    profiles: Option<Vec<Profile>>,
}

impl<'a, Intv: Clone> Clone for ExperienceBuilder<'a, Intv> {
    fn clone(&self) -> Self {
        Self {
            entity: self.entity,
            event: self.event,
            profiles: self.profiles.clone(),
        }
    }
}

impl<'a, Intv> ExperienceBuilder<'a, Intv> {
    pub fn new(entity: &'a Entity, event: &'a Event<Intv>) -> Self {
        Self {
            entity,
            event,
            profiles: Default::default(),
        }
    }

    pub fn with_profiles(mut self, profiles: Option<Vec<Profile>>) -> Self {
        self.profiles = profiles;
        self
    }

    pub fn build(mut self) -> Result<Experience<Intv>> {
        if let Some(profiles) = self.profiles.as_mut() {
            let mut uniq = HashSet::new();
            if !profiles
                .iter()
                .all(move |profile| uniq.insert(profile.entity))
            {
                return Err(Error::RepeatedEntity);
            }
        }

        Ok(Experience {
            entity: self.entity.id(),
            event: self.event.id(),
            profiles: self.profiles.unwrap_or_default(),
        })
    }
}

impl<'a, Intv> ExperienceBuilder<'a, Intv>
where
    Intv: Interval,
{
    /// Tries to compute some value for those fields set to [Option::None].
    pub fn with_fallbacks(mut self, experienced_events: &[ExperiencedEvent<'a, Intv>]) -> Self {
        let mut previous = query::SelectPreviousExperience::new(self.event);
        let mut next = query::SelectNextExperience::new(self.event);
        for experienced_event in experienced_events.iter() {
            previous = previous.with(experienced_event);
            next = next.with(experienced_event);
        }

        self.profiles = self.profiles.or_else(|| {
            previous
                .value()
                .or_else(|| next.value())
                .and_then(|experienced_event| {
                    experienced_event
                        .experience
                        .profiles
                        .iter()
                        .find(|profile| profile.entity == self.entity.id())
                        .cloned()
                })
                .map(|profile| vec![profile])
        });

        self
    }
}

/// An ExperienceKind determines the kind of an [Experience] based on its
/// cardinality.
#[derive(Clone, Copy)]
pub enum ExperienceKind {
    /// The [Entity] has reached the end of its timeline.
    Terminal,
    /// The [Entity] is evolving.
    Transitive,
}

impl<Intv> From<&Experience<Intv>> for ExperienceKind {
    fn from(experience: &Experience<Intv>) -> Self {
        if experience.profiles.is_empty() {
            ExperienceKind::Terminal
        } else {
            ExperienceKind::Transitive
        }
    }
}

impl ExperienceKind {
    pub fn is_transitive(&self) -> bool {
        matches!(self, ExperienceKind::Transitive)
    }

    pub fn is_terminal(&self) -> bool {
        matches!(self, ExperienceKind::Terminal)
    }
}

/// An ExperiencedEvent represents the union between an [Experience] and the
/// [Event] that causes it.
#[derive(PartialEq, Eq)]
pub struct ExperiencedEvent<'a, Intv> {
    experience: &'a Experience<Intv>,
    event: &'a Event<Intv>,
}

impl<Intv> Ord for ExperiencedEvent<'_, Intv>
where
    Intv: Interval,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.event.cmp(other.event)
    }
}

impl<Intv> PartialOrd for ExperiencedEvent<'_, Intv>
where
    Intv: Interval,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a, Intv> ExperiencedEvent<'a, Intv> {
    pub fn event(&self) -> &Event<Intv> {
        self.event
    }
}

#[cfg(test)]
mod tests {
    use super::{Experience, Profile};
    use crate::id::Id;

    pub fn transitive_experience<Intv>() -> Experience<Intv> {
        Experience {
            entity: Id::default(),
            event: Id::default(),
            profiles: vec![Profile::new(Id::default())],
        }
    }

    pub fn terminal_experience<Intv>() -> Experience<Intv> {
        Experience {
            entity: Id::default(),
            event: Id::default(),
            profiles: Vec::default(),
        }
    }
}
