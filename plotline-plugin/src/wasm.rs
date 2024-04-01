use crate::{Plugin, PluginId, PluginKind};
use byteorder::{LittleEndian, ReadBytesExt};
use plotline::id::Identifiable;
use plotline_proto::plugin::{GetPluginId, GetPluginKind};
use protobuf::Message;
use std::{
    fs::File,
    io::Read,
    path::Path,
    str::FromStr,
    sync::{Arc, Mutex},
};
use wasmer::{
    CompileError, ExportError, Imports, Instance, InstantiationError, MemoryAccessError, Module,
    RuntimeError, Store, WasmSlice,
};
use wasmer_wasix::{WasiEnv, WasiError, WasiFunctionEnv, WasiRuntimeError};

const ID_FUNCTION_KEY: &str = "id";
const KIND_FUNCTION_KEY: &str = "kind";
const MEMORY_KEY: &str = "memory";

// const _HEAP_START: u32 = 0x110000;

// struct _HeapState {
//     start: u32,
//     offset: u32,
// }

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    WasiRuntimeError(#[from] WasiRuntimeError),
    #[error("{0}")]
    CompileError(#[from] CompileError),
    #[error("{0}")]
    WasiError(#[from] WasiError),
    #[error("{0}")]
    ExportError(#[from] ExportError),
    #[error("{0}")]
    MemoryAccessError(#[from] MemoryAccessError),
    #[error("{0}")]
    InstantiationError(#[from] Box<InstantiationError>), // Boxed because is to large
    #[error("{0}")]
    RuntimeError(#[from] RuntimeError),
    #[error("{0}")]
    Protobuf(#[from] protobuf::Error),
    #[error("{0}")]
    Io(#[from] std::io::Error),
    #[error("{0}")]
    Plugin(#[from] crate::Error),
    #[error("WASM engine may be corrupted")]
    Poison,
}

/// A WasmEngine holds all the information about Wasm that may be shared
/// between Wasm plugins.
pub struct WasmEngine {
    store: Store,
    wasi_env: WasiFunctionEnv,
}

impl WasmEngine {
    pub fn new() -> Result<Self> {
        let mut store = Store::default();
        let wasi_env = WasiEnv::builder("engine").finalize(&mut store)?;

        Ok(Self { store, wasi_env })
    }
}

/// WasmPluginBuilder builds instances of [WasmPlugin] by injecting to it the
/// same [WasmEngine].
pub struct WasmPluginBuilder {
    engine: Arc<Mutex<WasmEngine>>,
}

impl WasmPluginBuilder {
    pub fn new(engine: Arc<Mutex<WasmEngine>>) -> Self {
        Self { engine }
    }

    pub fn from_bytes(&self, bytes: &[u8]) -> Result<WasmPlugin> {
        let mut engine = self.engine.lock().map_err(|_| Error::Poison)?;
        let module = Module::new(&engine.store, bytes)?;
        let imports = engine
            .wasi_env
            .clone()
            .import_object(&mut engine.store, &module)?;

        let instance = Instance::new(&mut engine.store, &module, &imports)
            .map_err(Box::new)?;

        engine
            .wasi_env
            .clone()
            .initialize(&mut engine.store, instance.clone())?;

        let id_pointer = instance
            .exports
            .get_typed_function::<(), u8>(&engine.store, ID_FUNCTION_KEY)?
            .call(&mut engine.store)?;

        let kind_pointer = instance
            .exports
            .get_typed_function::<(), u8>(&engine.store, KIND_FUNCTION_KEY)?
            .call(&mut engine.store)?;

        Ok(WasmPlugin {
            _engine: self.engine.clone(),
            _module: module,
            _imports: imports,
            id: PluginId::from_str(
                &WasmPlugin::output::<GetPluginId>(&engine.store, &instance, id_pointer)?.id,
            )?,
            kind: PluginKind::from(
                WasmPlugin::output::<GetPluginKind>(&engine.store, &instance, kind_pointer)?
                    .kind
                    .enum_value()
                    .map_err(|_| crate::Error::NotAKind)?,
            ),
        })
    }

    pub fn from_file(&self, path: &Path) -> Result<WasmPlugin> {
        let mut f = File::open(path)?;
        let mut wasm_plugin = Vec::default();
        f.read_to_end(&mut wasm_plugin)?;

        self.from_bytes(&wasm_plugin)
    }
}

/// WasmPlugin implements the [Plugin] trait for any wasm module.
pub struct WasmPlugin {
    _engine: Arc<Mutex<WasmEngine>>,
    _module: Module,
    _imports: Imports,
    id: PluginId,
    kind: PluginKind,
}

impl Identifiable for WasmPlugin {
    type Id = PluginId;

    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}

impl Plugin for WasmPlugin {
    fn kind(&self) -> crate::PluginKind {
        self.kind.clone()
    }

    fn run(&self, _action: &str, _input: &[u8]) -> crate::PluginResult {
        unimplemented!()
    }
}

impl WasmPlugin {
    pub fn output<T>(store: &Store, instance: &Instance, pointer: u8) -> Result<T>
    where
        T: Message,
    {
        let memory = instance.exports.get_memory(MEMORY_KEY)?;
        let view = memory.view(&store);

        let output_len = WasmSlice::new(&view, pointer as u64, 4)?
            .read_to_vec()?
            .as_slice()
            .read_u32::<LittleEndian>()?;

        let output_bytes =
            WasmSlice::new(&view, pointer as u64 + 4, output_len as u64)?.read_to_vec()?;

        T::parse_from_bytes(&output_bytes).map_err(Into::into)
    }
}
