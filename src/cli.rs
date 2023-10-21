use clap::error::ErrorKind;

pub type CliResult = std::result::Result<(), CliError>;

impl From<CliError> for CliResult {
    fn from(value: CliError) -> Self {
        Self::Err(value)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum CliError {
    #[error("{0}")]
    Entity(#[from] crate::entity::Error),
    #[error("{0}")]
    Tag(#[from] crate::tag::Error),
    #[error("{0}")]
    Io(#[from] std::io::Error),
}

impl Into<clap::Error> for CliError {
    fn into(self) -> clap::Error {
        clap::Error::raw(ErrorKind::Io, self)
    }
}
