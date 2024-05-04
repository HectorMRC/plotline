/// RunPluginResult represents the output or crashing cause of a plugin.
pub type RunPluginResult = std::result::Result<Vec<u8>, String>;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("plugin not found")]
    NotFound,
    #[error("plugin already exists ")]
    AlreadyExists,
    #[error("plugin is not of the expected kind")]
    WrongKind,
    #[error("invalid plugin kind")]
    NotAPluginKind,
    #[error("field is missing: {0}")]
    MissingField(&'static str),
    #[error("invalid plugin id")]
    NotAnId,
    #[error("invalid plugin version: {0}")]
    NotAVersion(#[from] semver::Error),
    #[error("{0}")]
    Id(#[from] plotline::id::Error),
    #[error("{0}")]
    Name(#[from] plotline::name::Error),
    #[error("{0}")]
    Interval(String),
}
