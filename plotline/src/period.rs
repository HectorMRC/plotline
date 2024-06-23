use crate::{
    interval::{Bound, Interval, IntervalFactory},
    macros,
};
use serde::{Deserialize, Serialize};

/// A Period is the time being between two different moments in time.
#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Period<M> {
    lo: M,
    hi: M,
}

impl<M> Interval for Period<M>
where
    M: Bound,
{
    type Bound = M;

    fn lo(&self) -> &Self::Bound {
        &self.lo
    }

    fn hi(&self) -> &Self::Bound {
        &self.hi
    }
}

impl<M> IntervalFactory for Period<M>
where
    M: Bound + Clone,
{
    type Bound = <Self as Interval>::Bound;

    // fn new(mut lo: Self::Bound, mut hi: Self::Bound) -> Self {
    //     if lo > hi {
    //         std::mem::swap(&mut lo, &mut hi);
    //     }

    //     Self { lo, hi }
    // }
}

macros::interval_based_ord_for!(Period<M> where M: Bound);

#[cfg(any(test, feature = "fixtures"))]
mod tests {
    use super::Period;
    use crate::moment::Moment;

    impl From<[usize; 2]> for Period<Moment> {
        fn from(bounds: [usize; 2]) -> Self {
            Self {
                lo: Moment::new(bounds[0]),
                hi: Moment::new(bounds[1]),
            }
        }
    }
}
