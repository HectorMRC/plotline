pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    // application
    #[error("an entity cannot experience an event more than once")]
    EventAlreadyExperienced,
    // domain
    #[error("an experience cannot be empty of before and after simultaneously")]
    EmptyBeforeAndAfter,
    // constraint
    #[error("an experience cannot happen before the initial one")]
    ExperienceBeforeInitial,
    #[error("an entity cannot be after of the same experience more than once")]
    RepeatedEntity,
    #[error("an experience cannot belong to an entity not listed in the previous experience")]
    NotInPreviousExperience,
    #[error("an entity cannot experience simultaneous events")]
    SimultaneousEvents,
    // foreign
    #[error("{0}")]
    Entity(#[from] crate::entity::Error),
    #[error("{0}")]
    Event(#[from] crate::event::Error),
    #[error("{0}")]
    Lock(String),
}
