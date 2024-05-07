mod id;
pub use id::*;

mod error;
pub use error::*;
use tracing::info;

use crate::id::Indentify;
use futures::future;
use std::marker::PhantomData;

/// A command represents whatever that can be executed and may
/// return a value as result.
#[trait_variant::make]
pub trait Plugin<T>: Indentify<Id = PluginId> {
    async fn execute(self) -> Self;
    fn result(&self) -> Result<T>;
}

/// A PluginGroup represents a group of plugins of the same kind.
pub struct PluginGroup<T, R = ()> {
    plugins: Vec<T>,
    _result: PhantomData<R>,
}

impl<T, R> PluginGroup<T, R>
where
    T: Plugin<R>,
{
    pub async fn execute(mut self) -> Self {
        self.plugins = future::join_all(self.plugins.into_iter().map(|plugin| async {
            info!(id = plugin.id().as_ref(), "Running plugin");
            plugin.execute().await
        }))
        .await;

        self
    }

    pub fn result(&self) -> Result<()> {
        self.plugins
            .iter()
            .fold(None, |base, plugin| match (base, plugin.result()) {
                (Some(base_err), Err(err)) => Some(base_err.join(err)),
                (None, Err(err)) | (Some(err), Ok(_)) => Some(err),
                _ => None,
            })
            .map(Err)
            .unwrap_or(Ok(()))
    }
}

impl<T, R> PluginGroup<T, R> {
    pub fn new(plugins: Vec<T>) -> Self {
        Self {
            plugins,
            _result: PhantomData,
        }
    }

    pub fn map(mut self, f: impl Fn(T) -> T) -> Self {
        self.plugins = self.plugins.into_iter().map(f).collect();
        self
    }
}
