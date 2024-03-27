use futures::{future, Future};
use prettytable::{format::consts::FORMAT_CLEAN, Table};
use std::fmt::Display;

/// Displays the given result through the stdout if is [Result::Ok], or through
/// the stderr otherwise.
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

/// Calls the given closure for each item in the given iterator and displays
/// the result through the stdout if is [Result::Ok], or through the stderr
/// otherwise.
pub async fn display_each_result<I, V, F, O, T, E>(iter: I, f: F)
where
    I: Iterator<Item = V>,
    F: Fn(V) -> O,
    O: Future<Output = std::result::Result<T, E>>,
    T: Display + Sync + Send,
    E: Display + Sync + Send,
{
    future::join_all(iter.map(|value| async {
        display_result(f(value).await);
    }))
    .await;
}

pub struct DisplaySingle<'a, T, F> {
    item: &'a T,
    once: F,
}

impl<'a, T, F> Display for DisplaySingle<'a, T, F>
where
    F: Fn(&mut Table, &T),
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut table = Table::new();
        table.set_format(*FORMAT_CLEAN);

        let once = &self.once;
        once(&mut table, self.item);

        table.fmt(f)
    }
}

impl<'a, T, F> DisplaySingle<'a, T, F>
where
    F: Fn(&mut Table, &T),
{
    pub fn new(item: &'a T, f: F) -> Self {
        Self { item, once: f }
    }

    pub fn show(self) {
        print!("{}", self);
    }
}

pub struct DisplayMany<'a, T, F> {
    items: &'a [T],
    headers: Vec<&'a str>,
    foreach: F,
}

impl<'a, T, F> Display for DisplayMany<'a, T, F>
where
    F: Fn(&mut Table, &T),
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut table = Table::new();
        table.set_format(*FORMAT_CLEAN);

        if !self.headers.is_empty() {
            table.add_row(self.headers.iter().into());
        }

        let foreach = &self.foreach;
        self.items.iter().for_each(|item| {
            foreach(&mut table, item);
        });

        table.fmt(f)
    }
}

impl<'a, T, F> DisplayMany<'a, T, F>
where
    F: Fn(&mut Table, &T),
{
    pub fn new(items: &'a [T], f: F) -> Self {
        Self {
            items,
            headers: Default::default(),
            foreach: f,
        }
    }

    pub fn with_headers(mut self, headers: Vec<&'a str>) -> Self {
        self.headers = headers;
        self
    }

    pub fn show(self) {
        print!("{}", self);
    }
}
