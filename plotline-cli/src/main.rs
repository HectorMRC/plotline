use clap::{error::ErrorKind, Parser};
use once_cell::sync::Lazy;
use plotline::{
    entity::application::EntityApplication, event::application::EventApplication,
    experience::application::ExperienceApplication, period::Period, snapshot::Snapshot,
};
use plotline_cli::{entity::EntityCli, event::EventCli, experience::ExperienceCli, CliCommand};
use plugin::PluginStore;
use std::{
    ffi::OsString,
    fmt::Display,
    fs::{self, OpenOptions},
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
    let snapshot = if filepath.exists() {
        Snapshot::parse(|| {
            let f = unwrap_or_exit(fs::File::open(filepath));
            let reader = BufReader::new(f);
            unwrap_or_exit(serde_yaml::from_reader(reader))
        })
    } else {
        Snapshot::default()
    };

    // Load plugins
    let plugin_store = Arc::new(PluginStore::<Period<usize>>::default());

    // Build dependencies
    let entity_cli = EntityCli {
        entity_app: EntityApplication {
            entity_repo: snapshot.entities.clone(),
        },
    };

    let event_cli = EventCli {
        event_app: EventApplication {
            event_repo: snapshot.events.clone(),
        },
    };

    let experience_cli = ExperienceCli {
        experience_app: ExperienceApplication {
            experience_repo: snapshot.experiences.clone(),
            entity_repo: snapshot.entities.clone(),
            event_repo: snapshot.events.clone(),
            plugin_factory: plugin_store.clone(),
        },
    };

    // Execute command
    unwrap_or_exit(match args.command {
        CliCommand::Entity(command) => entity_cli.execute(command),
        CliCommand::Event(command) => event_cli.execute(command),
        CliCommand::Experience(command) => experience_cli.execute(command),
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
