use super::Interval;

/// Represents a node in an interval search tree.
#[derive(Debug, Clone, PartialEq)]
struct IntervalSearchTreeNode<Intv>
where
    Intv: Interval,
{
    value: Intv,
    max: Intv::Bound,
    left: Option<Box<IntervalSearchTreeNode<Intv>>>,
    right: Option<Box<IntervalSearchTreeNode<Intv>>>,
}

impl<Intv> From<Intv> for IntervalSearchTreeNode<Intv>
where
    Intv: Interval,
{
    fn from(value: Intv) -> Self {
        Self::new(value)
    }
}

impl<Intv> IntervalSearchTreeNode<Intv>
where
    Intv: Interval,
{
    /// Creates a new node containing the given interval.
    fn new(interval: Intv) -> Self {
        Self {
            max: interval.hi(),
            value: interval,
            left: Default::default(),
            right: Default::default(),
        }
    }

    /// Inserts the given interval in the tree rooted by self.
    fn with_interval(mut self, interval: Intv) -> Self {
        self.insert(interval);
        self
    }

    /// Adds the given interval in the tree rooted by self.
    fn insert(&mut self, interval: Intv) {
        if self.max < interval.hi() {
            self.max = interval.hi();
        }

        if interval.lo() < self.value.lo() {
            if let Some(left) = &mut self.left {
                left.insert(interval)
            } else {
                self.left = Some(Box::new(interval.into()));
            }
        } else if let Some(right) = &mut self.right {
            right.insert(interval);
        } else {
            self.right = Some(Box::new(interval.into()))
        }
    }

    /// Returns true if, and only if, there is an interval in the tree that intersects the given
    /// one.
    fn intersects(&self, interval: &Intv) -> bool {
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
    fn for_each_intersection<F>(&self, interval: &Intv, mut f: F)
    where
        F: FnMut(&Intv),
    {
        fn immersion<Intv, F>(node: &IntervalSearchTreeNode<Intv>, interval: &Intv, f: &mut F)
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
    fn count(&self) -> usize {
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

#[cfg(test)]
mod tests {
    use crate::interval::{
        fixtures::{interval_mock, IntervalMock},
        search_tree::IntervalSearchTreeNode,
    };

    #[test]
    fn intersects_with_tree() {
        struct Test<'a> {
            name: &'a str,
            tree: IntervalSearchTreeNode<IntervalMock<usize>>,
            query: IntervalMock<usize>,
            intersects: bool,
        }

        vec![
            Test {
                name: "no intersaction",
                tree: IntervalSearchTreeNode::new(interval_mock!(0, 2)),
                query: interval_mock!(3, 3),
                intersects: false,
            },
            Test {
                name: "left-hand intersaction",
                tree: IntervalSearchTreeNode::new(interval_mock!(0, 2)),
                query: interval_mock!(1, 3),
                intersects: true,
            },
            Test {
                name: "right-hand intersaction",
                tree: IntervalSearchTreeNode::new(interval_mock!(2, 4)),
                query: interval_mock!(0, 3),
                intersects: true,
            },
            Test {
                name: "superset intersaction",
                tree: IntervalSearchTreeNode::new(interval_mock!(0, 3)),
                query: interval_mock!(1, 2),
                intersects: true,
            },
            Test {
                name: "subset intersaction",
                tree: IntervalSearchTreeNode::new(interval_mock!(1, 2)),
                query: interval_mock!(0, 3),
                intersects: true,
            },
            Test {
                name: "complex tree",
                tree: IntervalSearchTreeNode::new(interval_mock!(5, 6))
                    .with_interval(interval_mock!(0, 4))
                    .with_interval(interval_mock!(2, 6))
                    .with_interval(interval_mock!(7, 9)),
                query: interval_mock!(1, 2),
                intersects: true,
            },
        ]
        .into_iter()
        .for_each(|test| {
            assert_eq!(
                test.tree.intersects(&test.query),
                test.intersects,
                "{}",
                test.name
            );
        });
    }

    #[test]
    fn for_each_intersection_in_tree() {
        struct Test<'a> {
            name: &'a str,
            tree: IntervalSearchTreeNode<IntervalMock<usize>>,
            query: IntervalMock<usize>,
            output: Vec<IntervalMock<usize>>,
        }

        vec![
            Test {
                name: "no intersactions",
                tree: IntervalSearchTreeNode::new(interval_mock!(1, 2)),
                query: interval_mock!(0, 0),
                output: Vec::default(),
            },
            Test {
                name: "multiple intersactions",
                tree: IntervalSearchTreeNode::new(interval_mock!(5, 6))
                    .with_interval(interval_mock!(0, 2))
                    .with_interval(interval_mock!(3, 3))
                    .with_interval(interval_mock!(5, 9))
                    .with_interval(interval_mock!(6, 6)),
                query: interval_mock!(3, 5),
                output: vec![
                    interval_mock!(5, 6),
                    interval_mock!(3, 3),
                    interval_mock!(5, 9),
                ],
            },
        ]
        .into_iter()
        .for_each(|test| {
            let mut intervals = Vec::default();
            test.tree
                .for_each_intersection(&test.query, |interval| intervals.push(interval.clone()));

            assert_eq!(
                intervals.len(),
                test.output.len(),
                "{}: got intersection = {:?}, want = {:?}",
                test.name,
                intervals,
                test.output
            );

            test.output.iter().for_each(|interval| {
                assert!(
                    intervals.contains(interval),
                    "{}: interval = {:?} does not exists in {:?}",
                    test.name,
                    interval,
                    intervals
                )
            });
        })
    }
}
