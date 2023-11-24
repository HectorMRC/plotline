pub mod application;
pub mod domain;

mod error;
pub use error::*;

use crate::{entity::Entity, event::Event, id::Id};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A Profile describes an [Entity] during the time being between two periods
/// of time.
#[derive(Clone, Serialize, Deserialize)]
pub struct Profile {
    entity: Id<Entity>,
    values: HashMap<String, String>,
}

/// An Experience represents the change caused by an [Event] on an [Entity].
#[derive(Clone, Serialize, Deserialize)]
pub struct Experience<Intv> {
    event: Id<Event<Intv>>,
    before: Option<Profile>,
    after: Option<Profile>,
}

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

/// An ExperiencedEvent represents the union between an [Experience] and the [Event] where it takes
/// place.
pub struct ExperiencedEvent<'a, Intv> {
    _experience: &'a Experience<Intv>,
    event: &'a Event<Intv>,
}

impl<'a, Intv> AsRef<Intv> for ExperiencedEvent<'a, Intv> {
    fn as_ref(&self) -> &Intv {
        &self.event.interval
    }
}