mod search_tree;

/// Represents one of the limits in an [Interval].
trait Bound: Eq + Ord + Clone {}
impl<T> Bound for T where T: Eq + Ord + Clone {}

/// Represents whatever delimited by two bounds.
trait Interval: Eq + Ord {
    type Bound: Bound;

    /// Retrives the lowest bound in the interval.
    fn lo(&self) -> &Self::Bound;

    /// Retrives the higher bound in the interval.
    fn hi(&self) -> &Self::Bound;

    /// Returns true if, and only if, the given bound is in self.
    fn contains(&self, bound: &Self::Bound) -> bool {
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

#[cfg(any(test, feature = "fixtures"))]
pub mod fixtures {
    use super::Interval;

    /// Implements the [Interval] trait for a pair of [usize]s.
    #[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
    pub struct FakeInterval([usize; 2]);

    impl Interval for FakeInterval {
        type Bound = usize;

        fn lo(&self) -> &Self::Bound {
            &self.0[0]
        }

        fn hi(&self) -> &Self::Bound {
            &self.0[1]
        }
    }

    impl From<[usize; 2]> for FakeInterval {
        fn from(value: [usize; 2]) -> Self {
            Self(value)
        }
    }
}
