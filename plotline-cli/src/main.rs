use clap::{error::ErrorKind, Parser};
use futures::executor::block_on;
use once_cell::sync::Lazy;
use plotline::{
    entity::{application::EntityApplication, repository::InMemoryEntityRepository},
    event::{application::EventApplication, repository::InMemoryEventRepository},
    experience::{application::ExperienceApplication, repository::InMemoryExperienceRepository},
    period::Period,
};
use plotline_cli::{entity::EntityCli, event::EventCli, experience::ExperienceCli, CliCommand};
use plugin::PluginStore;
use serde::{Deserialize, Serialize};
use std::{
    ffi::OsString,
    fmt::Display,
    fs::{File, OpenOptions},
    io::{BufReader, BufWriter, Write},
    path::Path,
    sync::Arc,
};

const ENV_PLOTFILE: &str = "PLOTFILE";

static DEFAULT_PLOTFILE: Lazy<OsString> = Lazy::new(|| {
    dirs::home_dir()
        .unwrap_or_default()
        .join(".plotline")
        .join("plotfile.yaml")
        .into_os_string()
});

/// Implements the [Serialize] and [Deserialize] traits to persist and recover
/// the state of the repositories.
#[derive(Default, Serialize, Deserialize)]
pub struct Snapshot {
    #[serde(flatten)]
    entity_repo: Arc<InMemoryEntityRepository>,
    #[serde(flatten)]
    event_repo: Arc<InMemoryEventRepository<Period<usize>>>,
    #[serde(flatten)]
    experience_repo: Arc<InMemoryExperienceRepository<Period<usize>>>,
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
        default_value = &*DEFAULT_PLOTFILE,
        default_missing_value = "always",
        global = true,
        short, long
    )]
    file: OsString,
}

/// Returns the value of the result if, and only if, the result is OK.
/// Otherwise prints the error and exits.
#[inline]
fn unwrap_or_exit<T, E>(result: Result<T, E>) -> T
where
    E: Display,
{
    match result {
        Err(error) => clap::Error::raw(ErrorKind::Io, format!("{error}\n")).exit(),
        Ok(value) => value,
    }
}

fn main() {
    let args = Cli::parse();

    // Load data from YAML file
    let filepath = Path::new(&args.file);
    let mut snapshot = if filepath.exists() {
        let f = unwrap_or_exit(File::open(filepath));
        let reader = BufReader::new(f);
        unwrap_or_exit(serde_yaml::from_reader(reader))
    } else {
        Snapshot::default()
    };

    // Load plugins
    let plugin_factory = Arc::new(PluginStore::<Period<usize>>::default());

    // Inject dependencies
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
            plugin_factory,
        },
    };

    // Execute command
    block_on(async {
        unwrap_or_exit(match args.command {
            CliCommand::Entity(command) => entity_cli.execute(command).await,
            CliCommand::Event(command) => event_cli.execute(command).await,
            CliCommand::Experience(command) => experience_cli.execute(command).await,
        })
    });

    // Persist data into YAML file
    let f = unwrap_or_exit(
        OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(filepath),
    );

    let mut writer = BufWriter::new(f);
    unwrap_or_exit(serde_yaml::to_writer(&mut writer, &snapshot));
    unwrap_or_exit(writer.flush());
}
