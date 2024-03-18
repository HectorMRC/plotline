pub mod application;
#[cfg(feature = "in_memory")]
pub mod repository;

mod error;
pub use error::*;

use crate::{
    id::{Id, Identifiable},
    interval::Interval,
    macros,
    name::Name,
};
use serde::{Deserialize, Serialize};

/// An Event is a specific happening in which one or more entities are involved.
#[derive(Clone, Eq, Serialize, Deserialize)]
pub struct Event<Intv> {
    /// The id of the event.
    pub id: Id<Self>,
    /// The name of the event.
    pub name: Name<Self>,
    /// The time during which the event takes place.
    pub interval: Intv,
}

impl<Intv> Identifiable for Event<Intv> {
    type Id = Id<Self>;

    fn id(&self) -> Id<Self> {
        self.id
    }
}

impl<Intv> PartialEq for Event<Intv> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
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

macros::impl_interval_based_ord_for!(Event<Intv> where Intv: Interval);

#[cfg(test)]
pub(crate) mod tests {
    use super::Event;
    use crate::id::Id;

    impl<Intv> Event<Intv> {
        pub fn fixture(interval: impl Into<Intv>) -> Self {
            Event {
                id: Id::default(),
                name: "test".to_string().try_into().unwrap(),
                interval: interval.into(),
            }
        }
    }
}
