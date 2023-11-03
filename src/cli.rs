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
    #[error("name: {0}")]
    Name(#[from] crate::name::Error),
    #[error("id: {0}")]
    Id(#[from] crate::id::Error),
    #[error("io: {0}")]
    Io(#[from] std::io::Error),
}
