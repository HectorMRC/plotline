use crate::{event::Event, id::Id, profile::Profile};

/// An Experience represents the change caused by an [Event] on an [Entity].
pub struct Experience<Intv> {
    event: Id<Event<Intv>>,
    before: Id<Profile>,
    after: Id<Profile>,
}
