use plotline::id::Id;
use std::{fs::File, io::Read, path::Path};
use wasmer::{imports, Instance, Module, Store, Value};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {}

pub enum PluginFamily {
    ExperienceConstraint,
}

struct WasmPlugin {
    store: Store,
    module: Module,
    instance: Instance,
}

pub struct Plugin {
    id: Id<Self>,
    family: PluginFamily,
    wasm: WasmPlugin,
}

/// Stores all the available plugins.
#[derive(Default)]
pub struct PluginStore {
    wasm: Store,
}

impl PluginStore {
    /// Given the plugin's binary, adds it into the store.
    pub fn add(&mut self, bytes: &[u8]) -> Result<()> {
        Ok(())
    }
}
