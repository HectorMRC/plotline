//! Interval search tree implementation.

/// An Interval is anything delimited by two bounds.
pub trait Interval {
    type Bound: Eq + Ord;

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
            if node.value.intersects(interval) {
                f(&node.value);
            }

            if let Some(right) = &node.right {
                immersion(right, interval, f);
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
}

#[cfg(test)]
mod tests {
    use super::{Interval, Node};

    #[derive(Clone, Default, PartialEq)]
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
}
