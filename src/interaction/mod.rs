use crate::{entity::Entity, timeline::Duration};
use std::sync::Arc;

/// An Interaction is the relation between time and one or more entities.
pub struct Interaction {
    duration: Arc<Duration>,
    entities: Vec<Arc<Entity>>,
}

/// InteractionEffect represents all the possible ways an [Interaction] may affect.
pub enum InteractionEffect {
    /// The entity becomes a different one (any of its attributes has changed).
    Change(Arc<Entity>),
    /// The entity becomes two or more new entities.
    Split(Vec<Arc<Entity>>),
    /// The entity no longer exists.
    Terminal,
    /// The entity begins to exists.
    Initial,
}

/// An InteractionResult is the way an [Entity] has been affected by an [Interaction].
pub struct InteractionResult {
    /// The Entity involved in the causing interaction.
    entity: Arc<Entity>,
    /// The Interaction causing the Effect.
    cause: Arc<Interaction>,
    /// The effect of the mutation itself.
    effect: InteractionEffect,
}
