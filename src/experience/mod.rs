pub mod application;
#[cfg(feature = "cli")]
pub mod cli;
pub mod query;
pub mod constraint;
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
    entity: Id<Entity>,
    event: Id<Event<Intv>>,
    after: Vec<Profile>,
}

impl<Intv> Identifiable for Experience<Intv> {
    type Id = (Id<Entity>, Id<Event<Intv>>);

    fn id(&self) -> Self::Id {
        todo!()
    }
}

impl<Intv> Experience<Intv> {
    pub fn with_after(mut self, after: Vec<Profile>) -> Self {
        self.after = after;
        self
    }
}

/// ExperienceBuilder makes sure an [Experience] is created if, and only if,
/// all of its requirements are meet.
pub struct ExperienceBuilder<'a, Intv> {
    entity: &'a Entity,
    event: &'a Event<Intv>,
    after: Option<Vec<Profile>>,
    kind: Option<ExperienceKind>,
}

impl<'a, Intv: Clone> Clone for ExperienceBuilder<'a, Intv> {
    fn clone(&self) -> Self {
        Self {
            entity: self.entity,
            event: self.event,
            after: self.after.clone(),
            kind: self.kind,
        }
    }
}

impl<'a, Intv> ExperienceBuilder<'a, Intv> {
    pub fn new(entity: &'a Entity, event: &'a Event<Intv>) -> Self {
        Self {
            entity,
            event,
            after: Default::default(),
            kind: Default::default(),
        }
    }

    pub fn with_after(mut self, after: Option<Vec<Profile>>) -> Self {
        self.after = after;
        self
    }

    pub fn with_kind(mut self, kind: Option<ExperienceKind>) -> Self {
        self.kind = kind;
        self
    }

    pub fn build(mut self) -> Result<Experience<Intv>> {
        if let Some(after) = self.after.as_mut() {
            let mut uniq = HashSet::new();
            if !after.iter().all(move |profile| uniq.insert(profile.entity)) {
                return Err(Error::RepeatedEntity);
            }
        }

        Ok(Experience {
            entity: self.entity.id(),
            event: self.event.id(),
            after: self.after.unwrap_or_default(),
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

        self.after = self.after.or(previous
            .value()
            .or(next.value())
            .and_then(|experienced_event| {
                experienced_event
                    .experience
                    .after
                    .iter()
                    .find(|profile| profile.entity == self.entity.id())
                    .cloned()
            })
            .map(|profile| vec![profile]));

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
        if experience.after.is_empty() {
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

#[cfg(test)]
mod tests {
    use super::{Experience, Profile};
    use crate::id::Id;

    pub fn transitive_experience<Intv>() -> Experience<Intv> {
        Experience {
            entity: Id::default(),
            event: Id::default(),
            after: vec![Profile::new(Id::default())],
        }
    }

    pub fn terminal_experience<Intv>() -> Experience<Intv> {
        Experience {
            entity: Id::default(),
            event: Id::default(),
            after: Vec::default(),
        }
    }
}
