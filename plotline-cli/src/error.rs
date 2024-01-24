use std::{
    fmt::Display,
    io::Write,
    io::{stderr, stdout},
    sync::mpsc,
};

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

/// Calls the given closure for each item in the given iterator and displays the result through the
/// stdout if is [Result::Ok], or through the stderr otherwise.
pub fn display_each_result<I, F, T, E>(iter: I, f: F) -> std::result::Result<(), std::io::Error>
where
    I: Iterator,
    I::Item: Sync + Send,
    F: Fn(I::Item) -> std::result::Result<T, E> + Copy + Sync + Send,
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
