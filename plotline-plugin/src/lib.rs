#[cfg(feature = "wasm")]
pub mod wasm;

pub mod experience;
pub mod kind;
pub mod proto;
pub mod store;
pub mod version;

mod error;
pub use error::*;

pub use kind::PluginKind;
pub use plotline::plugin::PluginId;
pub use version::PluginVersion;

use plotline::id::Indentify;

/// A RawPlugin is a set of methods loaded at runtime that extends the default
/// behavior based on its [PluginKind].
pub trait RawPlugin: Indentify<Id = PluginId> + Sync + Send {
    /// Identifies the kind of the plugin.
    fn kind(&self) -> kind::PluginKind;
    /// Determines the current version of the plugin.
    fn version(&self) -> version::PluginVersion;
    /// Executes the corresponding action passing its input encoded in bytes.
    fn run(&self, input: &[u8]) -> RawResult;
}

/// A PluginFlavor represents a layer of abstraction between the generic form
/// given by the trait [Plugin] and the actual methods associated to the kind
/// of plugin.
pub trait PluginFlavor<'a>: TryFrom<&'a dyn RawPlugin, Error = Error> {
    /// Determines the kind of the plugin.
    fn kind() -> kind::PluginKind;
}
