#[cfg(feature = "wasm")]
pub mod wasm;

mod entity;
mod event;

mod experience;
pub use experience::*;

use plotline::id::Identifiable;
use plotline_proto::plugin as proto;
use std::{collections::HashMap, marker::PhantomData, ops::Deref, str::FromStr};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("plugin not found")]
    NotFound,
    #[error("plugin already exists ")]
    AlreadyExists,
    #[error("plugin is not of the expected kind")]
    WrongKind,
    #[error("invalid id")]
    NotAnId,
    #[error("invalid kind")]
    NotAKind,
}

/// PluginKind determines the kind of a plugin.
#[derive(PartialEq, Eq, Clone, strum_macros::EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum PluginKind {
    /// Plugins of this kind will be executed before saving an experience. Its
    /// result will indicate whether the experience is suitable to be saved or
    /// not.
    BeforeSaveExperience,
}

impl From<proto::PluginKind> for PluginKind {
    fn from(value: proto::PluginKind) -> Self {
        match value {
            proto::PluginKind::BEFORE_SAVE_EXPERIENCE => PluginKind::BeforeSaveExperience,
        }
    }
}

/// A PluginId uniquely identifies a plugin.
#[derive(Hash, PartialEq, Eq, Clone)]
pub struct PluginId(String);

impl FromStr for PluginId {
    type Err = Error;

    fn from_str(value: &str) -> Result<Self> {
        let is_invalid_char = |c: char| -> bool {
            const INVALID_CHARS: [char; 3] = ['\n', '\r', ' '];
            INVALID_CHARS.contains(&c)
        };

        if value.is_empty() || value.contains(is_invalid_char) {
            return Err(Error::NotAnId);
        }

        Ok(PluginId(value.to_string()))
    }
}

/// PluginResult represents the output or crashing cause of a plugin.
pub type PluginResult = std::result::Result<Vec<u8>, String>;

/// A Plugin is a set of methods loaded at runtime that extends the default
/// behavior based on its [PluginKind].
pub trait Plugin: Identifiable<Id = PluginId> + Sync + Send {
    /// Identifies the kind of the plugin.
    fn kind(&self) -> PluginKind;
    /// Executes the corresponding action passing its input encoded in bytes.
    fn run(&self, action: &str, input: &[u8]) -> PluginResult;
}

/// A FlavoredPlugin represents a layer of abstraction between the generic form
/// given by the trait [Plugin] and the actual methods associated to the kind
/// of plugin.
pub trait FlavoredPlugin<'a>: TryFrom<&'a dyn Plugin, Error = Error> {
    /// Determines the kind of the plugin.
    fn kind() -> PluginKind;
}

/// A PluginStore holds all the available plugins.
#[derive(Default)]
pub struct PluginStore<Intv> {
    plugins: HashMap<PluginId, Box<dyn Plugin>>,
    _interval: PhantomData<Intv>,
}

impl<Intv> PluginStore<Intv> {
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

    /// Returns a vector with all those plugins of the corresponding flavor.
    pub fn retrieve<'b, T>(&'b self) -> Result<Vec<T>>
    where
        T: FlavoredPlugin<'b>,
    {
        self.plugins
            .values()
            .filter(|plugin| plugin.kind() == T::kind())
            .map(Deref::deref)
            .map(TryInto::try_into)
            .collect::<Result<Vec<_>>>()
    }
}
