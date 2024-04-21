pub mod application;
pub mod query;

pub mod profile;
pub use profile::*;

#[cfg(feature = "in_memory")]
pub mod repository;

mod error;
pub use error::*;

use crate::{
    entity::Entity,
    event::Event,
    id::{Id, Indentify},
    interval::Interval,
};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// An Experience represents the change caused by an [Event] on an [Entity].
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Experience<Intv> {
    /// The id of the experience.
    #[serde(default)]
    pub id: Id<Self>,
    /// The entity involved in the experience.
    pub entity: Entity,
    /// The event causing the experience.
    pub event: Event<Intv>,
    /// The profiles resulting from the experience.
    pub profiles: Vec<Profile>,
}

impl<Intv> Indentify for Experience<Intv> {
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

    // pub fn kind(&self) -> ExperienceKind {
    //     self.into()
    // }
}

/// ExperienceBuilder makes sure an [Experience] is created if, and only if,
/// all of its requirements are meet.
pub struct ExperienceBuilder<'a, Intv> {
    id: Id<Experience<Intv>>,
    entity: &'a Entity,
    event: &'a Event<Intv>,
    profiles: Option<Vec<Profile>>,
}

impl<'a, Intv> ExperienceBuilder<'a, Intv>
where
    Intv: Clone,
{
    pub fn new(entity: &'a Entity, event: &'a Event<Intv>) -> Self {
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
            entity: self.entity.clone(),
            event: self.event.clone(),
            profiles: self.profiles.unwrap_or_default(),
        })
    }
}

// /// An ExperienceKind determines the kind of an [Experience] based on its
// /// cardinality.
// #[derive(Clone, Copy)]
// pub enum ExperienceKind {
//     /// The entity has reached the end of its timeline.
//     /// Implies the experience has no profile for self.entity.
//     Terminal,
//     /// The entity is evolving.
//     /// Implies the experience has a profile for self.entity.
//     Transitive,
// }
//
// impl<Intv> From<&Experience<Intv>> for ExperienceKind {
//     fn from(experience: &Experience<Intv>) -> Self {
//         if experience
//             .profiles
//             .iter()
//             .find(|profile| profile.entity == experience.entity)
//             .is_some()
//         {
//             ExperienceKind::Transitive
//         } else {
//             ExperienceKind::Terminal
//         }
//     }
// }
//
// impl ExperienceKind {
//     pub fn is_transitive(&self) -> bool {
//         matches!(self, ExperienceKind::Transitive)
//     }
//
//     pub fn is_terminal(&self) -> bool {
//         matches!(self, ExperienceKind::Terminal)
//     }
// }

#[cfg(any(test, feature = "fixtures"))]
pub mod fixtures {
    use super::{Event, Experience, Profile};
    use crate::{entity::Entity, id::Id};

    impl<Intv> Experience<Intv> {
        pub fn fixture(interval: impl Into<Intv>) -> Self {
            Experience {
                id: Id::default(),
                entity: Entity::fixture(),
                event: Event::fixture(interval),
                profiles: Default::default(),
            }
        }
    }

    impl Profile {
        pub fn fixture() -> Self {
            Profile {
                entity: Entity::fixture(),
                values: Default::default(),
            }
        }
    }
}
