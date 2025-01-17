//! The plugin implementation for [`IntervalSearchTree`].

use std::marker::PhantomData;

use alvidir::{prelude::*, property::Extract};

use crate::{Interval, IntervalSearchTree};

/// Stores the relation between a node from the graph (node id) and its interval.
#[derive(Debug)]
struct NodeInterval<T, Intv>
where
    T: Identify,
{
    node_id: T::Id,
    interval: Intv,
}

impl<T, Intv> PartialEq for NodeInterval<T, Intv>
where
    T: Identify,
    T::Id: PartialEq,
    Intv: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.node_id == other.node_id && self.interval == other.interval
    }
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

type SearchTree<T, Intv> = IntervalSearchTree<NodeInterval<T, Intv>>;

pub struct IntervalPlugin<T, Extractor> {
    extractor: Extractor,
    node: PhantomData<T>,
}

impl<T, Extractor> IntervalPlugin<T, Extractor>
where
    T: 'static + Identify,
    T::Id: Clone + PartialEq,
    Extractor: 'static + Extract<T>,
    Extractor::Target: Interval + PartialEq,
{
    fn on_delete(
        _: Ctx<T>,
        target: Target<T>,
        search_tree: Res<SearchTree<T, Extractor::Target>>,
        extractor: Res<Extractor>,
    ) -> Result<()> {
        let Some(intervals) = (target, extractor).with(|(target, factory)| {
            factory.all(target).into_iter().map({
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

impl<T, Extractor> IntervalPlugin<T, Extractor>
where
    T: 'static + Identify,
    T::Id: Clone + PartialEq,
    Extractor: 'static + Extract<T>,
    Extractor::Target: Interval + PartialEq,
{
    fn on_save(
        _: Ctx<T>,
        target: Target<T>,
        search_tree: Res<SearchTree<T, Extractor::Target>>,
        extractor: Res<Extractor>,
    ) -> Result<()> {
        let Some(intervals) = (target, extractor).with(|(target, extractor)| {
            extractor.all(target).into_iter().map({
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

impl<T, Extractor> Plugin<T> for IntervalPlugin<T, Extractor>
where
    T: 'static + Identify,
    T::Id: Clone + PartialEq,
    Extractor: 'static + Extract<T>,
    Extractor::Target: Interval + PartialEq,
{
    fn install(self, schema: Schema<T>) -> Schema<T>
    where
        T: Identify,
    {
        schema
            .with_resource(self.extractor)
            .with_resource(SearchTree::<T, Extractor::Target>::default())
            .with_trigger(AfterSave, Self::on_save)
            .with_trigger(AfterDelete, Self::on_delete)
    }
}
