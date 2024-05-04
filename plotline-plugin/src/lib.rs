#[cfg(feature = "wasm")]
pub mod wasm;

pub mod experience;
pub mod proto;
pub mod store;

mod kind;
pub use kind::*;

mod error;
pub use error::*;

use plotline::{id::Indentify, plugin::PluginId};

/// A Plugin is a set of methods loaded at runtime that extends the default
/// behavior based on its [PluginKind].
pub trait Plugin: Indentify<Id = PluginId> + Sync + Send {
    /// Identifies the kind of the plugin.
    fn kind(&self) -> PluginKind;
    /// Executes the corresponding action passing its input encoded in bytes.
    fn run(&self, input: &[u8]) -> RunPluginResult;
}

/// A PluginFlavor represents a layer of abstraction between the generic form
/// given by the trait [Plugin] and the actual methods associated to the kind
/// of plugin.
pub trait PluginFlavor<'a>: TryFrom<&'a dyn Plugin, Error = Error> {
    /// Determines the kind of the plugin.
    fn kind() -> PluginKind;
}
