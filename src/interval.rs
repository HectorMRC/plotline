//! Centered interval tree implementation.

use std::{
    collections::HashMap,
    hash::Hash,
    iter::Sum,
    ops::{Add, Div},
    sync::Arc,
};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("empty vector")]
    EmptyVector,
}

/// An Interval is anything delimited by two bounds.
pub trait Interval: for<'a> Add<&'a Self, Output = Self> {
    type Bound: Hash
        + PartialEq
        + Eq
        + PartialOrd
        + Add<Output = Self::Bound>
        + Div<usize, Output = Self::Bound>;

    /// Retrives the left-most bound of the interval.
    fn beginning(&self) -> Self::Bound;

    /// Retrives the right-most bound of the interval.
    fn ending(&self) -> Self::Bound;
}

#[derive(Debug)]
struct Center<I>
where
    I: Interval,
{
    beginning: HashMap<I::Bound, I>,
    ending: HashMap<I::Bound, I>,
}

impl<I> Default for Center<I>
where
    I: Interval,
{
    fn default() -> Self {
        Self {
            beginning: Default::default(),
            ending: Default::default(),
        }
    }
}

impl<I> Center<I>
where
    I: Interval + Clone,
{
    fn insert(&mut self, value: I) {
        self.beginning.insert(value.beginning(), value.clone());
        self.ending.insert(value.ending(), value);
    }
}

pub struct Node<I>
where
    I: Interval,
{
    middle: I::Bound,
    bounds: Vec<Option<Node<I>>>,
    center: Center<I>,
}

impl<I> TryFrom<Vec<I>> for Node<I>
where
    I: Interval + Clone + for<'b> Sum<&'b I>,
{
    type Error = Error;

    fn try_from(value: Vec<I>) -> Result<Self> {
        if value.is_empty() {
            return Err(Error::EmptyVector);
        }

        let total: I = value.iter().sum();
        let middle: I::Bound = (total.beginning() + total.ending()) / 2;

        let mut left = Vec::default();
        let mut right = Vec::default();
        let mut center = Center::default();

        for interval in value.into_iter() {
            if interval.ending() < middle {
                left.push(interval);
            } else if interval.beginning() > middle {
                right.push(interval);
            } else {
                center.insert(interval);
            }
        }

        Ok(Self {
            middle,
            bounds: vec![left.try_into().ok(), right.try_into().ok()],
            center,
        })
    }
}

impl<I> Node<I>
where
    I: Interval + Clone + for<'a> Sum<&'a I>,
{
    pub fn new(interval: I) -> Self {
        let middle = (interval.beginning() + interval.ending()) / 2;

        let mut center = Center::default();
        center.insert(interval);

        Self {
            middle,
            bounds: Default::default(),
            center,
        }
    }

    pub fn insert(&mut self, interval: I) {
        if interval.ending() < self.middle {
            if let Some(left) = &mut self.bounds[0] {
                left.insert(interval);
            } else {
                self.bounds[0] = Some(Self::new(interval))
            }
        } else if interval.beginning() > self.middle {
            if let Some(right) = &mut self.bounds[1] {
                right.insert(interval);
            } else {
                self.bounds[1] = Some(Self::new(interval));
            }
        } else {
            self.center.insert(interval);
        }
    }
}
