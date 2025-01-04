//! The interval search tree (IST) definition.

use crate::{node::IntervalSearchTreeNode, Interval};

/// An interval search tree.
pub struct IntervalSearchTree<Intv>
where
    Intv: Interval,
{
    root: Option<Box<IntervalSearchTreeNode<Intv>>>,
}

impl<Intv> Default for IntervalSearchTree<Intv>
where
    Intv: Interval,
{
    fn default() -> Self {
        Self {
            root: Default::default(),
        }
    }
}

impl<Intv> IntervalSearchTree<Intv>
where
    Intv: PartialEq + Interval,
{
    /// Deletes the given interval from the tree.
    pub fn delete(&mut self, id: &Intv) {
        if let Some(root) = self.root.take() {
            self.root = root.delete(id);
        }
    }
}

impl<Intv> IntervalSearchTree<Intv>
where
    Intv: Interval,
{
    /// Inserts the given interval in the tree.
    pub fn with_interval(mut self, interval: Intv) -> Self {
        self.insert(interval);
        self
    }

    /// Inserts the given interval in the tree.
    pub fn insert(&mut self, interval: Intv) {
        if let Some(root) = self.root.take() {
            self.root = Some(root.insert(interval));
            return;
        }

        self.root = Some(Box::new(IntervalSearchTreeNode::new(interval)));
    }

    /// Returns true if, and only if, there is an interval in the tree that intersects the given
    /// one.
    pub fn intersects(&self, interval: &Intv) -> bool {
        self.root
            .as_ref()
            .map(|root| root.intersects(interval))
            .unwrap_or_default()
    }

    /// Calls the given closure for each interval in the tree overlapping the given one.
    pub fn for_each_intersection<F>(&self, interval: &Intv, f: F)
    where
        F: FnMut(&Intv),
    {
        self.root
            .as_ref()
            .map(|root| root.for_each_intersection(interval, f));
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        fixtures::{interval_mock, IntervalMock},
        IntervalSearchTree,
    };

    #[test]
    fn intersects_with_tree() {
        struct Test<'a> {
            name: &'a str,
            tree: IntervalSearchTree<IntervalMock<usize>>,
            query: IntervalMock<usize>,
            intersects: bool,
        }

        vec![
            Test {
                name: "no intersaction",
                tree: IntervalSearchTree::default().with_interval(interval_mock!(0, 2)),
                query: interval_mock!(3, 3),
                intersects: false,
            },
            Test {
                name: "left-hand intersaction",
                tree: IntervalSearchTree::default().with_interval(interval_mock!(0, 2)),
                query: interval_mock!(1, 3),
                intersects: true,
            },
            Test {
                name: "right-hand intersaction",
                tree: IntervalSearchTree::default().with_interval(interval_mock!(2, 4)),
                query: interval_mock!(0, 3),
                intersects: true,
            },
            Test {
                name: "superset intersaction",
                tree: IntervalSearchTree::default().with_interval(interval_mock!(0, 3)),
                query: interval_mock!(1, 2),
                intersects: true,
            },
            Test {
                name: "subset intersaction",
                tree: IntervalSearchTree::default().with_interval(interval_mock!(1, 2)),
                query: interval_mock!(0, 3),
                intersects: true,
            },
            Test {
                name: "complex tree",
                tree: IntervalSearchTree::default()
                    .with_interval(interval_mock!(5, 6))
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
            tree: IntervalSearchTree<IntervalMock<usize>>,
            query: IntervalMock<usize>,
            output: Vec<IntervalMock<usize>>,
        }

        vec![
            Test {
                name: "no intersactions",
                tree: IntervalSearchTree::default().with_interval(interval_mock!(1, 2)),
                query: interval_mock!(0, 0),
                output: Vec::default(),
            },
            Test {
                name: "multiple intersactions",
                tree: IntervalSearchTree::default()
                    .with_interval(interval_mock!(5, 6))
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
