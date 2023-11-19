pub mod service;

mod error;
pub use error::*;

use crate::{event::Event, id::Id, profile::Profile};
use serde::{Deserialize, Serialize};

/// An Experience represents the change caused by an [Event] on an [Entity].
#[derive(Serialize, Deserialize)]
pub struct Experience<Intv> {
    event: Id<Event<Intv>>,
    before: Option<Id<Profile>>,
    after: Option<Id<Profile>>,
}

impl<Intv> Experience<Intv> {
    pub fn new(event: Id<Event<Intv>>) -> Self {
        Self {
            event,
            before: Default::default(),
            after: Default::default(),
        }
    }

    pub fn with_before(mut self, before: Option<Id<Profile>>) -> Self {
        self.before = before;
        self
    }
    pub fn with_after(mut self, after: Option<Id<Profile>>) -> Self {
        self.after = after;
        self
    }
}
