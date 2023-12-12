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
    #[error("an initial experience cannot follows a non-terminal one")]
    InitialFollowsNonTerminal,
    #[error("a transitive experience cannot follows a terminal one")]
    TransitiveFollowsTerminal,
    #[error("a terminal experience cannot follows a terminal one")]
    TerminalFollowsTerminal,
    #[error("an initial experience cannot precede an initial one")]
    InitialPrecedesInitial,
    #[error("a transitive experience cannot precede an initial one")]
    TransitivePrecedesInitial,
    #[error("a terminal experience cannot precede a non-initial one")]
    TerminalPrecedesNonInitial,
    #[error("an initial experience cannot result in more than one entity")]
    InitialResultsInMoreThanOne,
    // foreign
    #[error("{0}")]
    Entity(#[from] crate::entity::Error),
    #[error("{0}")]
    Event(#[from] crate::event::Error),
    #[error("{0}")]
    Lock(String),
}
