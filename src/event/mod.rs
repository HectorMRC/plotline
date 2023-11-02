#[cfg(feature = "in_memory")]
pub mod repository;
pub mod service;

mod error;
pub use error::*;

use crate::{
    entity::EntityId,
    id::Id,
    interval::Interval,
    name::Name,
    timeline::{Moment, Period},
};

/// EventId determines an instance of [Id] belongs to an [Event].
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct EventId;

/// EventName determines an instance of [Name] belongs to an [Event].
#[derive(Clone)]
pub struct EventName;

/// An Event is a specific happening in which one or more entities are involved.
#[derive(Clone)]
pub struct Event {
    id: Id<EventId>,
    name: Name<EventName>,
    entities: Vec<Id<EntityId>>,
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
