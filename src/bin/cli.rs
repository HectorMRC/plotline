use clap::{
    error::{ContextKind, ContextValue, ErrorKind},
    Parser, Subcommand,
};
use plotline::{
    entity::{cli::EntityCommand, service::EntityService},
    snapshot::Snapshot,
};
use std::{
    ffi::OsString,
    fs::File,
    io::{BufReader, BufWriter, Write},
};

const DEFAULT_PLOTFILE: &str = "~/.plotline/plotfile.yaml";
const ENV_PLOTFILE: &str = "PLOTFILE";

/// A plotline manager.
#[derive(Parser)]
#[command(name = "plot", about = "A plotline manager.", version = "0.0.1")]
struct Cli {
    #[command(subcommand)]
    command: CliCommand,

    /// The data source file
    #[arg(
        env = ENV_PLOTFILE,
        default_value = DEFAULT_PLOTFILE,
        default_missing_value = "always",
        global = true,
        short, long
    )]
    file: OsString,
}

#[derive(Subcommand)]
enum CliCommand {
    /// Manage entities
    Entity(EntityCommand),
}

/// Returns the value of the result if, and only if, the result is OK. Otherwise prints the error and exits.
fn unwrap_or_exit<T, E>(result: Result<T, E>) -> T
where
    E: ToString,
{
    match result {
        Ok(value) => value,
        Err(error) => {
            let mut cli_error = clap::Error::new(ErrorKind::Io);
            cli_error.insert(ContextKind::Custom, ContextValue::String(error.to_string()));
            cli_error.exit();
        }
    }
}

fn main() {
    let args = Cli::parse();

    let f = File::open(&args.file).unwrap();
    let snapshot = unwrap_or_exit(Snapshot::parse(|| {
        let reader = BufReader::new(&f);
        serde_yaml::from_reader(reader)
    }));

    let entity_srv = EntityService {
        entity_repo: snapshot.entities.clone(),
    };

    unwrap_or_exit(match args.command {
        CliCommand::Entity(command) => entity_srv.execute(command),
    });

    let mut writer = BufWriter::new(f);
    unwrap_or_exit(writer.write_all(serde_yaml::to_string(&snapshot).unwrap().as_bytes()));
    unwrap_or_exit(writer.flush());
}
