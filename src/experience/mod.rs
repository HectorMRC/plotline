pub mod application;
pub mod domain;

mod error;
pub use error::*;

use crate::{entity::Entity, event::Event, id::Id, interval::Interval};
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
    after: Option<Profile>,
}

/// ExperienceBuilder makes sure an [Experience] is created if, and only if,
/// all of its requirements are meet.
pub struct ExperienceBuilder<Intv> {
    event: Id<Event<Intv>>,
    before: Option<Profile>,
    after: Option<Profile>,
}

impl<Intv> ExperienceBuilder<Intv> {
    pub fn new(event: Id<Event<Intv>>) -> Self {
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

    pub fn with_after(mut self, after: Option<Profile>) -> Self {
        self.after = after;
        self
    }

    pub fn build(self) -> Result<Experience<Intv>> {
        if self.before.is_none() && self.after.is_none() {
            return Err(Error::MustBeforeOrAfter);
        }

        Ok(Experience {
            event: self.event,
            before: self.before,
            after: self.after,
        })
    }
}

/// An ExperiencedEvent represents the union between an [Experience] and the
/// [Event] that causes it.
#[derive(PartialEq, Eq)]
pub struct ExperiencedEvent<'a, Intv> {
    experience: &'a Experience<Intv>,
    event: &'a Event<Intv>,
}

impl<'a, Intv> Ord for ExperiencedEvent<'a, Intv>
where
    Intv: Interval,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.event.lo() > other.event.hi() {
            std::cmp::Ordering::Greater
        } else if self.event.hi() < other.event.lo() {
            std::cmp::Ordering::Less
        } else {
            std::cmp::Ordering::Equal
        }
    }
}

impl<'a, Intv> PartialOrd for ExperiencedEvent<'a, Intv>
where
    Intv: Interval,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
