pub mod application;
pub mod domain;

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

impl Profile {
    pub fn new(entity: Id<Entity>) -> Self {
        Self {
            entity,
            values: HashMap::new(),
        }
    }
}

/// An Experience represents the change caused by an [Event] on an [Entity].
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Experience<Intv> {
    event: Id<Event<Intv>>,
    before: Option<Profile>,
    after: Vec<Profile>,
}

impl<Intv> Identifiable for Experience<Intv> {
    type Id = (Id<Entity>, Id<Event<Intv>>);

    fn id(&self) -> Self::Id {
        todo!()
    }
}

impl<Intv> Experience<Intv> {
    pub fn with_before(mut self, before: Option<Profile>) -> Self {
        self.before = before;
        self
    }

    pub fn with_after(mut self, after: Vec<Profile>) -> Self {
        self.after = after;
        self
    }
}

/// ExperienceBuilder makes sure an [Experience] is created if, and only if,
/// all of its requirements are meet.
pub struct ExperienceBuilder<'a, Intv> {
    event: &'a Event<Intv>,
    before: Option<Profile>,
    after: Option<Vec<Profile>>,
}

impl<'a, Intv> ExperienceBuilder<'a, Intv> {
    pub fn new(event: &'a Event<Intv>) -> Self {
        Self {
            event,
            before: Default::default(),
            after: Default::default(),
        }
    }

    pub fn with_before(mut self, before: Option<Profile>) -> Self {
        self.before = before;
        self
    }

    pub fn with_after(mut self, after: Option<Vec<Profile>>) -> Self {
        self.after = after;
        self
    }

    pub fn build(mut self) -> Result<Experience<Intv>> {
        if self.before.is_none() && self.after.as_ref().map(Vec::is_empty).unwrap_or(true) {
            return Err(Error::EmptyBeforeAndAfter);
        }

        if let Some(after) = self.after.as_mut() {
            let mut uniq = HashSet::new();
            if !after.iter().all(move |profile| uniq.insert(profile.entity)) {
                return Err(Error::RepeatedEntity);
            }
        }

        let experience = Experience {
            event: self.event.id(),
            before: self.before,
            after: self.after.unwrap_or_default(),
        };

        if ExperienceKind::from(&experience).is_initial() && 1 != experience.after.len() {
            return Err(Error::InitialResultsInMoreThanOne);
        }

        Ok(experience)
    }
}

/// An ExperienceKind determines the kind of an [Experience] based on its
/// cardinality.
pub enum ExperienceKind {
    Initial,
    Terminal,
    Transitive,
}

impl<Intv> From<&Experience<Intv>> for ExperienceKind {
    fn from(experience: &Experience<Intv>) -> Self {
        if experience.before.is_none() {
            ExperienceKind::Initial
        } else if experience.after.is_empty() {
            ExperienceKind::Terminal
        } else {
            ExperienceKind::Transitive
        }
    }
}

impl ExperienceKind {
    pub fn is_initial(&self) -> bool {
        matches!(self, ExperienceKind::Initial)
    }

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

#[cfg(test)]
mod tests {
    use super::{Experience, Profile};
    use crate::id::Id;

    pub fn initial_experience<Intv>() -> Experience<Intv> {
        Experience {
            event: Id::default(),
            before: None,
            after: vec![Profile::new(Id::default())],
        }
    }

    pub fn transitive_experience<Intv>() -> Experience<Intv> {
        Experience {
            event: Id::default(),
            before: Some(Profile::new(Id::default())),
            after: vec![Profile::new(Id::default())],
        }
    }

    pub fn terminal_experience<Intv>() -> Experience<Intv> {
        Experience {
            event: Id::default(),
            before: Some(Profile::new(Id::default())),
            after: Vec::default(),
        }
    }
}
