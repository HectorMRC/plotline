//! Transaction definition.

use std::sync::{Arc, OnceLock, RwLock, RwLockReadGuard, RwLockWriteGuard};

use crate::{
    deref::{ReadOnly, ReadWrite, TryDeref, TryDerefMut},
    graph::{Graph, NodeProxy, Source},
    id::Identify,
};

use super::{guard::SchemaWriteGuard, resource::ResourceSet, trigger::TriggerSet, Result, Schema};

/// Represents a set of operations that must be perfomed as a whole.
pub trait Transaction: Sized {
    /// The type being targeted by this transaction.
    type Target;

    /// Executes the given closure as a transaction.
    fn with<F, T>(self, f: F) -> Result<T>
    where
        F: FnOnce(Context<'_, Self::Target>) -> Result<T>;
}

/// Represents a set of operations that must be completed transactionally.
pub struct Background<'a, T>
where
    T: Identify,
{
    schema: &'a Schema<T>,
    guard: OnceLock<SchemaWriteGuard<'a, T>>,
    operations: Arc<RwLock<Vec<Operation<T>>>>,
}

impl<'a, T> From<&'a Schema<T>> for Background<'a, T>
where
    T: Identify,
{
    fn from(schema: &'a Schema<T>) -> Self {
        Self {
            schema,
            guard: Default::default(),
            operations: Default::default(),
        }
    }
}

impl<T> Transaction for Background<'_, T>
where
    T: Identify,
    T::Id: Clone + Ord,
{
    type Target = T;

    fn with<F, U>(self, f: F) -> Result<U>
    where
        F: FnOnce(Context<'_, Self::Target>) -> Result<U>,
    {
        f((&self).into()).inspect(|_| {
            self.commit();
        })
    }
}

impl<T> Background<'_, T>
where
    T: Identify,
    T::Id: Clone + Ord,
{
    fn commit(mut self) {
        let Some(mut guard) = self.guard.take() else {
            tracing::error!("committing uninitialized transaction");
            return;
        };

        let Some(ops) = Arc::into_inner(self.operations) else {
            tracing::error!("commiting transaction with contexts yet in use");
            return;
        };

        let ops = match ops.into_inner() {
            Ok(ops) => ops,
            Err(err) => {
                tracing::error!(error = err.to_string(), "committing poisoned transaction");
                return;
            }
        };

        ops.into_iter().for_each(|op| match op {
            Operation::Save(node) => {
                guard.insert(node);
            }
            Operation::Delete(node_id) => {
                guard.remove(&node_id);
            }
        });
    }
}

/// Represents a subset of operations that must be completed transactionally.
pub struct Foreground<'a, T>
where
    T: Identify,
{
    context: &'a Context<'a, T>,
    operations: Arc<RwLock<Vec<Operation<T>>>>,
}

impl<'a, T> From<&'a Context<'a, T>> for Foreground<'a, T>
where
    T: Identify,
{
    fn from(context: &'a Context<'_, T>) -> Self {
        Foreground {
            context,
            operations: Default::default(),
        }
    }
}

impl<T> Transaction for Foreground<'_, T>
where
    T: Identify,
{
    type Target = T;

    fn with<F, U>(self, f: F) -> Result<U>
    where
        F: FnOnce(Context<'_, Self::Target>) -> Result<U>,
    {
        f((&self).into()).inspect(|_| {
            self.commit();
        })
    }
}

impl<T> Foreground<'_, T>
where
    T: Identify,
{
    fn commit(self) {
        let Some(ops) = Arc::into_inner(self.operations) else {
            tracing::error!("commiting transaction with contexts yet in use");
            return;
        };

        let ops = match ops.into_inner() {
            Ok(ops) => ops,
            Err(err) => {
                tracing::error!(error = err.to_string(), "committing poisoned transaction");
                return;
            }
        };

        let mut upstream_ops = match self.context.operations.write() {
            Ok(ops) => ops,
            Err(err) => {
                tracing::error!(
                    error = err.to_string(),
                    "committing transaction into poisoned context"
                );
                return;
            }
        };

        upstream_ops.extend(ops);
    }
}

/// Represents an operation into the schema.
enum Operation<T>
where
    T: Identify,
{
    Save(T),
    Delete(T::Id),
}

impl<T> Identify for Operation<T>
where
    T: Identify,
{
    type Id = T::Id;

    fn id(&self) -> &Self::Id {
        match self {
            Operation::Save(node) => node.id(),
            Operation::Delete(node_id) => node_id,
        }
    }
}

/// The node targeted by a context.
pub struct Target<T> {
    lock: Option<Arc<RwLock<T>>>,
}

impl<T> Default for Target<T> {
    fn default() -> Self {
        Self {
            lock: Default::default(),
        }
    }
}

impl<T> Clone for Target<T> {
    fn clone(&self) -> Self {
        Self {
            lock: self.lock.clone(),
        }
    }
}

/// Holds a read-only access to the target's value.
pub struct TargetReadGuard<'a, T> {
    guard: Option<RwLockReadGuard<'a, T>>,
}

impl<T> Default for TargetReadGuard<'_, T> {
    fn default() -> Self {
        Self {
            guard: Default::default(),
        }
    }
}

impl<T> TryDeref for TargetReadGuard<'_, T> {
    type Target = T;

    fn try_deref(&self) -> Option<&Self::Target> {
        self.guard.as_deref()
    }
}

impl<T> ReadOnly for Target<T>
where
    T: 'static,
{
    type Target = T;

    type Guard<'a> = TargetReadGuard<'a, T>;

    fn read(&self) -> Self::Guard<'_> {
        let Some(lock) = self.lock.as_ref() else {
            return Default::default();
        };

        match lock.read() {
            Ok(guard) => TargetReadGuard { guard: Some(guard) },
            Err(err) => {
                tracing::error!(error = err.to_string(), "accessing transaction target");
                Default::default()
            }
        }
    }
}

/// Holds a read-write access to the target's value.
pub struct TargetWriteGuard<'a, T> {
    guard: Option<RwLockWriteGuard<'a, T>>,
}

impl<T> Default for TargetWriteGuard<'_, T> {
    fn default() -> Self {
        Self {
            guard: Default::default(),
        }
    }
}

impl<T> TryDeref for TargetWriteGuard<'_, T> {
    type Target = T;

    fn try_deref(&self) -> Option<&Self::Target> {
        self.guard.as_deref()
    }
}

impl<T> TryDerefMut for TargetWriteGuard<'_, T> {
    fn try_deref_mut(&mut self) -> Option<&mut Self::Target> {
        self.guard.as_deref_mut()
    }
}

impl<T> ReadWrite for Target<T>
where
    T: 'static,
{
    type Target = T;

    type Guard<'a> = TargetWriteGuard<'a, T>;

    fn write(&self) -> Self::Guard<'_> {
        let Some(lock) = self.lock.as_ref() else {
            return Default::default();
        };

        match lock.write() {
            Ok(guard) => TargetWriteGuard { guard: Some(guard) },
            Err(err) => {
                tracing::error!(error = err.to_string(), "accessing transaction target");
                Default::default()
            }
        }
    }
}

impl<T> Target<T> {
    /// Sets the given value as the target.
    pub fn set(&mut self, value: T) {
        self.lock = Some(Arc::new(RwLock::new(value)));
    }

    // /// Gets a read-only access to the target and executes the given closure.
    // pub fn with<F, R>(&self, f: F) -> Option<R>
    // where
    //     F: FnOnce(&T) -> R,
    // {
    //     match self.lock.as_ref()?.read() {
    //         Ok(guard) => Some(f(&guard)),
    //         Err(err) => {
    //             tracing::error!(error = err.to_string(), "accessing target");
    //             None
    //         }
    //     }
    // }

    // /// Gets a read-write access to the target and executes the given closure.
    // pub fn with_mut<F, R>(&self, f: F) -> Option<R>
    // where
    //     F: FnOnce(&mut T) -> R,
    // {
    //     match self.lock.as_ref()?.write() {
    //         Ok(mut guard) => Some(f(&mut guard)),
    //         Err(err) => {
    //             tracing::error!(error = err.to_string(), "accessing target");
    //             None
    //         }
    //     }
    // }
}

impl<'a, T> From<&'a Context<'a, T>> for Target<T>
where
    T: Identify,
{
    fn from(context: &'a Context<T>) -> Self {
        context.target().clone()
    }
}

/// Represents a subset of operations from a transaction.
pub struct Context<'a, T>
where
    T: Identify,
{
    graph: &'a Graph<T>,
    schema: &'a Schema<T>,
    parent: Option<&'a Context<'a, T>>,
    operations: Arc<RwLock<Vec<Operation<T>>>>,
    target: Target<T>,
}

impl<T> Source for Context<'_, T>
where
    T: Identify + Clone,
    T::Id: Ord + PartialEq,
{
    type Node = T;

    fn get(&self, id: &<Self::Node as Identify>::Id) -> Option<Self::Node> {
        let guard = match self.operations.read() {
            Ok(ops) => ops,
            Err(err) => err.into_inner(),
        };

        match guard.iter().rev().find(|&op| op.id() == id) {
            Some(Operation::Save(node)) => Some(node.clone()),
            Some(Operation::Delete(_)) => None,
            None => self
                .parent
                .map(|parent| parent.get(id))
                .unwrap_or_else(|| self.graph.get(id)),
        }
    }

    fn contains(&self, id: &<Self::Node as Identify>::Id) -> bool {
        let guard = match self.operations.read() {
            Ok(ops) => ops,
            Err(err) => err.into_inner(),
        };

        match guard.iter().rev().find(|&op| op.id() == id) {
            Some(Operation::Save(_)) => true,
            Some(Operation::Delete(_)) => false,
            None => self
                .parent
                .map(|parent| parent.contains(id))
                .unwrap_or_else(|| self.graph.contains(id)),
        }
    }
}

impl<'a, T> From<&'a Background<'_, T>> for Context<'a, T>
where
    T: Identify,
{
    fn from(tx: &'a Background<'_, T>) -> Self {
        Context {
            schema: tx.schema,
            graph: tx.guard.get_or_init(|| tx.schema.write()),
            operations: tx.operations.clone(),
            target: Default::default(),
            parent: Default::default(),
        }
    }
}

impl<'a, T> From<&'a Foreground<'_, T>> for Context<'a, T>
where
    T: Identify,
{
    fn from(tx: &'a Foreground<'_, T>) -> Self {
        Context {
            graph: tx.context.graph,
            schema: tx.context.schema,
            operations: tx.operations.clone(),
            target: Default::default(),
            parent: Some(tx.context),
        }
    }
}

impl<T> Context<'_, T>
where
    T: Identify + Clone,
    T::Id: Ord,
{
    /// Returns the [`NodeProxy`] for the given id.
    pub fn node(&self, node_id: T::Id) -> NodeProxy<'_, Self> {
        NodeProxy::new(self, node_id)
    }
}

impl<T> Context<'_, T>
where
    T: Identify,
{
    /// Assigns a target to this context.
    pub fn with_target(mut self, target: T) -> Self {
        self.target.set(target);
        self
    }

    /// Registers the save operation as part of the transaction.
    pub fn save(&self, node: T) {
        let mut guard = match self.operations.write() {
            Ok(ops) => ops,
            Err(err) => err.into_inner(),
        };

        guard.push(Operation::Save(node));
    }

    /// Registers the delete operation as part of the transaction.
    pub fn delete(&self, node_id: T::Id) {
        let mut guard = match self.operations.write() {
            Ok(ops) => ops,
            Err(err) => err.into_inner(),
        };

        guard.push(Operation::Delete(node_id));
    }

    /// Returns a reference to the underlying schema's [`ResourceSet`].
    pub fn resources(&self) -> &ResourceSet {
        self.schema.resources()
    }

    /// Returns a reference to the underlying schema's [`TriggerSet`].
    pub fn triggers(&self) -> &TriggerSet<T> {
        self.schema.triggers()
    }

    /// Returns a reference to the transaction's target.
    pub fn target(&self) -> &Target<T> {
        &self.target
    }

    /// Returns a new transaction holded by this context.
    #[inline]
    pub fn transaction(&self) -> Foreground<'_, T> {
        self.into()
    }
}

/// Represents a constrained access to a [`Context`].
pub struct Ctx<'a, T>
where
    T: Identify,
{
    context: &'a Context<'a, T>,
}

impl<'a, T> Ctx<'a, T>
where
    T: Identify,
{
    /// Returns a new transaction holded by this context.
    #[inline]
    pub fn transaction(&'a self) -> Foreground<'a, T> {
        self.context.into()
    }
}

impl<'a, T> From<&'a Context<'a, T>> for Ctx<'a, T>
where
    T: Identify,
{
    fn from(context: &'a Context<T>) -> Self {
        Ctx { context }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        graph::{
            fixtures::{fake_node, FakeNode},
            Graph, Source,
        },
        schema::{transaction::Context, Error, Result, Schema},
    };

    use super::Transaction;

    #[test]
    fn context_should_overwrite_original_state() {
        let schema: Schema<_> = Graph::default().with_node(fake_node!(1)).into();

        let tx = schema.transaction();
        tx.with(|ctx| {
            ctx.delete(1);
            assert!(
                !ctx.contains(&1),
                "deletion should be registered into the context"
            );

            ctx.save(fake_node!(2));
            assert!(
                ctx.contains(&2),
                "save should be registered into the context"
            );

            Ok(())
        })
        .expect("non-failing transaction should return Ok");
    }

    #[test]
    fn uncommited_transaction_should_not_apply_changes() {
        let schema: Schema<_> = Graph::default().with_node(fake_node!(1)).into();
        schema
            .transaction()
            .with(|ctx| {
                ctx.delete(1);
                ctx.save(fake_node!(2));

                Result::<()>::Err(Error::custom("failed transaction"))
            })
            .expect_err("transaction error should be propagated");

        assert!(
            schema.read().contains(&1),
            "uncommitted transaction should not apply changes"
        );

        assert!(
            !schema.read().contains(&2),
            "uncommitted transaction should not apply changes"
        );
    }

    #[test]
    fn commited_transaction_should_apply_changes() {
        let schema: Schema<_> = Graph::default().with_node(fake_node!(1)).into();
        let tx = schema.transaction();

        tx.with(|ctx| {
            ctx.delete(1);
            ctx.save(fake_node!(2));

            Ok(())
        })
        .expect("transaction should not fail");

        assert!(
            !schema.read().contains(&1),
            "committed transaction should apply changes"
        );

        assert!(
            schema.read().contains(&2),
            "committed transaction should apply changes"
        );
    }

    #[test]
    fn subtransactions_should_be_independent() {
        let schema: Schema<_> = Graph::default().with_node(fake_node!(1)).into();

        let tx_1 = schema.transaction();
        let ctx_1 = Context::from(&tx_1);

        let tx_2 = ctx_1.transaction();
        let ctx_2 = Context::from(&tx_2);
        ctx_2.delete(1);

        let tx_3 = ctx_1.transaction();
        let ctx_3 = Context::from(&tx_3);
        ctx_3.save(fake_node!(2));

        assert!(
            ctx_1.contains(&1),
            "clean context should keep original state"
        );
        assert!(
            !ctx_1.contains(&2),
            "clean context should keep original state"
        );

        assert!(
            !ctx_2.contains(&1),
            "context shoudl overwrite original state"
        );
        assert!(
            !ctx_2.contains(&2),
            "subtransactions should not interfer each other"
        );

        assert!(
            ctx_3.contains(&1),
            "subtransactions should not interfer each other"
        );
        assert!(
            ctx_3.contains(&2),
            "context shoudl overwrite original state."
        );
    }

    #[test]
    fn committed_subtransaction_should_apply_on_parent_context() {
        let schema: Schema<_> = Graph::default().with_node(fake_node!(1)).into();

        schema
            .transaction()
            .with(|ctx_1| {
                ctx_1
                    .transaction()
                    .with(|ctx_2| {
                        ctx_2.delete(1);
                        Ok(())
                    })
                    .expect("transaction should not fail");

                assert!(
                    !ctx_1.contains(&1),
                    "committed subtransaction should apply on parent context"
                );

                Ok(())
            })
            .unwrap();
    }

    #[test]
    fn parent_changes_should_be_visible_from_subtransactions() {
        let schema: Schema<_> = Graph::default().into();

        schema
            .transaction()
            .with(|ctx_1| {
                ctx_1.save(fake_node!(1));
                ctx_1
                    .transaction()
                    .with(|ctx_2| {
                        assert!(
                            ctx_2.contains(&1),
                            "parent changes should be visible from substransaction"
                        );

                        Ok(())
                    })
                    .unwrap();

                Ok(())
            })
            .unwrap();
    }
}
