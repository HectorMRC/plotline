//! Centered interval tree implementation.

use std::{
    collections::BTreeMap,
    iter::Sum,
};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("empty vector")]
    EmptyVector,
}

/// An Interval is anything delimited by two bounds.
pub trait Interval: for<'b> Sum<&'b Self> {
    type Bound: Eq + Ord;

    /// Retrives the left bound of the interval.
    fn left(&self) -> Self::Bound;

    /// Retrives the middle point in the interval.
    fn middle(&self) -> Self::Bound;

    /// Retrives the right bound of the interval.
    fn right(&self) -> Self::Bound;
}

struct Center<I>
where
    I: Interval
{
    by_left: BTreeMap<I::Bound, I>,
    by_right: BTreeMap<I::Bound, I>
}

impl<I> Default for Center<I>
where
    I: Interval
{
    fn default() -> Self {
        Self {
            by_left: Default::default(),
            by_right: Default::default()
        }
    }
}

impl<I> Center<I>
where
    I: Interval + Clone
{
    fn insert(&mut self, interval: I) {
        self.by_left.insert(interval.left(), interval.clone());
        self.by_right.insert(interval.right(), interval);
    }

    /// Retrives all those intervals which leftmore bound is smaller than the given one.
    fn smaller_left(&self, bound: &I::Bound) -> Vec<I> {
        let mut intervals = Vec::default();
        for (left, value) in self.by_left.iter() {
            if left > &bound {
                break;
            }

            intervals.push(value.clone());
        }

        intervals
    }

    /// Retrives all those intervals which rightmore bound is greater than the given one.
    fn greater_right(&self, bound: &I::Bound) -> Vec<I> {
        let mut intervals = Vec::default();
        for (right, value) in self.by_right.iter().rev() {
            if right < &bound {
                break;
            }

            intervals.push(value.clone());
        }

        intervals
    }

    /// Retrives all the intervals.
    fn all(&self) -> Vec<I> {
        self.by_left.values().cloned().collect()
    }
}

/// A Node is the minimum unit of information in a centered interval tree.
pub struct Node<I>
where
    I: Interval,
{
    middle: I::Bound,
    left: Option<Box<Node<I>>>,
    right: Option<Box<Node<I>>>,
    center: Center<I>,
}

impl<I> TryFrom<Vec<I>> for Node<I>
where
    I: Interval + Clone,
{
    type Error = Error;

    fn try_from(value: Vec<I>) -> Result<Self> {
        if value.is_empty() {
            return Err(Error::EmptyVector);
        }

        let total: I = value.iter().sum();
        let middle = total.middle();

        let mut left = Vec::default();
        let mut right = Vec::default();
        let mut center = Center::default();

        for interval in value.into_iter() {
            if interval.right() < middle {
                left.push(interval);
            } else if interval.left() > middle {
                right.push(interval);
            } else {
                center.insert(interval);
            }
        }

        Ok(Self {
            middle,
            left: left.try_into().ok().map(Box::new),
            right: right.try_into().ok().map(Box::new),
            center,
        })
    }
}

impl<I> Node<I>
where
    I: Interval + Clone,
{
    /// Creates a new node as the root of a centered interval tree.
    pub fn new(interval: I) -> Self {
        let middle = interval.middle();

        let mut center = Center::default();
        center.insert(interval);

        Self {
            middle,
            left: Default::default(),
            right: Default::default(),
            center,
        }
    }

    /// Adds the given interval in the centered interval tree rooted by self.
    pub fn insert(&mut self, interval: I) {
        if interval.right() < self.middle {
            if let Some(left) = &mut self.left {
                left.insert(interval);
            } else {
                self.left = Some(Box::new(Self::new(interval)))
            }
        } else if interval.left() > self.middle {
            if let Some(right) = &mut self.right {
                right.insert(interval);
            } else {
                self.right = Some(Box::new(Self::new(interval)));
            }
        } else {
            self.center.insert(interval);
        }
    }

    fn intersecting_bound(&self, bound: &I::Bound, intervals: &mut Vec<I>) {
        if bound < &self.middle {
            intervals.append(&mut self.center.smaller_left(bound));
            if let Some(left) = &self.left {
                left.intersecting_bound(bound, intervals);
            }
        } else if bound > &self.middle {
            intervals.append(&mut self.center.greater_right(bound));
            if let Some(right) = &self.right {
                right.intersecting_bound(bound, intervals);
            }
        } else {
            intervals.append(&mut self.center.all())
        }
    }

    fn intersecting_interval(&self, interval: &I, intervals: &mut Vec<I>) {
        if (interval.left() <= self.middle && interval.right() >= self.middle) {
            intervals.append(&mut self.center.all());
        }

        if interval.left() > self.middle {
            if let Some(right) = &self.right {
                right.intersecting_interval(interval, intervals);
            }
        }
        
        if interval.right() < self.middle {
            if let Some(left) = &self.left {
                left.intersecting_interval(interval, intervals);
            }
        }
    }

    /// Returns all the intervals in the tree overlapping the given one.
    pub fn intersecting(&self, interval: &I) -> Vec<I> {
        let mut intervals = Vec::default();
        
        if interval.left() == interval.right() {
            self.intersecting_bound(&interval.left(), &mut intervals);
        } else {
            self.intersecting_interval(interval, &mut intervals);
        }
        
        intervals
    }
}

#[cfg(test)]
mod tests {
    use std::{ops::Add, iter::Sum};
    use super::{Node, Interval, Error};

    #[derive(Clone, Default, PartialEq)]
    struct IntervalMock (usize, usize);

    impl<'a> Add<&'a IntervalMock> for IntervalMock {
        type Output = IntervalMock;

        fn add(self, rhs: &'a IntervalMock) -> Self::Output {
            IntervalMock(self.0.min(rhs.0), self.1.max(rhs.1))
        }
    }

    impl<'a> Sum<&'a IntervalMock> for IntervalMock {
        fn sum<I: Iterator<Item = &'a IntervalMock>>(mut iter: I) -> Self {
            let Some(mut total) = iter.next().cloned() else {
                return Default::default();
            };

            for interval in iter {
                total = total + interval;
            }

            total
        }
    }

    impl Interval for IntervalMock {
        type Bound = usize;

        fn left(&self) -> Self::Bound {
            self.0
        }

        fn middle(&self) -> Self::Bound {
            (self.left() + self.right()) / 2
        }

        fn right(&self) -> Self::Bound {
            self.1
        }
    }

    #[test]
    fn centered_interval_tree_from_vec() {
        let empty_vec: Vec<IntervalMock> = Vec::default();
        let err = Node::<IntervalMock>::try_from(empty_vec).err().expect("node from emtpy vector must fail");
        assert!(matches!(err, Error::EmptyVector), "node from empty vector must return EmptyVector error");

        Node::<IntervalMock>::try_from(vec![
            IntervalMock(0, 1),
        ]).expect("node from non-empty vector should not fail");
    }

    #[test]
    fn intersecting_intervals() {
        struct Test {
            intervals: Vec<IntervalMock>,
            query: IntervalMock,
            result: Vec<IntervalMock>
        }

        vec![
            Test{
                intervals: vec![
                    IntervalMock(0, 1),
                    IntervalMock(2, 5),
                    IntervalMock(4, 6),
                    IntervalMock(6, 9),
                ],
                query: IntervalMock(4, 4),
                result: vec![
                    IntervalMock(2, 5),
                    IntervalMock(4, 6)
                ]
            },
            Test{
                intervals: vec![
                    IntervalMock(0, 3),
                    IntervalMock(5, 9),
                ],
                query: IntervalMock(4, 4),
                result: Vec::default()
            },
            Test{
                intervals: vec![
                    IntervalMock(0, 3),
                    IntervalMock(5, 9),
                    IntervalMock(2, 8),
                    IntervalMock(1, 4),
                    IntervalMock(6, 7),
                ],
                query: IntervalMock(3, 5),
                result: vec![
                    IntervalMock(0, 3),
                    IntervalMock(5, 9),
                    IntervalMock(2, 8),
                    IntervalMock(1, 4),
                ],
            },
        ].into_iter().for_each(|test| {
            let interval_tree: Node<IntervalMock> = test.intervals.try_into().unwrap();
            let intersecting = interval_tree.intersecting(&test.query);
            assert_eq!(intersecting.len(), test.result.len());

            test.result.into_iter().for_each(|want| {
                assert!(intersecting.contains(&want));
            })
        });
    }
}