use super::{Error, PluginFlavor, PluginId, RawPlugin, Result};
use std::{collections::HashMap, marker::PhantomData, ops::Deref};

/// A PluginStore holds all the available plugins.
#[derive(Default)]
pub struct PluginStore<Intv> {
    plugins: HashMap<PluginId, Box<dyn RawPlugin>>,
    _interval: PhantomData<Intv>,
}

impl<Intv> PluginStore<Intv> {
    /// Adds a new plugin into the store.
    pub fn add(&mut self, plugin: Box<dyn RawPlugin>) -> Result<()> {
        if self.plugins.contains_key(&plugin.id()) {
            return Err(Error::AlreadyExists);
        }

        self.plugins.insert(plugin.id(), plugin);
        Ok(())
    }

    /// Removes from the store the plugin with the given id.
    pub fn remove(&mut self, id: PluginId) -> Result<()> {
        if self.plugins.remove(&id).is_none() {
            return Err(Error::NotFound);
        }

        Ok(())
    }

    /// Returns a vector with all those plugins of the corresponding flavor.
    pub fn retrieve<'b, T>(&'b self) -> Result<Vec<T>>
    where
        T: PluginFlavor<'b>,
    {
        self.plugins
            .values()
            .filter(|plugin| plugin.kind() == T::kind())
            .map(Deref::deref)
            .map(TryInto::try_into)
            .collect::<Result<Vec<_>>>()
    }
}
