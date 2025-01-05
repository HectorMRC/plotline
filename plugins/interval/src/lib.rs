//! An interval search tree.

mod features;
#[allow(unused_imports)]
pub use features::*;
mod node;
mod plugin;
mod tree;
pub use tree::IntervalSearchTree;

/// One of the limits in an [`Interval`].
#[allow(dead_code)]
pub trait Bound: Copy + Ord {}
impl<T> Bound for T where T: Copy + Ord {}

/// A type delimited by two bounds.
pub trait Interval {
    type Bound: Bound;

    /// Retrives the lowest bound in the interval.
    fn lo(&self) -> Self::Bound;

    /// Retrives the higher bound in the interval.
    fn hi(&self) -> Self::Bound;
}

trait IntervalExt: Interval {
    /// Returns true if, and only if, the given bound is in self.
    fn contains(&self, bound: Self::Bound) -> bool {
        self.lo() <= bound && bound <= self.hi()
    }

    /// Returns true if, and only if, self intersects other.
    fn intersects(&self, other: &Self) -> bool {
        self.contains(other.lo())
            || self.contains(other.hi())
            || other.contains(self.lo())
            || other.contains(self.hi())
    }
}

impl<T> IntervalExt for T where T: Interval {}

#[cfg(any(test, feature = "fixtures"))]
#[allow(unused_imports)]
#[allow(unused_macros)]
pub mod fixtures {
    use std::fmt::Debug;

    use super::{Bound, Interval};

    /// A mock implementation for the [`Interval`] trait.
    #[derive(Default, Clone, PartialOrd, Ord)]
    pub struct IntervalMock<Bound> {
        lo_fn: Option<fn() -> Bound>,
        hi_fn: Option<fn() -> Bound>,
    }

    impl<B: Bound + Debug> Debug for IntervalMock<B> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let mut debug = f.debug_struct("IntervalMock");

            if self.lo_fn.is_some() {
                debug.field("lo", &self.lo());
            }

            if self.hi_fn.is_some() {
                debug.field("hi", &self.hi());
            }

            debug.finish()
        }
    }

    impl<B: PartialEq> Eq for IntervalMock<B> {}
    impl<B: PartialEq> PartialEq for IntervalMock<B> {
        fn eq(&self, other: &Self) -> bool {
            if !(self.lo_fn.is_some() == other.lo_fn.is_some()
                && self.hi_fn.is_some() == other.hi_fn.is_some())
            {
                return false;
            }

            if let (Some(self_lo_fn), Some(other_lo_fn)) = (self.lo_fn, other.lo_fn) {
                if self_lo_fn() != other_lo_fn() {
                    return false;
                }
            }

            if let (Some(self_hi_fn), Some(other_hi_fn)) = (self.hi_fn, other.hi_fn) {
                if self_hi_fn() != other_hi_fn() {
                    return false;
                }
            }

            true
        }
    }

    impl<B: Bound> Interval for IntervalMock<B> {
        type Bound = B;

        fn lo(&self) -> Self::Bound {
            self.lo_fn.expect("lo method must be set")()
        }

        fn hi(&self) -> Self::Bound {
            self.hi_fn.expect("hi method must be set")()
        }
    }

    impl<Bound> IntervalMock<Bound> {
        pub fn with_lo_fn(mut self, f: fn() -> Bound) -> Self {
            self.lo_fn = Some(f);
            self
        }

        pub fn with_hi_fn(mut self, f: fn() -> Bound) -> Self {
            self.hi_fn = Some(f);
            self
        }
    }

    macro_rules! interval_mock {
        ($lo:tt, $hi:tt) => {
            IntervalMock::default()
                .with_lo_fn(|| $lo)
                .with_hi_fn(|| $hi)
        };
    }

    pub(crate) use interval_mock;
}
