//! An [Iterator] implementation for traversing arbitrary graphs.

use crate::id::Identify;

/// A placeholder for non-set generics.
pub struct NotSet;

/// An [Iterator] that traverses a graph starting from an specific node.
pub struct GraphIterator<T, SelectFn> {
    next_item: Option<T>,
    select_fn: SelectFn,
}

impl<T, SelectFn> Iterator for GraphIterator<T, SelectFn>
where
    T: Identify,
    SelectFn: Fn(&T) -> Option<T>,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let next_item = self.next_item.take();
        if let Some(node) = &next_item {
            self.next_item = (self.select_fn)(node);
        }

        next_item
    }
}

impl<T> GraphIterator<T, NotSet> {
    /// Returns a new iterator starting from the given root.
    pub fn new(root: T) -> Self {
        Self {
            next_item: Some(root),
            select_fn: NotSet,
        }
    }

    /// Sets the select function for the iterator.
    pub fn with_select<SelectFn>(self, select_fn: SelectFn) -> GraphIterator<T, SelectFn>
    where
        SelectFn: Fn(&T) -> Option<T>,
    {
        GraphIterator {
            next_item: self.next_item,
            select_fn,
        }
    }
}
