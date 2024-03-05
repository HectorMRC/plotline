#[cfg(feature = "wasm")]
mod wasm;

use plotline::id::Identifiable;
use std::collections::HashMap;

/// PluginFamily determines the kind of a plugin.
#[derive(strum_macros::EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum PluginFamily {
    OnSaveExperienceConstraint,
}

/// A PluginId uniquely identifies a plugin.
#[derive(Hash, PartialEq, Eq)]
pub struct PluginId(String);

/// A Plugin is a piece of code that extens the default behavior of Plotline.
trait Plugin: Identifiable<Id = PluginId> {
    fn family(&self) -> PluginFamily;
}

/// A PluginStore holds all the available plugins, ready to be executed.
pub struct PluginStore {
    _plugins: HashMap<PluginId, Box<dyn Plugin>>,
}
