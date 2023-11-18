pub mod service;

mod error;
pub use error::*;

use crate::{event::Event, id::Id, profile::Profile};
use serde::{Deserialize, Serialize};

/// An Experience represents the change caused by an [Event] on an [Entity].
#[derive(Serialize, Deserialize)]
pub struct Experience<Intv> {
    event: Id<Event<Intv>>,
    before: Id<Profile>,
    after: Id<Profile>,
}
