use crate::interval::Interval;
use serde::{Deserialize, Serialize};
use std::num::ParseIntError;

/// A Period is the time being between two different moments in time.
#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Period<M> {
    lo: M,
    hi: M,
}

impl<M> Interval for Period<M>
where
    M: Eq + Ord + Copy,
{
    type Bound = M;

    fn lo(&self) -> Self::Bound {
        self.lo
    }

    fn hi(&self) -> Self::Bound {
        self.hi
    }
}

impl TryFrom<Vec<String>> for Period<usize> {
    type Error = ParseIntError;

    fn try_from(bounds: Vec<String>) -> Result<Self, Self::Error> {
        let mut bounds = bounds.into_iter();
        let lo = bounds.next().unwrap_or_default().parse::<usize>()?;
        let hi = bounds
            .next()
            .map(|s| s.parse::<usize>())
            .transpose()?
            .unwrap_or(lo);

        Ok(Self { lo, hi })
    }
}
