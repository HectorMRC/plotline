//! Context-based command definitions.

/// An entity that can be executed under a specific context.
pub trait Command<Ctx, Args = ()> {
    /// Performs the command.
    fn execute(self, ctx: &Ctx);
}

macro_rules! impl_command {
    ($($args:tt),*) => {
        impl<Ctx, U, $($args),*> Command<Ctx, ($($args,)*)> for U
        where
            U: Fn($($args),*),
            $($args: for<'a> From<&'a Ctx>),*
        {
            fn execute(self, ctx: &Ctx) {
                (self)($($args::from(ctx)),*);
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

#[cfg(test)]
mod tests {
    use crate::command::Command;

    #[test]
    fn handle_arbitrary_commands() {
        struct Handler;
        impl Handler {
            fn with_command<M>(self, _: impl Command<Self, M>) -> Self {
                self
            }
        }

        struct Foo;
        impl From<&Handler> for Foo {
            fn from(_: &Handler) -> Self {
                Foo
            }
        }

        struct Bar;
        impl From<&Handler> for Bar {
            fn from(_: &Handler) -> Self {
                Bar
            }
        }

        fn one_command(_: Foo) {}
        fn another_command(_: Foo, _: Bar) {}
        fn _not_a_command(_: Foo, _: Bar, _: usize) {}

        Handler
            .with_command(one_command)
            // .with_command(_not_a_command)
            .with_command(another_command);
    }
}
