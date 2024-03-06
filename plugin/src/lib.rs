#[cfg(feature = "wasm")]
mod wasm;

use plotline::id::Identifiable;
use std::collections::HashMap;

/// PluginKind determines the kind of a plugin.
#[derive(strum_macros::EnumString)]
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

/// A PluginEngine is the layer between the actual plugin and plotline.
pub trait PluginEngine: Identifiable<Id = PluginId> {
    /// Identifies the kind of plugin.
    fn kind(&self) -> PluginKind;
    /// Executes the corresponding action passing its parameters encoded in
    /// bytes.
    fn run(&self, action: &str, bytes: &[u8]) -> PluginResult;
}

/// A PluginStore holds all the available plugins.
pub struct PluginStore {
    _plugins: HashMap<PluginId, Box<dyn PluginEngine>>,
}
