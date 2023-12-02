use crate::{entity::Entity, id::Id};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("experience not found")]
    NotFound,
    #[error("more than one experience for the same entity and event")]
    MoreThanOne,
    #[error("an experience must include at least one before or after")]
    MustBeforeOrAfter,
    #[error("the event has already been experienced by the entity")]
    EventAlreadyExperienced,
    #[error("an experience cannot happen before the initial one")]
    BeforeInitial,
    #[error("an entity cannot be after of the same experience more than once")]
    RepeatedEntity,
    #[error("the profile before of the experience must belong to one of {0:?}")]
    ExperienceMustBelongToOneOf(Vec<Id<Entity>>),
    #[error("{0}")]
    Entity(#[from] crate::entity::Error),
    #[error("{0}")]
    Event(#[from] crate::event::Error),
    #[error("{0}")]
    Lock(String),
}
