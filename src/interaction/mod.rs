use std::sync::Arc;

pub trait Duration: PartialEq + PartialOrd {}
pub trait Entity: PartialEq {}

/// An Interaction is the relation between time and one or more entities.
pub struct Interaction<D, E> {
    duration: Arc<D>,
    entities: Vec<Arc<E>>,
}

/// InteractionEffect represents all the possible ways an [Interaction] may affect.
pub enum InteractionEffect<E> {
    /// The entity becomes a different one (any of its attributes has changed).
    Change(Arc<E>),
    /// The entity becomes two or more new entities.
    Split(Vec<Arc<E>>),
    /// The entity no longer exists.
    Terminal,
    /// The entity begins to exists.
    Initial,
}

/// An InteractionResult is the way an [Entity] has been affected by an [Interaction].
pub struct InteractionResult<D, E> {
    /// The Entity involved in the causing interaction.
    entity: Arc<E>,
    /// The Interaction causing the Effect.
    cause: Arc<Interaction<D, E>>,
    /// The effect of the mutation itself.
    effect: InteractionEffect<E>,
}
