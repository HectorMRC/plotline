use std::{
    fmt::Display,
    io::Write,
    io::{stderr, stdout},
    sync::mpsc,
};

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
    Name(#[from] crate::name::Error),
    #[error("{0}")]
    Id(#[from] crate::id::Error),
    #[error("{0}")]
    Event(#[from] crate::event::Error),
    #[error("{0}")]
    Io(#[from] std::io::Error),
    #[error("{0}")]
    ParseIntError(#[from] std::num::ParseIntError),
}

/// Displays the given result through the stdout if is [Result::Ok], or through the stderr
/// otherwise.
pub fn display_result<T, E>(result: Result<T, E>) -> Result<(), std::io::Error>
where
    T: Display + Sync + Send,
    E: Display + Sync + Send,
{
    let mut stdout = stdout().lock();
    let mut stderr = stderr().lock();
    match result {
        Ok(ok) => writeln!(stdout, "{ok}")?,
        Err(error) => writeln!(stderr, "{error}")?,
    }

    Ok(())
}


/// Calls the given closure for each item in the given iterator and displays the result through the
/// stdout if is [Result::Ok], or through the stderr otherwise.
pub fn display_each_result<I, F, T, E>(iter: I, f: F) -> Result<(), std::io::Error>
where
    I: Iterator,
    I::Item: Sync + Send,
    F: Fn(I::Item) -> Result<T, E> + Copy + Sync + Send,
    T: Display + Sync + Send,
    E: Display + Sync + Send,
{
    let receiver = std::thread::scope(|scope| {
        let (sender, receiver) = mpsc::channel();
        iter.for_each(|item| {
            let sender = sender.clone();
            scope.spawn(move || sender.send(f(item)));
        });

        receiver
    });

    let mut stdout = stdout().lock();
    let mut stderr = stderr().lock();
    while let Ok(result) = receiver.recv() {
        match result {
            Ok(ok) => writeln!(stdout, "{ok}")?,
            Err(error) => writeln!(stderr, "{error}")?,
        }
    }

    Ok(())
}
