//! Bounds and intervals implementing the [`Property`] trait.

use std::{cmp::Ordering, marker::PhantomData};

use crate::Interval;

/// Marks a [`Date`] as being encoded using the Little-endian format.
#[derive(Debug, Clone, Copy)]
pub struct LittleEndian;

/// Represents an arbitrary date of N components of type T encoded using the Big-endian format.
pub type LittleEndianDate<T, const N: usize> = Date<T, N, LittleEndian>;

/// Marks a [`Date`] as being encoded using the Big-endian format.
#[derive(Debug, Clone, Copy)]
pub struct BigEndian;

/// Represents an arbitrary date of N components of type T encoded using the Big-endian format.
pub type BigEndianDate<T, const N: usize> = Date<T, N, BigEndian>;

/// Represents an arbitrary date of N components of type T encoded using the specified endian.
#[derive(Debug, Clone, Copy)]
pub struct Date<T, const N: usize, Endian> {
    components: [T; N],
    endian: PhantomData<Endian>,
}

impl<T, const N: usize, Endian> Eq for Date<T, N, Endian> where T: Eq {}

impl<T, const N: usize, Endian> PartialEq for Date<T, N, Endian>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.components == other.components
    }
}

impl<T, const N: usize, Endian> PartialOrd for Date<T, N, Endian>
where
    Self: Ord,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<T, const N: usize, Endian> Interval for Date<T, N, Endian>
where
    Self: Copy + Ord,
{
    type Bound = Self;

    fn lo(&self) -> Self::Bound {
        *self
    }

    fn hi(&self) -> Self::Bound {
        *self
    }
}

impl<T, const N: usize> Ord for Date<T, N, LittleEndian>
where
    T: Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        for (this, other) in self.components.iter().zip(&other.components).rev() {
            let order = this.cmp(other);
            if !order.is_eq() {
                return order;
            }
        }

        Ordering::Equal
    }
}

impl<T, const N: usize> Ord for Date<T, N, BigEndian>
where
    T: Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        for (this, other) in self.components.iter().zip(&other.components) {
            let order = this.cmp(other);
            if !order.is_eq() {
                return order;
            }
        }

        Ordering::Equal
    }
}
