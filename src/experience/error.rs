pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, PartialEq, thiserror::Error)]
pub enum Error {
    // repo
    #[error("experience already exists")]
    AlreadyExists,
    #[error("experience not found")]
    NotFound,
    // application
    #[error("an entity cannot experience an event more than once")]
    EventAlreadyExperienced,
    // domain
    #[error("an entity cannot be after of the same experience more than once")]
    RepeatedEntity,
    // foreign
    #[error("{0}")]
    Entity(#[from] crate::entity::Error),
    #[error("{0}")]
    Event(#[from] crate::event::Error),
    #[error("{0}")]
    Constraint(#[from] super::constraint::Error),
    #[error("{0}")]
    Lock(String),
}
