//! Hook representation .

use crate::id::Identify;

use super::{FromGraph, Graph};

/// A command in the context of a [`Graph`].
pub trait Command<T: Identify, Args> {
    /// Performs the command.
    fn execute(self, graph: &Graph<T>);
}

macro_rules! impl_command {
    ($($args:tt),*) => {
        impl<T, U, $($args),*> Command<T, ($($args,)*)> for U
        where
            T: Identify,
            U: Fn($($args),*),
            $($args: FromGraph<T>),*
        {
            fn execute(self, graph: &Graph<T>) {
                (self)($($args::from_graph(graph)),*);
            }
        }
    };
}

impl_command!(A);
impl_command!(A, B);
impl_command!(A, B, C);
impl_command!(A, B, C, D);
impl_command!(A, B, C, D, E);
impl_command!(A, B, C, D, E, F);
impl_command!(A, B, C, D, E, F, G);
impl_command!(A, B, C, D, E, F, G, H);

/// Converts self into a [`Command`].
pub trait IntoCommand<T: Identify, Args> {
    fn into_command(self) -> impl Command<T, Args>;
}

impl<T, U, Args> IntoCommand<T, Args> for U
where 
    T: Identify,
    U: Command<T, Args>,
{
    fn into_command(self) -> impl Command<T, Args> {
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        graph::{fixtures::NodeMock, Graph},
        id::Identify,
    };

    #[test]
    fn graph_handle_arbitrary_hooks() {
        struct Foo;

        impl<T: Identify> From<&Graph<T>> for Foo {
            fn from(_: &Graph<T>) -> Self {
                Foo
            }
        }

        struct Bar;

        impl<T: Identify> From<&Graph<T>> for Bar {
            fn from(_: &Graph<T>) -> Self {
                Bar
            }
        }

        fn one_hook(_: Foo) {}
        fn another_hook(_: Foo, _: Bar) {}

        Graph::<NodeMock<usize>>::default()
            .with_hook(one_hook)
            .with_hook(another_hook);
    }
}
