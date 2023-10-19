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

fn main() {
    let args = Cli::parse();

    let f = File::open(args.file).unwrap();
    let snapshot = Snapshot::parse(|| {
        let reader = BufReader::new(&f);
        serde_yaml::from_reader(reader).unwrap()
    });

    let entity_srv = EntityService {
        entity_repo: snapshot.entities.clone(),
    };

    let command_result = match args.command {
        CliCommand::Entity(command) => entity_srv.execute(command),
    };

    if let Err(error) = command_result {
        let mut stderr = clap::Error::new(ErrorKind::Io);
        stderr.insert(ContextKind::Custom, ContextValue::String(error.to_string()));
        stderr.exit();
    }

    let mut writer = BufWriter::new(f);
    writer
        .write_all(serde_yaml::to_string(&snapshot).unwrap().as_bytes())
        .unwrap();

    writer.flush().unwrap();
}
