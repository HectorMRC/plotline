//! The node from an interval search tree.
#![allow(dead_code)]

use super::{Interval, IntervalExt};

/// A node in an interval search tree.
#[derive(Debug, Clone, PartialEq)]
pub struct IntervalSearchTreeNode<Intv>
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
    Intv::Bound: Clone,
{
    /// Rotetes the tree rooted by self rightwards.
    fn rotate_right(mut self: Box<Self>) -> Box<Self> {
        let Some(mut root) = self.left.take() else {
            return self;
        };

        self.left = root.right.take();
        self.max = Some(self.value.hi())
            .max(self.left.as_ref().map(|left| left.max))
            .max(self.right.as_ref().map(|right| right.max))
            .expect("max with Some should never be None");

        root.right = Some(self);
        root.max = Some(root.value.hi())
            .max(root.left.as_ref().map(|left| left.max))
            .max(root.right.as_ref().map(|right| right.max))
            .expect("max with Some should never be None");

        root
    }

    /// Rotetes the tree rooted by self leftwards.
    fn rotate_left(mut self: Box<Self>) -> Box<Self> {
        let Some(mut root) = self.right.take() else {
            return self;
        };

        self.right = root.left.take();
        self.max = Some(self.value.hi())
            .max(self.left.as_ref().map(|left| left.max))
            .max(self.right.as_ref().map(|right| right.max))
            .expect("max with Some should never be None");

        root.left = Some(self);
        root.max = Some(root.value.hi())
            .max(root.left.as_ref().map(|left| left.max))
            .max(root.right.as_ref().map(|right| right.max))
            .expect("max with Some should never be None");

        root
    }
}

impl<Intv> IntervalSearchTreeNode<Intv>
where
    Intv: PartialEq + Interval,
{
    /// Deletes the given interval from the tree rooted by self.
    pub fn delete(mut self: Box<Self>, interval: &Intv) -> Option<Box<Self>> {
        if &self.value == interval {
            return match (self.left, self.right) {
                (Some(left), Some(right)) => Some(left.join(right)),
                (left, _) if left.is_some() => left,
                (_, right) if right.is_some() => right,
                _ => None,
            };
        }

        if interval.lo() < self.value.lo() {
            self.left = self.left.map(|left| left.delete(interval)).flatten();
        } else if interval.lo() > self.value.lo() {
            self.right = self.right.map(|right| right.delete(interval)).flatten();
        }

        Some(self)
    }
}

impl<Intv> IntervalSearchTreeNode<Intv>
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

    /// Returns true if, and only if, there is an interval in the tree that intersects the given
    /// one.
    pub fn intersects(&self, interval: &Intv) -> bool {
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
    pub fn for_each_intersection<F>(&self, interval: &Intv, mut f: F)
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

    /// Inserts the given interval in the tree rooted by self.
    pub fn insert(mut self: Box<Self>, interval: Intv) -> Box<Self> {
        if self.max < interval.hi() {
            self.max = interval.hi();
        }

        if interval.lo() < self.value.lo() {
            if let Some(left) = self.left.take() {
                self.left = Some(left.insert(interval));
            } else {
                self.left = Some(Box::new(interval.into()));
            }
        } else if let Some(right) = self.right.take() {
            self.right = Some(right.insert(interval));
        } else {
            self.right = Some(Box::new(interval.into()));
        }

        self
    }

    /// Given the root of a left (self) and right trees, joins them into a single one.
    fn join(self: Box<Self>, right: Box<Self>) -> Box<Self> {
        fn immersion<Intv>(
            root: Box<IntervalSearchTreeNode<Intv>>,
            mut intervals: Vec<Intv>,
        ) -> Box<IntervalSearchTreeNode<Intv>>
        where
            Intv: Interval,
        {
            if intervals.is_empty() {
                return root;
            }

            let center = intervals.len() / 2;
            let left = intervals.drain(0..center).collect();
            immersion(immersion(root, left), intervals)
        }

        let mut intervals = self.into_inorder();
        intervals.extend(right.into_inorder());

        let center = intervals.len() / 2;
        let root = Self::new(intervals.remove(center));

        immersion(Box::new(root), intervals)
    }

    /// Returns a vector with all the intervals in order.
    fn into_inorder(self: Box<Self>) -> Vec<Intv> {
        fn immersion<Intv>(node: Box<IntervalSearchTreeNode<Intv>>, v: &mut Vec<Intv>)
        where
            Intv: Interval,
        {
            node.left.map(|left| immersion(left, v));
            v.push(node.value);
            node.right.map(|right| immersion(right, v));
        }

        let mut v = Vec::with_capacity(self.count());
        immersion(self, &mut v);

        v
    }
}
