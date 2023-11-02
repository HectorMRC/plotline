use crate::{
    event::{EventId, EventName},
    id::Id,
    name::Name,
};

pub struct CreateEvent<R> {
    event_repo: R,
    name: Name<EventName>,
    id: Option<Id<EventId>>,
}
