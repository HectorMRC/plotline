pub mod constraint;
pub mod service;

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
#[derive(Serialize, Deserialize)]
pub struct Experience<Intv> {
    event: Id<Event<Intv>>,
    before: Option<Profile>,
    after: Option<Profile>,
}

impl<Intv> Experience<Intv> {
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
}
