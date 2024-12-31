use alvidir::prelude::*;

use crate::{search_tree::IntervalSearchTreeNode, Interval};

/// Stores the relation between a node (node id) and its interval.
struct NodeInterval<T, Intv>
where
    T: Identify,
{
    node_id: T::Id,
    interval: Intv,
}

impl<T, Intv> Interval for NodeInterval<T, Intv>
where
    T: Identify,
    Intv: Interval,
{
    type Bound = Intv::Bound;

    fn lo(&self) -> Self::Bound {
        self.interval.lo()
    }

    fn hi(&self) -> Self::Bound {
        self.interval.hi()
    }
}

impl<T, Intv> Identify for NodeInterval<T, Intv>
where
    T: Identify,
{
    type Id = T::Id;

    fn id(&self) -> &Self::Id {
        &self.node_id
    }
}

/// An internval search tree.
pub struct IntervalSearchTree<T, Intv>
where
    T: Identify,
    Intv: Interval,
{
    root: Option<IntervalSearchTreeNode<NodeInterval<T, Intv>>>,
}

impl<T, Intv> Default for IntervalSearchTree<T, Intv>
where
    T: Identify,
    Intv: Interval,
{
    fn default() -> Self {
        Self {
            root: Default::default(),
        }
    }
}

impl<T, Intv> IntervalSearchTree<T, Intv>
where
    T: Identify,
    Intv: Interval,
{
    fn insert(&mut self, interval: NodeInterval<T, Intv>) {
        if let Some(root) = &mut self.root {
            return root.insert(interval);
        }

        self.root = Some(IntervalSearchTreeNode::new(interval));
    }
}

impl<T, Intv> IntervalSearchTree<T, Intv>
where
    T: Identify,
    Intv: Identify<Id = T::Id> + Interval,
{
    fn delete(&mut self, id: &T::Id) {
        if let Some(root) = self.root.as_mut() {
            root.delete(id);
        }
    }
}

impl<T, Intv> IntervalSearchTree<T, Intv>
where
    T: 'static + Identify,
    T::Id: Clone,
    Intv: 'static + Interval + Property<T>,
{
    fn on_save(_: Ctx<T>, target: Target<T>, search_tree: Res<Self>) -> Result<()> {
        let Some(intervals) = target.with(|target| {
            Intv::all(target).into_iter().map({
                let node_id = target.id().clone();
                move |interval| NodeInterval {
                    node_id: node_id.clone(),
                    interval,
                }
            })
        }) else {
            return Ok(());
        };

        search_tree.with_mut(|search_tree| {
            intervals.into_iter().for_each(|interval| {
                search_tree.insert(interval);
            });
        });

        Ok(())
    }
}

impl<T, Intv> IntervalSearchTree<T, Intv>
where
    T: 'static + Identify,
    T::Id: Clone,
    Intv: 'static + Identify<Id = T::Id> + Interval + Property<T>,
{
    fn on_delete(_: Ctx<T>, target: Target<T>, search_tree: Res<Self>) -> Result<()> {
        let Some(id) = target.with(|target| target.id().clone()) else {
            return Ok(());
        };

        search_tree.with_mut(|search_tree| {
            search_tree.delete(&id);
        });

        Ok(())
    }
}

impl<T, Intv> Plugin<T> for IntervalSearchTree<T, Intv>
where
    T: 'static + Identify,
    T::Id: Clone,
    Intv: 'static + Identify<Id = T::Id> + Interval + Property<T>,
{
    fn setup(&self, schema: Schema<T>) -> Schema<T>
    where
        T: Identify,
    {
        schema
            .with_resource(Self::default())
            .with_trigger(AfterSave, Self::on_save)
            .with_trigger(AfterDelete, Self::on_delete)
    }
}
