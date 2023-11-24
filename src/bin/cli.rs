use clap::{error::ErrorKind, Parser, Subcommand};
use once_cell::sync::Lazy;
use plotline::{
    entity::{cli::EntityCommand, application::EntityApplication},
    event::{cli::EventCommand, application::EventApplication},
    snapshot::Snapshot,
};
use std::{
    ffi::OsString,
    fmt::Display,
    fs::{self, File},
    io::{BufReader, BufWriter, Write},
    path::Path,
};

const ENV_PLOTFILE: &str = "PLOTFILE";

static DEFAULT_PLOTFILE: Lazy<OsString> = Lazy::new(|| {
    dirs::home_dir()
        .unwrap_or_default()
        .join(".plotline/plotfile.yaml")
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

#[derive(Subcommand, strum_macros::Display)]
enum CliCommand {
    /// Manage entities.
    Entity(EntityCommand),
    /// Manage events.
    Event(EventCommand),
}

/// Returns the value of the result if, and only if, the result is OK. Otherwise prints the error and exits.
fn unwrap_or_exit<D, T, E>(msg: D, result: Result<T, E>) -> T
where
    D: Display,
    E: Display,
{
    match result {
        Err(error) => clap::Error::raw(ErrorKind::Io, format!("{msg}: {error}\n")).exit(),
        Ok(value) => value,
    }
}

fn main() {
    let args = Cli::parse();

    // Load data from YAML file
    let filepath = Path::new(&args.file);
    let snapshot = if filepath.exists() {
        Snapshot::parse(|| {
            let f = unwrap_or_exit(filepath.to_string_lossy(), File::open(filepath));
            let reader = BufReader::new(f);
            unwrap_or_exit("yaml reader", serde_yaml::from_reader(reader))
        })
    } else {
        Snapshot::default()
    };

    // Build dependencies
    let entity_srv = EntityApplication {
        entity_repo: snapshot.entities.clone(),
    };

    let event_srv = EventApplication {
        event_repo: snapshot.events.clone(),
    };

    // Execute command
    unwrap_or_exit(
        format!("{}", args.command),
        match args.command {
            CliCommand::Entity(command) => entity_srv.execute(command),
            CliCommand::Event(command) => event_srv.execute(command),
        },
    );

    // Persist data into YAML file
    if let Some(parent) = filepath.parent() {
        if !parent.exists() {
            unwrap_or_exit(parent.to_string_lossy(), fs::create_dir_all(parent));
        }
    }

    let f = unwrap_or_exit(filepath.to_string_lossy(), File::create(filepath));

    let mut writer = BufWriter::new(f);
    unwrap_or_exit("yaml writer", serde_yaml::to_writer(&mut writer, &snapshot));
    unwrap_or_exit("io writer", writer.flush());
}
