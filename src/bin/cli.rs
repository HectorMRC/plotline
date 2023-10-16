use clap::{Parser, Subcommand};
use plotline::entity::{
    cli::EntityCommand, repository::InMemoryEntityRepository, service::EntityService,
};
use std::{ffi::OsString, sync::Arc};

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
    let entity_repo = Arc::new(InMemoryEntityRepository::default());
    let entity_srv = EntityService { entity_repo };

    let args = Cli::parse();

    let command_result = match args.command {
        CliCommand::Entity(command) => entity_srv.run(command),
    };
}
