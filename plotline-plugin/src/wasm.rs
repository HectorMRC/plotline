use crate::{PluginId, PluginKind, PluginVersion, RawError, RawPlugin, RawResult};
use byteorder::{LittleEndian, ReadBytesExt};
use plotline::id::Indentify;
use plotline_proto::plugin::{GetPluginId, GetPluginKind, GetPluginVersion};
use protobuf::Message;
use std::{
    fs::File,
    io::Read,
    path::Path,
    str::FromStr,
    sync::{Arc, Mutex, PoisonError},
};
use wasmer::{
    CompileError, ExportError, Instance, InstantiationError, MemoryAccessError, MemoryError, Module, RuntimeError, Store, WasmSlice
};
use wasmer_wasix::{WasiEnv, WasiError, WasiRuntimeError};

const PROGRAM_NAME: &str = "plugin";
const ID_FUNCTION_KEY: &str = "id";
const KIND_FUNCTION_KEY: &str = "kind";
const VERSION_FUNCTION_KEY: &str = "version";
const FUNCTION_KEY_RUN: &str = "run";
const MEMORY_KEY: &str = "memory";
const HEAP_START: u32 = 0x110000;

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
    MemoryError(#[from] MemoryError),
    #[error("{0}")]
    InstantiationError(#[from] Box<InstantiationError>), // Boxed because is too large
    #[error("{0}")]
    RuntimeError(#[from] RuntimeError),
    #[error("{0}")]
    Protobuf(#[from] protobuf::Error),
    #[error("{0}")]
    Io(#[from] std::io::Error),
    #[error("{0}")]
    Id(#[from] plotline::id::Error),
    #[error("{0}")]
    Kind(#[from] crate::kind::Error),
    #[error("{0}")]
    Version(#[from] crate::version::Error),
    #[error("WASM engine may be corrupted")]
    Poison,
}

impl<T> From<PoisonError<T>> for Error {
    fn from(_: PoisonError<T>) -> Self {
        Error::Poison
    }
}

impl From<Error> for RawError {
    fn from(value: Error) -> Self {
        value.to_string().into()
    }
}

/// WasmPluginFactory builds instances of [WasmPlugin] by injecting to it the
/// same [WasmEngine].
#[derive(Default)]
pub struct WasmPluginFactory {
    store: Arc<Mutex<Store>>,
}

impl WasmPluginFactory {
    pub fn from_bytes(&self, bytes: &[u8]) -> Result<WasmPlugin> {
        let mut store = self.store.lock().map_err(|_| Error::Poison)?;
        let module = Module::new(&store, bytes)?;

        let mut wasi_env = WasiEnv::builder(PROGRAM_NAME)
            .finalize(&mut store)?;

        let imports = wasi_env
            .import_object(&mut store, &module)?;

        let instance = Instance::new(&mut store, &module, &imports).map_err(Box::new)?;
        wasi_env.initialize(&mut store, instance.clone())?;

        let id = PluginId::from_str(
            &WasmPlugin::call::<GetPluginId>(ID_FUNCTION_KEY, &mut store, &instance)?.id,
        )?;

        let kind = PluginKind::try_from(
            WasmPlugin::call::<GetPluginKind>(KIND_FUNCTION_KEY, &mut store, &instance)?
                .kind,
        )?;

        let version = PluginVersion::from_str(
            &WasmPlugin::call::<GetPluginVersion>(
                VERSION_FUNCTION_KEY,
                &mut store,
                &instance,
            )?
            .version,
        )?;

        Ok(WasmPlugin {
            store: self.store.clone(),
            instance,
            id,
            kind,
            version,
        })
    }

    pub fn from_file(&self, path: &Path) -> Result<WasmPlugin> {
        let mut f = File::open(path)?;
        let mut wasm_plugin = Vec::default();
        f.read_to_end(&mut wasm_plugin)?;

        self.from_bytes(&wasm_plugin)
    }
}

/// WasmPlugin implements the [RawPlugin] trait for any wasm module.
pub struct WasmPlugin {
    store: Arc<Mutex<Store>>,
    instance: Instance,
    id: PluginId,
    kind: PluginKind,
    version: PluginVersion,
}

impl Indentify for WasmPlugin {
    type Id = PluginId;

    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}

impl RawPlugin for WasmPlugin {
    fn kind(&self) -> PluginKind {
        self.kind.clone()
    }

    fn version(&self) -> PluginVersion {
        self.version.clone()
    }

    fn run(&self, input: &[u8]) -> RawResult {
        self.execute(input).map_err(Into::into)
    }
}

impl WasmPlugin {
    fn execute(&self, input: &[u8]) -> Result<Vec<u8>> {
        let mut store = self.store.lock()?;
        Self::input(&mut store, &self.instance, input)?;

        let action = self
            .instance
            .exports
            .get_typed_function::<u32, u32>(&store, FUNCTION_KEY_RUN)?;

        let pointer = action.call(&mut store, HEAP_START)?;
        Self::output(&store, &self.instance, pointer)
    }

    fn call<T>(method: &str, store: &mut Store, instance: &Instance) -> Result<T>
    where
        T: Message,
    {
        let pointer = instance
            .exports
            .get_typed_function::<(), u32>(store, method)?
            .call(store)?;

        let output_bytes = Self::output(store, instance, pointer)?;
        T::parse_from_bytes(&output_bytes).map_err(Into::into)
    }

    fn input(store: &mut Store, instance: &Instance, input: &[u8]) -> Result<()> {
        let input_len = (input.len() as u32).to_le_bytes();
        let input = [&input_len[..], input].concat();

        let memory = instance.exports.get_memory(MEMORY_KEY)?;
        let pages = (input.len() / wasmer::WASM_PAGE_SIZE) + 1;
        memory.grow(store, pages as u32)?;

        let view = memory.view(store);
        view.write(HEAP_START as u64, &input)?;

        Ok(())
    }

    fn output(store: &Store, instance: &Instance, pointer: u32) -> Result<Vec<u8>> {
        let memory = instance.exports.get_memory(MEMORY_KEY)?;
        let view = memory.view(&store);

        let output_len = WasmSlice::new(&view, pointer as u64, 4)?
            .read_to_vec()?
            .as_slice()
            .read_u32::<LittleEndian>()?;

        eprintln!("OUTPUT LEN: {output_len}");
        Ok(WasmSlice::new(&view, pointer as u64 + 4, output_len as u64)?.read_to_vec()?)
    }
}
