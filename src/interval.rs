//! Interval search tree implementation.

use serde::{Deserialize, Deserializer, Serialize};
use std::cmp;

/// An Interval is anything delimited by two bounds.
pub trait Interval: Clone {
    type Bound: Eq + Ord + Copy;

    /// Retrives the lowest bound in the interval.
    fn lo(&self) -> Self::Bound;

    /// Retrives the higher bound in the interval.
    fn hi(&self) -> Self::Bound;

    /// Returns true if, and only if, the given bound is in self.
    fn contains(&self, bound: Self::Bound) -> bool {
        self.lo() <= bound && bound <= self.hi()
    }

    /// Returns true if, and only if, self intersects other.
    fn intersects(&self, other: &Self) -> bool {
        (other.lo() < self.lo() && self.hi() < other.hi())
            || self.contains(other.lo())
            || self.contains(other.hi())
    }
}

/// A Node is the minimum unit of information in an interval search tree.
#[derive(Debug, Clone, PartialEq)]
pub struct Node<I>
where
    I: Interval,
{
    value: I,
    max: I::Bound,
    left: Option<Box<Node<I>>>,
    right: Option<Box<Node<I>>>,
}

impl<I> From<I> for Node<I>
where
    I: Interval,
{
    fn from(value: I) -> Self {
        Self::new(value)
    }
}

impl<I> Node<I>
where
    I: Interval,
{
    /// Creates a new node containing the given interval.
    pub fn new(interval: I) -> Self {
        Self {
            max: interval.hi(),
            value: interval,
            left: Default::default(),
            right: Default::default(),
        }
    }

    /// Inserts the given interval in the tree rooted by self.
    pub fn with_interval(mut self, interval: I) -> Self {
        self.insert(interval);
        self
    }

    /// Adds the given interval in the tree rooted by self.
    pub fn insert(&mut self, interval: I) {
        if self.max < interval.hi() {
            self.max = interval.hi();
        }

        if interval.lo() < self.value.lo() {
            if let Some(left) = &mut self.left {
                left.insert(interval)
            } else {
                self.left = Some(Box::new(interval.into()));
            }
        } else {
            if let Some(right) = &mut self.right {
                right.insert(interval);
            } else {
                self.right = Some(Box::new(interval.into()))
            }
        }
    }

    /// Returns true if, and only if, there is an interval in the tree that intersects the
    /// given one.
    pub fn intersects(&self, interval: &I) -> bool {
        if self.value.intersects(interval) {
            return true;
        }

        let continue_right = || {
            self.right
                .as_ref()
                .is_some_and(|right| right.intersects(interval))
        };

        let Some(left) = &self.left else {
            return continue_right();
        };

        if left.max < interval.lo() {
            return continue_right();
        }

        left.intersects(interval)
    }

    /// Calls the given closure for each interval in the tree overlapping the given one.
    pub fn for_each_intersection<F>(&self, interval: &I, mut f: F)
    where
        F: FnMut(&I),
    {
        fn immersion<I, F>(node: &Node<I>, interval: &I, f: &mut F)
        where
            I: Interval,
            F: FnMut(&I),
        {
            if let Some(right) = &node.right {
                immersion(right, interval, f);
            }

            if node.value.intersects(interval) {
                f(&node.value);
            }

            let Some(left) = &node.left else {
                return;
            };

            if left.max < interval.lo() {
                return;
            }

            immersion(left, interval, f);
        }

        immersion(self, interval, &mut f);
    }

    /// Removes the given interval from the tree rooted by self.
    pub fn remove(mut self, interval: &I) -> Option<Self> {
        todo!()
    }

    /// Returns the total amount of intervals in the tree.
    pub fn count(&self) -> usize {
        let mut count = 1;

        count += self
            .left
            .as_ref()
            .map(|left| left.count())
            .unwrap_or_default();

        count += self
            .right
            .as_ref()
            .map(|right| right.count())
            .unwrap_or_default();

        count
    }
}

/// An IntervalST represents an interval search tree that may be empty.
#[derive(Clone, PartialEq)]
pub struct IntervalST<I>(Option<Node<I>>)
where
    I: Interval;

impl<I> Serialize for IntervalST<I>
where
    I: Serialize + Interval,
{
    /// Serializes an [IntervalST] as a vector of intervals.
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.collect_seq(self.intervals())
    }
}

impl<'de, I> Deserialize<'de> for IntervalST<I>
where
    I: Deserialize<'de> + Interval,
{
    /// Deserializes a [IntervalST] from a vector of intervals.
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error;

        Vec::<I>::deserialize(deserializer)?
            .try_into()
            .map_err(Error::custom)
    }
}

impl<I> Default for IntervalST<I>
where
    I: Interval,
{
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<I> From<Vec<I>> for IntervalST<I>
where
    I: Interval,
{
    fn from(value: Vec<I>) -> Self {
        fn immersion<I>(mut intervals: Vec<I>) -> Option<Node<I>>
        where
            I: Interval,
        {
            if intervals.is_empty() {
                return Default::default();
            }

            let center = intervals.len() / 2;
            let mut root: Node<I> = intervals.remove(center).into();
            if !intervals.is_empty() {
                let left = intervals.drain(0..center).collect();
                root.left = immersion(left).map(Box::new);
                root.right = immersion(intervals).map(Box::new);
            }

            root.max = cmp::max(
                root.left.as_ref().map(|node| node.max),
                root.right.as_ref().map(|node| node.max),
            )
            .max(Some(root.value.hi()))
            .unwrap_or(root.value.hi());

            Some(root)
        }

        let mut tree = IntervalST::default();
        tree.0 = immersion(value);
        tree
    }
}

impl<I> IntervalST<I>
where
    I: Interval,
{
    /// Calls the given closure for each interval in the tree.
    pub fn for_each<F>(&self, mut f: F)
    where
        F: FnMut(&I),
    {
        fn immersion<I, F>(node: &Node<I>, f: &mut F)
        where
            I: Interval,
            F: FnMut(&I),
        {
            if let Some(left) = &node.left {
                immersion(left, f);
            };

            f(&node.value);

            if let Some(right) = &node.right {
                immersion(right, f);
            }
        }

        let Some(root) = &self.0 else {
            return;
        };

        immersion(root, &mut f);
    }

    /// Returns a vector with all the intervals in the tree rooted by self.
    pub fn intervals(&self) -> Vec<I> {
        let mut intervals = Vec::new();
        self.for_each(|interval| intervals.push(interval.clone()));
        intervals
    }
}

#[cfg(test)]
mod tests {
    use super::{Interval, IntervalST, Node};
    use std::fmt::Debug;

    #[derive(Debug, Clone, PartialEq)]
    struct IntervalMock(usize, usize);

    impl Interval for IntervalMock {
        type Bound = usize;

        fn lo(&self) -> Self::Bound {
            self.0.clone()
        }

        fn hi(&self) -> Self::Bound {
            self.1.clone()
        }
    }

    impl Debug for IntervalST<IntervalMock> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_tuple("IntervalST").field(&self.0).finish()
        }
    }

    #[test]
    fn intersects_with_tree() {
        struct Test<'a> {
            name: &'a str,
            tree: Node<IntervalMock>,
            query: IntervalMock,
            output: bool,
        }

        vec![
            Test {
                name: "no intersaction",
                tree: Node::new(IntervalMock(0, 2)),
                query: IntervalMock(3, 3),
                output: false,
            },
            Test {
                name: "left-hand intersaction",
                tree: Node::new(IntervalMock(0, 2)),
                query: IntervalMock(1, 3),
                output: true,
            },
            Test {
                name: "right-hand intersaction",
                tree: Node::new(IntervalMock(2, 4)),
                query: IntervalMock(0, 3),
                output: true,
            },
            Test {
                name: "superset intersaction",
                tree: Node::new(IntervalMock(0, 3)),
                query: IntervalMock(1, 2),
                output: true,
            },
            Test {
                name: "subset intersaction",
                tree: Node::new(IntervalMock(1, 2)),
                query: IntervalMock(0, 3),
                output: true,
            },
            Test {
                name: "complex tree",
                tree: Node::new(IntervalMock(5, 6))
                    .with_interval(IntervalMock(0, 4))
                    .with_interval(IntervalMock(2, 6))
                    .with_interval(IntervalMock(7, 9)),
                query: IntervalMock(1, 2),
                output: true,
            },
        ]
        .into_iter()
        .for_each(|test| {
            assert_eq!(
                test.tree.intersects(&test.query),
                test.output,
                "{}",
                test.name
            );
        });
    }

    #[test]
    fn for_each_intersection_in_tree() {
        struct Test<'a> {
            name: &'a str,
            tree: Node<IntervalMock>,
            query: IntervalMock,
            output: Vec<IntervalMock>,
        }

        vec![
            Test {
                name: "no intersactions",
                tree: Node::new(IntervalMock(1, 2)),
                query: IntervalMock(0, 0),
                output: Vec::default(),
            },
            Test {
                name: "multiple intersactions",
                tree: Node::new(IntervalMock(5, 6))
                    .with_interval(IntervalMock(0, 2))
                    .with_interval(IntervalMock(3, 3))
                    .with_interval(IntervalMock(5, 9))
                    .with_interval(IntervalMock(6, 6)),
                query: IntervalMock(3, 5),
                output: vec![IntervalMock(5, 6), IntervalMock(3, 3), IntervalMock(5, 9)],
            },
        ]
        .into_iter()
        .for_each(|test| {
            let mut intervals = Vec::default();
            test.tree
                .for_each_intersection(&test.query, |interval| intervals.push(interval.clone()));

            assert_eq!(intervals.len(), test.output.len(), "{}", test.name,);
            test.output
                .into_iter()
                .for_each(|interval| assert!(intervals.contains(&interval), "{}", test.name));
        })
    }

    #[test]
    fn interval_search_tree_from_vector() {
        struct Test<'a> {
            name: &'a str,
            input: Vec<IntervalMock>,
            output: IntervalST<IntervalMock>,
        }

        vec![
            Test {
                name: "node from empty vector must fail",
                input: Vec::new(),
                output: IntervalST::default(),
            },
            Test {
                name: "node from non empty vec must not fail",
                input: vec![
                    IntervalMock(0, 0),
                    IntervalMock(3, 3),
                    IntervalMock(5, 6),
                    IntervalMock(5, 9),
                    IntervalMock(6, 6),
                ],
                output: IntervalST(Some(
                    Node::new(IntervalMock(5, 6))
                        .with_interval(IntervalMock(3, 3))
                        .with_interval(IntervalMock(0, 0))
                        .with_interval(IntervalMock(6, 6))
                        .with_interval(IntervalMock(5, 9)),
                )),
            },
        ]
        .into_iter()
        .for_each(|test| {
            let tree: IntervalST<IntervalMock> = test.input.into();
            assert_eq!(tree, test.output, "{0}", test.name);
        });
    }
}
