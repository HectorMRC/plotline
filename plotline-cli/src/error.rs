use std::fmt::Display;

pub type Result = std::result::Result<(), Error>;

impl From<Error> for Result {
    fn from(value: Error) -> Self {
        Self::Err(value)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("{0}: argument is missing")]
    MissingArgument(&'static str),
    #[error("{0}")]
    Entity(#[from] plotline::entity::Error),
    #[error("{0}")]
    Name(#[from] plotline::name::Error),
    #[error("{0}")]
    Id(#[from] plotline::id::Error),
    #[error("{0}")]
    Event(#[from] plotline::event::Error),
    #[error("{0}")]
    Experience(#[from] plotline::experience::Error),
    #[error("{0}")]
    Io(#[from] std::io::Error),
    #[error("{0}")]
    ParseIntError(#[from] std::num::ParseIntError),
}

/// Displays the given result through the stdout if is [Result::Ok], or through the stderr
/// otherwise.
pub fn display_result<T, E>(result: std::result::Result<T, E>)
where
    T: Display + Sync + Send,
    E: Display + Sync + Send,
{
    match result {
        Ok(ok) => println!("{ok}"),
        Err(error) => eprintln!("{error}"),
    };
}