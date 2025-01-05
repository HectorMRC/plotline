//! Bounds and intervals implementing the [`Property`] trait.

use std::cmp::Ordering;

use crate::Interval;

/// Represents an arbitrary date of N components of type T that is encoded using the Little-endian format.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LittleEndianDate<T, const N: usize>([T; N]);

impl<T, const N: usize> PartialOrd for LittleEndianDate<T, N>
where
    T: Ord,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<T, const N: usize> Ord for LittleEndianDate<T, N>
where
    T: Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        for (this, other) in self.0.iter().zip(&other.0).rev() {
            let order = this.cmp(other);
            if !order.is_eq() {
                return order;
            }
        }

        Ordering::Equal
    }
}

impl<T, const N: usize> Interval for LittleEndianDate<T, N>
where
    T: Copy + Ord,
{
    type Bound = Self;

    fn lo(&self) -> Self::Bound {
        *self
    }

    fn hi(&self) -> Self::Bound {
        *self
    }
}

impl<T, const N: usize> From<BigEndianDate<T, N>> for LittleEndianDate<T, N> {
    fn from(mut date: BigEndianDate<T, N>) -> Self {
        date.0.reverse();
        Self(date.0)
    }
}

/// Represents an arbitrary date of N components of type T that is encoded using the Big-endian format.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BigEndianDate<T, const N: usize>([T; N]);

impl<T, const N: usize> PartialOrd for BigEndianDate<T, N>
where
    T: Ord,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<T, const N: usize> Ord for BigEndianDate<T, N>
where
    T: Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        for (this, other) in self.0.iter().zip(&other.0) {
            let order = this.cmp(other);
            if !order.is_eq() {
                return order;
            }
        }

        Ordering::Equal
    }
}

impl<T, const N: usize> Interval for BigEndianDate<T, N>
where
    T: Copy + Ord,
{
    type Bound = Self;

    fn lo(&self) -> Self::Bound {
        *self
    }

    fn hi(&self) -> Self::Bound {
        *self
    }
}

impl<T, const N: usize> From<LittleEndianDate<T, N>> for BigEndianDate<T, N> {
    fn from(mut date: LittleEndianDate<T, N>) -> Self {
        date.0.reverse();
        Self(date.0)
    }
}
