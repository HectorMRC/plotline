//! Interval search tree implementation.

use serde::{Deserialize, Deserializer, Serialize};
use std::cmp;

/// A Bound represents the limit of an [Interval].
pub trait Bound: Eq + Ord + Copy {}
impl<T> Bound for T where T: Eq + Ord + Copy {}

/// An Interval is anything delimited by two bounds.
pub trait Interval: Clone {
    type Bound: Bound;

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
pub struct Node<Intv>
where
    Intv: Interval,
{
    value: Intv,
    max: Intv::Bound,
    left: Option<Box<Node<Intv>>>,
    right: Option<Box<Node<Intv>>>,
}

impl<Intv> From<Intv> for Node<Intv>
where
    Intv: Interval,
{
    fn from(value: Intv) -> Self {
        Self::new(value)
    }
}

impl<Intv> Node<Intv>
where
    Intv: Interval,
{
    /// Creates a new node containing the given interval.
    pub fn new(interval: Intv) -> Self {
        Self {
            max: interval.hi(),
            value: interval,
            left: Default::default(),
            right: Default::default(),
        }
    }

    /// Inserts the given interval in the tree rooted by self.
    pub fn _with_interval(mut self, interval: Intv) -> Self {
        self._insert(interval);
        self
    }

    /// Adds the given interval in the tree rooted by self.
    pub fn _insert(&mut self, interval: Intv) {
        if self.max < interval.hi() {
            self.max = interval.hi();
        }

        if interval.lo() < self.value.lo() {
            if let Some(left) = &mut self.left {
                left._insert(interval)
            } else {
                self.left = Some(Box::new(interval.into()));
            }
        } else if let Some(right) = &mut self.right {
            right._insert(interval);
        } else {
            self.right = Some(Box::new(interval.into()))
        }
    }

    /// Returns true if, and only if, there is an interval in the tree that intersects the
    /// given one.
    pub fn _intersects(&self, interval: &Intv) -> bool {
        if self.value.intersects(interval) {
            return true;
        }

        let continue_right = || {
            self.right
                .as_ref()
                .is_some_and(|right| right._intersects(interval))
        };

        let Some(left) = &self.left else {
            return continue_right();
        };

        if left.max < interval.lo() {
            return continue_right();
        }

        left._intersects(interval)
    }

    /// Calls the given closure for each interval in the tree overlapping the given one.
    pub fn _for_each_intersection<F>(&self, interval: &Intv, mut f: F)
    where
        F: FnMut(&Intv),
    {
        fn immersion<Intv, F>(node: &Node<Intv>, interval: &Intv, f: &mut F)
        where
            Intv: Interval,
            F: FnMut(&Intv),
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

    /// Returns the total amount of intervals in the tree.
    pub fn _count(&self) -> usize {
        let mut count = 1;

        count += self
            .left
            .as_ref()
            .map(|left| left._count())
            .unwrap_or_default();

        count += self
            .right
            .as_ref()
            .map(|right| right._count())
            .unwrap_or_default();

        count
    }
}

/// An IntervalST represents an interval search tree that may be empty.
#[derive(Clone, PartialEq)]
pub struct IntervalST<Intv>(Option<Node<Intv>>)
where
    Intv: Interval;

impl<Intv> Serialize for IntervalST<Intv>
where
    Intv: Serialize + Interval,
{
    /// Serializes an [IntervalST] as a vector of intervals.
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.collect_seq(self.intervals())
    }
}

impl<'de, Intv> Deserialize<'de> for IntervalST<Intv>
where
    Intv: Deserialize<'de> + Interval,
{
    /// Deserializes a [IntervalST] from a vector of intervals.
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error;

        Vec::<Intv>::deserialize(deserializer)?
            .try_into()
            .map_err(Error::custom)
    }
}

impl<Intv> Default for IntervalST<Intv>
where
    Intv: Interval,
{
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<Intv> From<Vec<Intv>> for IntervalST<Intv>
where
    Intv: Interval,
{
    fn from(value: Vec<Intv>) -> Self {
        fn immersion<Intv>(mut intervals: Vec<Intv>) -> Option<Node<Intv>>
        where
            Intv: Interval,
        {
            if intervals.is_empty() {
                return Default::default();
            }

            let center = intervals.len() / 2;
            let mut root: Node<Intv> = intervals.remove(center).into();
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

        IntervalST::<Intv>(immersion(value))
    }
}

impl<Intv> IntervalST<Intv>
where
    Intv: Interval,
{
    /// Calls the given closure for each interval in the tree.
    pub fn for_each<F>(&self, mut f: F)
    where
        F: FnMut(&Intv),
    {
        fn immersion<Intv, F>(node: &Node<Intv>, f: &mut F)
        where
            Intv: Interval,
            F: FnMut(&Intv),
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
    pub fn intervals(&self) -> Vec<Intv> {
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
            self.0
        }

        fn hi(&self) -> Self::Bound {
            self.1
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
                    ._with_interval(IntervalMock(0, 4))
                    ._with_interval(IntervalMock(2, 6))
                    ._with_interval(IntervalMock(7, 9)),
                query: IntervalMock(1, 2),
                output: true,
            },
        ]
        .into_iter()
        .for_each(|test| {
            assert_eq!(
                test.tree._intersects(&test.query),
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
                    ._with_interval(IntervalMock(0, 2))
                    ._with_interval(IntervalMock(3, 3))
                    ._with_interval(IntervalMock(5, 9))
                    ._with_interval(IntervalMock(6, 6)),
                query: IntervalMock(3, 5),
                output: vec![IntervalMock(5, 6), IntervalMock(3, 3), IntervalMock(5, 9)],
            },
        ]
        .into_iter()
        .for_each(|test| {
            let mut intervals = Vec::default();
            test.tree
                ._for_each_intersection(&test.query, |interval| intervals.push(interval.clone()));

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
                        ._with_interval(IntervalMock(3, 3))
                        ._with_interval(IntervalMock(0, 0))
                        ._with_interval(IntervalMock(6, 6))
                        ._with_interval(IntervalMock(5, 9)),
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
