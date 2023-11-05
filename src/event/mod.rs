#[cfg(feature = "in_memory")]
pub mod repository;
pub mod service;

mod error;
pub use error::*;
use serde::{Serialize, Deserialize};

use crate::{
    id::{Id, Identified},
    interval::Interval,
    name::Name,
    entity::Entity,
};

/// An Event is a specific happening in which one or more entities are involved.
#[derive(Clone, Serialize, Deserialize)]
pub struct Event<I> {
    id: Id<Event<I>>,
    name: Name<Event<I>>,
    entities: Vec<Id<Entity>>,
    interval: I,
}

impl<I> Identified<Event<I>> for Event<I> {
    fn id(&self) -> Id<Event<I>> {
        self.id
    }
}

impl<I> Interval for Event<I> 
where
    I: Interval
{
    type Bound = I::Bound;

    fn lo(&self) -> Self::Bound {
        self.interval.lo()
    }

    fn hi(&self) -> Self::Bound {
        self.interval.hi()
    }
}