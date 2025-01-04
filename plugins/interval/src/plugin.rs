//! The plugin implementation for [`IntervalSearchTree`].

use alvidir::prelude::*;

use crate::{Interval, IntervalSearchTree};

/// Stores the relation between a node (node id) and its interval.
#[derive(Debug, PartialEq)]
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

impl<T, Intv> IntervalSearchTree<NodeInterval<T, Intv>>
where
    T: 'static + PartialEq + Identify,
    T::Id: Clone + PartialEq,
    Intv: 'static + PartialEq + Interval + Property<T>,
{
    fn on_delete(_: Ctx<T>, target: Target<T>, search_tree: Res<Self>) -> Result<()> {
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
                search_tree.delete(&interval);
            });
        });

        Ok(())
    }
}

impl<T, Intv> IntervalSearchTree<NodeInterval<T, Intv>>
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

impl<T, Intv> Plugin<T> for IntervalSearchTree<NodeInterval<T, Intv>>
where
    T: 'static + PartialEq + Identify,
    T::Id: Clone + PartialEq,
    Intv: 'static + PartialEq + Identify<Id = T::Id> + Interval + Property<T>,
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
