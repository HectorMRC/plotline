#[cfg(feature = "in_memory")]
pub mod repository;
pub mod service;

mod error;
pub use error::*;

use crate::{
    id::Id,
    interval::Interval,
    name::Name,
    timeline::{Moment, Period}, entity::Entity,
};

/// An Event is a specific happening in which one or more entities are involved.
#[derive(Clone)]
pub struct Event {
    id: Id<Event>,
    name: Name<Event>,
    entities: Vec<Id<Entity>>,
    period: Period,
}

impl Interval for Event {
    type Bound = Moment;

    fn lo(&self) -> Self::Bound {
        self.period.lo()
    }

    fn hi(&self) -> Self::Bound {
        self.period.hi()
    }
}
