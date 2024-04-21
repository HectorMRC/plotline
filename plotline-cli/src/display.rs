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

/// Provides a method to display the inner data into a table.
pub struct DisplayTable<'a, T> {
    item: &'a T,
}

impl<'a, T> DisplayTable<'a, T> {
    pub fn new(item: &'a T) -> Self {
        Self { item }
    }

    /// Displays through stdout the table resulting from calling the given
    /// closure.
    pub fn show<F>(self, f: F)
    where
        F: Fn(&mut Table, &T),
    {
        let mut table = Table::new();
        table.set_format(*FORMAT_CLEAN);

        f(&mut table, self.item);
        print!("{}", table)
    }
}

/// Provides a method to display the inner data into a tree.
pub struct DisplayTree<'a, T> {
    chunks: Vec<&'a [T]>,
    // chunk_name_fn: Option<ChunkNameFn>,
}

impl<'a, T> DisplayTree<'a, T>
where
    T: Eq,
{
    pub fn new(items: &'a [T]) -> Self {
        Self {
            chunks: items.chunk_by(T::eq).collect(),
        }
    }
}

impl<'a, T> Display for DisplayTree<'a, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}

impl<'a, T> DisplayTree<'a, T> {
    /// Displays the tree through stdout.
    pub fn show<F>(self) {
        print!("{}", self)
    }
}

#[cfg(test)]
mod tests {
    use crate::display::DisplayTree;

    #[test]
    fn display_tree() {
        struct Test<'a> {
            name: &'a str,
            items: Vec<&'a str>,
            want: &'a str,
        }

        vec![
            Test {
                name: "empty list should display nothing",
                items: vec![],
                want: "",
            },
            Test {
                name: "list with only one element should display a single bullet",
                items: vec!["this is a 'bullet' in the tree"],
                want: &format!("\u{25EF}    this is a 'bullet' in the tree"),
            },
        ]
        .into_iter()
        .for_each(|test| {
            let got = DisplayTree::new(test.items.as_slice()).to_string();
            assert_eq!(test.want, got, "{}", test.name);
        });
    }
}
