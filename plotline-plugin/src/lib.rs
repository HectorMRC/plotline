#[cfg(feature = "wasm")]
mod wasm;

use plotline::id::Identifiable;
use std::collections::HashMap;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("plugin not found")]
    NotFound,
    #[error("plugin already exists ")]
    AlreadyExists,
}

/// PluginKind determines the kind of a plugin.
#[derive(PartialEq, Eq, strum_macros::EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum PluginKind {
    /// Plugins of this kind will be executed before saving an experience. Its
    /// result will indicate whether the experience is suitable to be saved or
    /// not.
    OnSaveExperienceConstraint,
}

/// A PluginId uniquely identifies a plugin.
#[derive(Hash, PartialEq, Eq)]
pub struct PluginId(String);

/// PluginResult represents the output or crashing cause of a plugin.
type PluginResult = std::result::Result<Vec<u8>, String>;

/// A Plugin is a set of methods loaded at runtime that extends the default
/// behavior based on its [PluginKind].
pub trait Plugin: Identifiable<Id = PluginId> {
    /// Identifies the kind of the plugin.
    fn kind(&self) -> PluginKind;
    /// Executes the corresponding action passing its inpu encoded in bytes.
    fn run(&self, action: &str, input: &[u8]) -> PluginResult;
}

/// A PluginStore holds all the available plugins.
#[derive(Default)]
pub struct PluginStore {
    plugins: HashMap<PluginId, Box<dyn Plugin>>,
}

impl PluginStore {
    /// Adds a new plugin into the store.
    pub fn add(&mut self, plugin: Box<dyn Plugin>) -> Result<()> {
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

    /// Returns a vector with all those plugins matching the given filter.
    pub fn filter<'a>(&'a self, filter: PluginFilter) -> Vec<&'a Box<dyn Plugin>> {
        self.plugins
            .values()
            .filter(|plugin| filter.matches(plugin))
            .collect()
    }
}

#[derive(Default)]
pub struct PluginFilter {
    id: Option<PluginId>,
    kind: Option<PluginKind>,
}

impl PluginFilter {
    pub fn with_id(mut self, id: Option<PluginId>) -> Self {
        self.id = id;
        self
    }

    pub fn with_kind(mut self, kind: Option<PluginKind>) -> Self {
        self.kind = kind;
        self
    }

    fn matches(&self, plugin: &Box<dyn Plugin>) -> bool {
        if self
            .id
            .as_ref()
            .map(|id| id != &plugin.id())
            .unwrap_or_default()
        {
            return false;
        }

        if self
            .kind
            .as_ref()
            .map(|kind| kind != &plugin.kind())
            .unwrap_or_default()
        {
            return false;
        }

        true
    }
}
