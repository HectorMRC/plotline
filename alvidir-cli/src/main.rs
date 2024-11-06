use std::{
    ffi::OsString,
    io,
    path::PathBuf,
    sync::{Arc, LazyLock},
};

use alvidir::{document::lazy::LazyDocument, graph::Graph};
use alvidir_cli::{repository::LocalDocumentRepository, CliCommand};
use clap::Parser;
use ignore::Walk;
use regex::Regex;
use tracing::Level;

/// Matches any filename ending with '.md'
const DEFAULT_FILE_PATTERN: &str = "^.*\\.md$";

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
    command: CliCommand,

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
    )]
    pattern: String,
}

impl Cli {
    fn documents(&self) -> impl Iterator<Item = PathBuf> + '_ {
        Walk::new(&self.context)
            .filter_map(move |entry| {
                if let Err(err) = &entry {
                    tracing::error!(
                        error = err.to_string(),
                        context = self.context.to_string_lossy().to_string(),
                        "walking base directory"
                    );
                }

                entry.ok()
            })
            .filter({
                let pattern =
                    Regex::new(&self.pattern).expect("pattern must be a valid regular expression");

                move |entry| {
                    let matches = pattern.is_match(&entry.file_name().to_string_lossy());
                    tracing::debug!(path = entry.path().to_string_lossy().to_string(), matches);

                    matches
                }
            })
            .filter_map(move |entry| {
                let path = entry
                    .path()
                    .strip_prefix(&self.context)
                    .map(ToOwned::to_owned);

                if let Err(err) = &path {
                    tracing::error!(
                        error = err.to_string(),
                        path = entry.path().to_string_lossy().to_string(),
                        context = self.context.to_string_lossy().to_string(),
                        "stripping context from path"
                    );
                }

                path.ok()
            })
    }
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();

    tracing_subscriber::fmt()
        .without_time()
        .with_target(false)
        .with_max_level(Level::INFO)
        .with_writer(io::stderr)
        .init();

    let document_repo = Arc::new(LocalDocumentRepository {
        context: &args.context,
    });

    let graph_app = Graph::from_iter(args.documents().map(LazyDocument::builder(document_repo)));
}
