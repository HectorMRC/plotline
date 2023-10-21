use clap::{error::ErrorKind, Parser, Subcommand};
use once_cell::sync::Lazy;
use plotline::{
    entity::{cli::EntityCommand, service::EntityService},
    snapshot::Snapshot,
};
use std::{
    ffi::OsString,
    fmt::Display,
    fs::File,
    io::{BufReader, BufWriter, Write},
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

    /// The data source file
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
    /// Manage entities
    Entity(EntityCommand),
}

/// Returns the value of the result if, and only if, the result is OK. Otherwise prints the error and exits.
fn unwrap_or_exit<D, T, E>(msg: D, result: Result<T, E>) -> T
where
    D: Display,
    E: Display,
{
    match result {
        Ok(value) => value,
        Err(error) => clap::Error::raw(ErrorKind::Io, format!("{msg}: {error}\n")).exit(),
    }
}

fn main() {
    let args = Cli::parse();

    let filepath = format!("{:?}", &args.file);
    let snapshot = Snapshot::parse(|| {
        let f = unwrap_or_exit(&filepath, File::open(&args.file));
        let reader = BufReader::new(f);
        unwrap_or_exit("yaml reader", serde_yaml::from_reader(reader))
    });

    let entity_srv = EntityService {
        entity_repo: snapshot.entities.clone(),
    };

    unwrap_or_exit(
        format!("{}", args.command),
        match args.command {
            CliCommand::Entity(command) => entity_srv.execute(command),
        },
    );

    let f = unwrap_or_exit(&filepath, File::create(&args.file));
    let mut writer = BufWriter::new(f);
    unwrap_or_exit("yaml writer", serde_yaml::to_writer(&mut writer, &snapshot));
    unwrap_or_exit("io writer", writer.flush());
}
