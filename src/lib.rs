use std::sync::Arc;

/// An Entity is anything which to interact with.
pub struct Entity {}

/// A Moment answers the "when", giving the order of time.
pub struct Moment {
    /// The previous moment immediately before self.
    before: Option<Arc<Moment>>,
    /// The next moment immediately after self.
    after: Option<Arc<Moment>>,
}

/// A Period represents the time being between two different [Moment]s in time.
pub struct Period([Arc<Moment>; 2]);

/// A Duration represents the time during which something takes place.
pub enum Duration {
    Moment(Arc<Moment>),
    Period(Arc<Period>),
}

/// An Event is the relation between time and one or more entities.
pub struct Event {
    duration: Arc<Duration>,
    entities: Vec<Arc<Entity>>,
}

/// An Effect is the way an [Entity] has mutated.
pub enum Effect {
    Change(Arc<Entity>),
    Split(Vec<Arc<Entity>>),
    Terminal,
}

/// A Mutation represents the way an [Event] has affected an [Entity].
pub struct Mutation {
    /// The Entity involved in the causing event.
    subject: Arc<Entity>,
    /// The event that has triggered the mutation.
    cause: Arc<Event>,
    /// The effect of the mutation itself.
    effect: Effect,
}
