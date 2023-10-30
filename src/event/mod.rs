use crate::{entity::EntityID, id::ID};
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

/// EventID determines an instance of [ID] belongs to an [Event].
pub struct EventID;

/// An Event is a specific happening in which one or more entities are involved.
pub struct Event {
    entities: Vec<ID<EntityID>>,
    duration: Duration,
}
