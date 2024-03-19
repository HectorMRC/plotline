/// A Command represents an executable action with an associated result.
pub trait Command {
    type Result;

    fn execute(self) -> Self;
    fn result(self) -> Self::Result;
}
