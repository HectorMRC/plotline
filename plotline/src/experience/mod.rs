pub mod application;
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
    pub entity: Entity,
    pub values: HashMap<String, String>,
}

impl Identifiable for Profile {
    type Id = <Entity as Identifiable>::Id;

    fn id(&self) -> Self::Id {
        self.entity.id()
    }
}

impl Profile {
    pub fn new(entity: Entity) -> Self {
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
    pub id: Id<Self>,
    /// The entity involved in the experience.
    pub entity: Entity,
    /// The event causing the experience.
    pub event: Event<Intv>,
    /// The profiles resulting from the experience.
    pub profiles: Vec<Profile>,
}

impl<Intv> Identifiable for Experience<Intv> {
    type Id = Id<Self>;

    fn id(&self) -> Self::Id {
        self.id
    }
}

impl<Intv> Ord for Experience<Intv>
where
    Intv: Interval,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.event.cmp(&other.event)
    }
}

impl<Intv> PartialOrd for Experience<Intv>
where
    Intv: Interval,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
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
pub struct ExperienceBuilder<Intv> {
    id: Id<Experience<Intv>>,
    entity: Entity,
    event: Event<Intv>,
    profiles: Option<Vec<Profile>>,
}

impl<Intv> ExperienceBuilder<Intv> {
    pub fn new(entity: Entity, event: Event<Intv>) -> Self {
        Self {
            id: Default::default(),
            entity,
            event,
            profiles: Default::default(),
        }
    }

    pub fn with_id(mut self, id: Id<Experience<Intv>>) -> Self {
        self.id = id;
        self
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
                .all(move |profile| uniq.insert(profile.entity.id()))
            {
                return Err(Error::RepeatedEntity);
            }
        }

        Ok(Experience {
            id: self.id,
            entity: self.entity,
            event: self.event,
            profiles: self.profiles.unwrap_or_default(),
        })
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
