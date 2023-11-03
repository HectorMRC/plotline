use crate::{
    event::Event,
    id::Id,
    name::Name,
};

pub struct CreateEvent<R> {
    event_repo: R,
    name: Name<Event>,
    id: Option<Id<Event>>,
}
