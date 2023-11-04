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
    Timeline(#[from] crate::timeline::Error),
    #[error("{0}")]
    Name(#[from] crate::name::Error),
    #[error("{0}")]
    Id(#[from] crate::id::Error),
    #[error("{0}")]
    Io(#[from] std::io::Error),
}
