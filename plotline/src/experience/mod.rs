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
    interval::Interval, macros,
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

impl<Intv> Experience<Intv> {
    pub fn with_profiles(mut self, profiles: Vec<Profile>) -> Self {
        self.profiles = profiles;
        self
    }

    pub fn profiles(&self) -> &[Profile] {
        &self.profiles
    }
}

macros::interval_based_ord_for!(event as Intv in Experience<Intv>);

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
