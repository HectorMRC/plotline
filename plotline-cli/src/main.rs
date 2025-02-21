use std::{
    ffi::OsString,
    io,
    path::PathBuf,
    sync::{Arc, LazyLock},
};

use plotline::{graph::Graph, schema::Schema};
use plotline_cli::{document::DocumentCli, repository::LocalDocumentRepository, CliCommand};
use anyhow::Result;
use clap::Parser;
use tracing::Level;

static DEFAULT_EXTENSION: &str = "md";

static DEFAULT_CONTEXT: LazyLock<OsString> = LazyLock::new(|| {
    std::env::current_dir()
        .expect("current working directory")
        .into_os_string()
});

/// An astonishing graph-based docs manager.
#[derive(Parser)]
#[command(
    name = "plot",
    about = "An astonishing graph-based docs manager.",
    version = "0.1.0"
)]
struct Cli {
    #[command(subcommand)]
    subcommand: CliCommand,

    /// The base directory.
    #[arg(
        default_value = &*DEFAULT_CONTEXT,
        default_missing_value = "always",
        global = true,
        short = 'C',
        long
    )]
    context: PathBuf,

    /// The file's extension.
    #[arg(
        default_value = &*DEFAULT_EXTENSION,
        default_missing_value = "always",
        global = true,
        short,
        long
    )]
    extension: String,
}

#[allow(clippy::arc_with_non_send_sync)]
fn main() -> Result<()> {
    let args = Cli::parse();

    tracing_subscriber::fmt()
        .without_time()
        .with_target(false)
        .with_max_level(Level::INFO)
        .with_writer(io::stderr)
        .init();

    let document_repo = Arc::new(LocalDocumentRepository {
        context: args.context,
        extension: args.extension,
    });

    let graph = Graph::from_iter(document_repo.all());
    let schema = Arc::new(Schema::from(graph));

    let node_cli = DocumentCli {
        schema,
        document_repo,
    };

    match args.subcommand {
        CliCommand::Doc(command) => node_cli.execute(command),
    }
}
