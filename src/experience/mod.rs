pub mod application;
pub mod domain;

mod error;
pub use error::*;

use crate::{
    entity::Entity,
    event::Event,
    id::{Id, Identifiable},
    interval::Interval,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A Profile describes an [Entity] during the time being between two periods
/// of time.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Profile {
    entity: Id<Entity>,
    values: HashMap<String, String>,
}

/// An Experience represents the change caused by an [Event] on an [Entity].
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Experience<Intv> {
    event: Id<Event<Intv>>,
    before: Option<Profile>,
    after: Vec<Profile>,
}

/// ExperienceBuilder makes sure an [Experience] is created if, and only if,
/// all of its requirements are meet.
pub struct ExperienceBuilder<'a, Intv> {
    event: &'a Event<Intv>,
    before: Option<Profile>,
    after: Vec<Profile>,
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

    pub fn with_after(mut self, after: Vec<Profile>) -> Self {
        self.after = after;
        self
    }

    pub fn build(self) -> Result<Experience<Intv>> {
        if self.before.is_none() && self.after.is_empty() {
            return Err(Error::MustBeforeOrAfter);
        }

        Ok(Experience {
            event: self.event.id(),
            before: self.before,
            after: self.after,
        })
    }
}

/// An ExperienceKind determines the cardinality of an [Experience].
pub enum ExperienceKind {
    Initial,
    Terminal,
    Transition,
}

impl<Intv> From<&Experience<Intv>> for ExperienceKind {
    fn from(experience: &Experience<Intv>) -> Self {
        if experience.before.is_none() {
            ExperienceKind::Initial
        } else if experience.after.is_empty() {
            ExperienceKind::Terminal
        } else {
            ExperienceKind::Transition
        }
    }
}

impl ExperienceKind {
    pub fn is_initial(&self) -> bool {
        matches!(self, ExperienceKind::Initial)
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
