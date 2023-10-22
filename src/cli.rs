use clap::error::ErrorKind;

pub type CliResult = std::result::Result<(), CliError>;

impl From<CliError> for CliResult {
    fn from(value: CliError) -> Self {
        Self::Err(value)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum CliError {
    #[error("entity: {0}")]
    Entity(#[from] crate::entity::Error),
    #[error("tag: {0}")]
    Tag(#[from] crate::tag::Error),
    #[error("io: {0}")]
    Io(#[from] std::io::Error),
    #[error("uuid: {0}")]
    Uuid(#[from] uuid::Error),
}

impl Into<clap::Error> for CliError {
    fn into(self) -> clap::Error {
        clap::Error::raw(ErrorKind::Io, self)
    }
}
