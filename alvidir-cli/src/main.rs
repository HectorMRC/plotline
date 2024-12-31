use std::{
    ffi::OsString,
    io,
    path::PathBuf,
    sync::{Arc, LazyLock},
};

use alvidir::{graph::Graph, schema::Schema};
use alvidir_cli::{document::DocumentCli, repository::LocalDocumentRepository, CliCommand};
use anyhow::Result;
use clap::Parser;
use regex::Regex;
use tracing::Level;

/// Matches any filename.
const DEFAULT_FILE_PATTERN: &str = ".*";

static DEFAULT_CONTEXT_PATH: LazyLock<OsString> = LazyLock::new(|| {
    std::env::current_dir()
        .expect("current working directory")
        .into_os_string()
});

/// An astonishing graph-based docs manager.
#[derive(Parser)]
#[command(
    name = "alvidir",
    about = "An astonishing graph-based docs manager.",
    version = "0.0.1"
)]
struct Cli {
    #[command(subcommand)]
    subcommand: CliCommand,

    /// The base directory.
    #[arg(
        default_value = &*DEFAULT_CONTEXT_PATH,
        default_missing_value = "always",
        global = true,
        short = 'C',
        long
    )]
    context: PathBuf,

    /// The filename pattern.
    #[arg(
        default_value = &*DEFAULT_FILE_PATTERN,
        default_missing_value = "always",
        global = true,
        short,
        long
    )]
    pattern: String,
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
        pattern: Regex::new(&args.pattern).expect("pattern should be a valid regular expression"),
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
