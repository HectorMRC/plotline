pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, PartialEq, thiserror::Error)]
pub enum Error {
    // repo
    #[error("experience already exists")]
    AlreadyExists,
    #[error("experience not found")]
    NotFound,
    // application
    #[error("{0}: must to be set")]
    MandatoryField(&'static str),
    // domain
    #[error("an entity cannot result from the same experience more than once")]
    RepeatedEntity,
    // foreign
    #[error("{0}")]
    Entity(#[from] crate::entity::Error),
    #[error("{0}")]
    Event(#[from] crate::event::Error),
    #[error("{plugin_id}: {error_msg}")]
    Plugin {
        plugin_id: String,
        error_msg: String,
    },
}
