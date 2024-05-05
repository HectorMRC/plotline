use clap::Parser;
use once_cell::sync::Lazy;
use plotline::{
    entity::{application::EntityApplication, repository::InMemoryEntityRepository},
    event::{application::EventApplication, repository::InMemoryEventRepository},
    experience::{application::ExperienceApplication, repository::InMemoryExperienceRepository},
    moment::Moment,
    period::Period,
};
use plotline_cli::{entity::EntityCli, event::EventCli, experience::ExperienceCli, CliCommand};
use plotline_plugin::{store::PluginStore, wasm::WasmPluginFactory};
use serde::{Deserialize, Serialize};
use std::{
    ffi::OsString,
    fmt::Display,
    fs::{read_dir, File, OpenOptions},
    io::{BufReader, BufWriter, Write},
    path::{Path, PathBuf},
    process,
    str::FromStr,
    sync::Arc,
};
use tracing::{debug, error, info, warn, Level};

const ENV_PLOTFILE: &str = "PLOTFILE";
const ENV_PLUGINS: &str = "PLUGINS";

static BASE_PATH: Lazy<PathBuf> =
    Lazy::new(|| dirs::home_dir().unwrap_or_default().join(".plotline"));

static DEFAULT_PLOTFILE_PATH: Lazy<OsString> =
    Lazy::new(|| BASE_PATH.join("plotfile.yaml").into_os_string());

static DEFAULT_PLUGINS_DIR: Lazy<OsString> =
    Lazy::new(|| BASE_PATH.join("plugins").into_os_string());

/// Implements the [Serialize] and [Deserialize] traits to persist and recover
/// the state of the repositories.
#[derive(Default, Serialize, Deserialize)]
pub struct Snapshot {
    #[serde(flatten)]
    entity_repo: Arc<InMemoryEntityRepository>,
    #[serde(flatten)]
    event_repo: Arc<InMemoryEventRepository<Period<Moment>>>,
    #[serde(flatten)]
    experience_repo: Arc<InMemoryExperienceRepository<Period<Moment>>>,
}

/// A plotline manager.
#[derive(Parser)]
#[command(name = "plot", about = "A plotline manager.", version = "0.0.1")]
struct Cli {
    #[command(subcommand)]
    command: CliCommand,

    /// The data source file.
    #[arg(
        env = ENV_PLOTFILE,
        default_value = &*DEFAULT_PLOTFILE_PATH,
        default_missing_value = "always",
        global = true,
        short, long
    )]
    file: PathBuf,

    /// The plugins directory.
    #[arg(
        env = ENV_PLUGINS,
        default_value = &*DEFAULT_PLUGINS_DIR,
        default_missing_value = "always",
        global = true,
        short, long
    )]
    plugins: PathBuf,

    /// Do not print progress bars.
    #[arg(global = true, short, long)]
    verbose: bool,
}

/// Returns the value of the result if, and only if, the result is OK.
/// Otherwise prints the error and exits.
#[inline]
fn unwrap_or_exit<T, E>(result: Result<T, E>) -> T
where
    E: Display,
{
    match result {
        Err(error) => {
            error!(error = error.to_string(), "Aborting transaction");
            process::exit(1)
        }
        Ok(value) => value,
    }
}

async fn snapshot_from_yaml(path: &Path) -> Snapshot {
    if !path.exists() {
        warn!(
            path = path.to_string_lossy().to_string(),
            "Snapshot not found, using default"
        );

        return Snapshot::default();
    }

    info!(
        path = path.to_string_lossy().to_string(),
        "Loading snapshot"
    );

    let f = unwrap_or_exit(File::open(path));
    let reader = BufReader::new(f);
    unwrap_or_exit(serde_yaml::from_reader(reader))
}

async fn snapshot_into_yaml(path: &Path, snapshot: &Snapshot) {
    info!(
        path = path.to_string_lossy().to_string(),
        "Storing snapshot"
    );

    let f = unwrap_or_exit(
        OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(path),
    );

    let mut writer = BufWriter::new(f);
    unwrap_or_exit(serde_yaml::to_writer(&mut writer, &snapshot));
    unwrap_or_exit(writer.flush());
}

#[derive(strum_macros::EnumString, strum_macros::Display)]
#[strum(serialize_all = "lowercase")]
enum PluginExtension {
    Wasm,
}

async fn plugins_from_dir(path: &Path) -> PluginStore<Period<Moment>> {
    let mut plugin_store = PluginStore::<Period<Moment>>::default();
    let wasm_plugin_builder = unwrap_or_exit(WasmPluginFactory::new());

    unwrap_or_exit(read_dir(path))
        .filter_map(|path| path.ok())
        .map(|path| path.path())
        .filter(|path| path.is_file())
        .for_each(|path| {
            let Some(Ok(extension)) = path
                .extension()
                .map(|ext| PluginExtension::from_str(&ext.to_string_lossy()))
            else {
                debug!(
                    path = path.to_string_lossy().to_string(),
                    "Not a plugin, unknown extension:"
                );

                return;
            };

            info!(path = path.to_string_lossy().to_string(), "Loading plugin");

            match extension {
                PluginExtension::Wasm => {
                    let wasm_plugin = unwrap_or_exit(wasm_plugin_builder.from_file(&path));
                    unwrap_or_exit(plugin_store.add(Box::new(wasm_plugin)));
                }
            }
        });

    plugin_store
}

fn init_tracing(verbose: bool) {
    let max_level = if verbose { Level::INFO } else { Level::ERROR };
    tracing_subscriber::fmt()
        .without_time()
        .with_target(false)
        .with_max_level(max_level)
        .init();
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();

    init_tracing(args.verbose);

    let (mut snapshot, plugins) = futures::join!(
        snapshot_from_yaml(&args.file),
        plugins_from_dir(&args.plugins),
    );

    snapshot.experience_repo = Arc::new(
        unwrap_or_exit(
            Arc::into_inner(snapshot.experience_repo).ok_or("could not bind experience repository"),
        )
        .with_entity_repo(snapshot.entity_repo.clone())
        .with_event_repo(snapshot.event_repo.clone()),
    );

    let entity_cli = EntityCli {
        entity_app: EntityApplication {
            entity_repo: snapshot.entity_repo.clone(),
        },
    };

    let event_cli = EventCli {
        event_app: EventApplication {
            event_repo: snapshot.event_repo.clone(),
        },
    };

    let experience_cli = ExperienceCli {
        experience_app: ExperienceApplication {
            experience_repo: snapshot.experience_repo.clone(),
            entity_repo: snapshot.entity_repo.clone(),
            event_repo: snapshot.event_repo.clone(),
            plugin_factory: Arc::new(plugins),
        },
    };

    unwrap_or_exit(match args.command {
        CliCommand::Entity(command) => entity_cli.execute(command).await,
        CliCommand::Event(command) => event_cli.execute(command).await,
        CliCommand::Experience(command) => experience_cli.execute(command).await,
    });

    snapshot_into_yaml(&args.file, &snapshot).await;
}
