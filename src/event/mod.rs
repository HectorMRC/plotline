use crate::{entity::EntityId, id::Id};
use std::sync::Arc;

/// A Moment answers the "when", giving the order of time.
pub struct Moment;

/// A Period is the time being between two different [Moment]s in time. Both included.
pub struct Period([Arc<Moment>; 2]);

/// A Duration is the time during which something takes place.
pub enum Duration {
    Moment(Arc<Moment>),
    Period(Arc<Period>),
}

/// EventId determines an instance of [Id] belongs to an [Event].
pub struct EventId;

/// An Event is a specific happening in which one or more entities are involved.
pub struct Event {
    entities: Vec<Id<EntityId>>,
    duration: Duration,
}
