#[cfg(feature = "wasm")]
mod wasm;

use plotline::id::Identifiable;
use std::collections::HashMap;

/// PluginKind determines the kind of a plugin.
#[derive(strum_macros::EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum PluginKind {
    OnSaveExperienceConstraint,
}

/// A PluginId uniquely identifies a plugin.
#[derive(Hash, PartialEq, Eq)]
pub struct PluginId(String);

/// PluginResult represents the output or crashing cause of a plugin.
type PluginResult = std::result::Result<Vec<u8>, String>;

/// A Plugin is a piece of code that extends the default behavior of Plotline.
pub trait Plugin: Identifiable<Id = PluginId> {
    /// Identifies the kind of plugin.
    fn kind(&self) -> PluginKind;
    /// Executes the corresponding action passing the given bytes as parameter.
    fn run(&self, action: &str, bytes: &[u8]) -> PluginResult;
}

/// A PluginStore holds all the available plugins.
pub struct PluginStore {
    _plugins: HashMap<PluginId, Box<dyn Plugin>>,
}
