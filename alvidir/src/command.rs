//! Context-based command definitions.

use std::convert::Infallible;

/// An entity that can be executed under a specific context.
pub trait Command<Ctx, Args> {
    type Err;

    /// Performs the command.
    fn execute(self, ctx: &Ctx) -> Result<(), Self::Err>;
}

/// An entity that can be executed by reference under a specific context.
pub trait CommandRef<Ctx, Args> {
    type Err;

    /// Performs the command.
    fn execute(&self, ctx: &Ctx) -> Result<(), Self::Err>;
}

/// A [`Command`] implementation that does nothing.
#[derive(Debug, Default)]
pub struct NoopCommand;

impl<Ctx> Command<Ctx, ()> for NoopCommand {
    type Err = Infallible;

    fn execute(self, _: &Ctx) -> Result<(), Self::Err> {
        Ok(())
    }
}

macro_rules! impl_command {
    ($($args:tt),*) => {
        impl<T, Ctx, Err, $($args),*> CommandRef<Ctx, (Err, $($args,)*)> for T
        where
            T: Fn($($args),*) -> Result<(), Err>,
            $($args: for<'a> From<&'a Ctx>),*
        {
            type Err = Err;

            fn execute(&self, _ctx: &Ctx) -> Result<(), Self::Err> {
                (self)($($args::from(_ctx)),*)
            }
        }
    };
}

impl_command!();
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
    use std::convert::Infallible;

    use crate::command::CommandRef;

    #[test]
    fn handle_arbitrary_commands() {
        struct Handler;
        impl Handler {
            fn with_command<M>(self, _: impl CommandRef<Self, M>) -> Self {
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

        fn one_command(_: Foo) -> Result<(), Infallible> {
            Ok(())
        }

        fn another_command(_: Foo, _: Bar) -> Result<(), Infallible> {
            Ok(())
        }

        fn argless_command() -> Result<(), Infallible> {
            Ok(())
        }

        // Not a command because From<&Handler> is not implemented for usize.
        fn _not_a_command(_: Foo, _: Bar, _: usize) -> Result<(), Infallible> {
            Ok(())
        }

        Handler
            .with_command(one_command)
            // .with_command(_not_a_command)
            .with_command(another_command)
            .with_command(argless_command);
    }
}
