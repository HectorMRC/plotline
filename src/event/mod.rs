#[cfg(feature = "cli")]
pub mod cli;
#[cfg(feature = "in_memory")]
pub mod repository;
pub mod service;

mod error;
pub use error::*;

use crate::{
    id::{Id, Identifiable},
    interval::Interval,
    name::Name,
};
use serde::{Deserialize, Serialize};

/// An Event is a specific happening in which one or more entities are involved.
#[derive(Clone, Serialize, Deserialize)]
pub struct Event<Intv> {
    id: Id<Self>,
    name: Name<Self>,
    /// the interval is the time during which the event takes place.
    interval: Intv,
}

impl<Intv> Identifiable<Event<Intv>> for Event<Intv> {
    fn id(&self) -> Id<Self> {
        self.id
    }
}

impl<Intv> Interval for Event<Intv>
where
    Intv: Interval,
{
    type Bound = Intv::Bound;

    fn lo(&self) -> Self::Bound {
        self.interval.lo()
    }

    fn hi(&self) -> Self::Bound {
        self.interval.hi()
    }
}

impl<Intv> Event<Intv> {
    /// Creates a new event with the given id.
    pub fn new(id: Id<Self>, name: Name<Self>, interval: Intv) -> Self {
        Self { id, name, interval }
    }
}
