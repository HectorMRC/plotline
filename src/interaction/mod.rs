use crate::{entity::Entity, moment::Duration};
use std::sync::Arc;

/// An Interaction is the relation between time and one or more entities.
pub struct Interaction {
    duration: Arc<Duration>,
    entities: Vec<Arc<Entity>>,
}

/// An Effect is the way an [Entity] has changed.
pub enum Effect {
    /// The entity becomes a different one (any of its attributes has changed).
    Change(Arc<Entity>),
    /// The entity becomes two or more new entities.
    Split(Vec<Arc<Entity>>),
    /// The entity no longer exists.
    Terminal,
    /// The entity begins to exists.
    Initial,
}

/// An Outcome is the [Effect] of an [Interaction] on an [Entity].
pub struct Outcome {
    /// The Entity involved in the causing interaction.
    subject: Arc<Entity>,
    /// The Interaction causing the Effect.
    cause: Arc<Interaction>,
    /// The effect of the mutation itself.
    effect: Effect,
}
