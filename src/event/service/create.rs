use crate::{
    event::Event,
    id::Id,
    name::Name,
};

use super::EventRepository;

pub struct CreateEvent<R>
where
    R: EventRepository {
    event_repo: R,
    name: Name<Event<R::Interval>>,
    id: Option<Id<Event<R::Interval>>>,
}
