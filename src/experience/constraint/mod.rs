use super::Result;

pub trait Constraint {
    fn result(self) -> Result<()>;
}
