//! Bounds and intervals implementing the [`Property`] trait.

use std::{cmp::Ordering, marker::PhantomData};

// use plotline::property::Extractor;

use crate::Interval;

/// Marks a [`Date`] as being encoded using the Little-endian format.
#[derive(Debug, Clone, Copy)]
pub struct LittleEndian;

/// Represents an arbitrary date of N components of type T encoded using the Big-endian format.
pub type LittleEndianDate<const N: usize, T> = Date<N, T, LittleEndian>;

/// Marks a [`Date`] as being encoded using the Big-endian format.
#[derive(Debug, Clone, Copy)]
pub struct BigEndian;

/// Represents an arbitrary date of N components of type T encoded using the Big-endian format.
pub type BigEndianDate<const N: usize, T> = Date<N, T, BigEndian>;

/// Represents an arbitrary date of N components of type T encoded using the specified endian.
#[derive(Debug, Clone, Copy)]
pub struct Date<const N: usize, T, Endian> {
    components: [T; N],
    endian: PhantomData<Endian>,
}

impl<const N: usize, T, Endian> Eq for Date<N, T, Endian> where T: Eq {}

impl<const N: usize, T, Endian> PartialEq for Date<N, T, Endian>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.components == other.components
    }
}

impl<const N: usize, T, Endian> PartialOrd for Date<N, T, Endian>
where
    Self: Ord,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<const N: usize, T, Endian> Interval for Date<N, T, Endian>
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

impl<const N: usize, T> Ord for Date<N, T, LittleEndian>
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

impl<const N: usize, T> Ord for Date<N, T, BigEndian>
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

// pub struct DateExtractor<const N: usize, T, Endian> {
//     separator: char,
//     component: PhantomData<T>,
//     endian: PhantomData<Endian>,
// }

// impl<const N: usize, Src, T, Endian> Extractor<Src> for DateExtractor<N, T, Endian> {
//     type Target = Date<N, T, Endian>;

//     fn all(&self, source: &Src) -> Vec<Self::Target> {
//         todo!()
//     }
// }