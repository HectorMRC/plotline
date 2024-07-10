use std::{ffi::OsString, io, path::PathBuf, str::FromStr, sync::Arc};

use alvidir::{
    document::{
        proxy::{DocumentProxy, ProxyTrigger},
        Document,
    },
    graph::{application::GraphApplication, directed::DirectedGraph},
    name::Name,
};
use alvidir_cli::{
    document::DocumentCli,
    node::NodeCli,
    CliCommand,
    repository::LocalDocumentRepository
};
use clap::Parser;
use ignore::{DirEntry, Walk};
use once_cell::sync::Lazy;
use regex::Regex;
use tracing::{error, info, warn, Level};

const ENV_CONTEXT: &str = "CONTEXT";
const ENV_FILE_REGEX: &str = "FILE_REGEX";

/// Matches any filename ending with '.md'
const DEFAULT_FILE_REGEX: &str = "^.*\\.md$";

static DEFAULT_CONTEXT_PATH: Lazy<OsString> = Lazy::new(|| {
    std::env::current_dir()
        .expect("current working directory")
        .into_os_string()
});

/// A plotline manager.
#[derive(Parser)]
#[command(name = "alvidir", about = "A plotline manager.", version = "0.0.1")]
struct Cli {
    #[command(subcommand)]
    command: CliCommand,

    /// The base directory.
    #[arg(
        env = &*ENV_CONTEXT,
        default_value = &*DEFAULT_CONTEXT_PATH,
        default_missing_value = "always",
        global = true,
        short = 'C', long
    )]
    context: PathBuf,

    /// The filename regex.
    #[arg(
        env = &*ENV_FILE_REGEX,
        default_value = &*DEFAULT_FILE_REGEX,
        default_missing_value = "always",
        global = true,
        long = "regex"
    )]
    file_regex: String,

    /// Print info logs though stderr.
    #[arg(global = true, short, long)]
    verbose: bool,
}

fn init_tracing(verbose: bool) {
    let max_level = if verbose { Level::INFO } else { Level::ERROR };
    tracing_subscriber::fmt()
        .without_time()
        .with_target(false)
        .with_max_level(max_level)
        .with_writer(io::stderr)
        .init();
}

fn errorless_entry(args: &Cli) -> impl Fn(Result<DirEntry, ignore::Error>) -> Option<DirEntry> {
    let context = args.context.to_string_lossy().to_string();
    move |entry| {
        if let Err(err) = &entry {
            error!(error = err.to_string(), context, "walking base directory")
        }

        entry.ok()
    }
}

fn entry_matching_regex(args: &Cli) -> impl Fn(&DirEntry) -> bool {
    let file_regex = Regex::new(&args.file_regex).expect("file_regex must compile");
    move |entry| {
        let matches = file_regex.is_match(&entry.file_name().to_string_lossy());
        if matches {
            info!(
                path = entry.path().to_string_lossy().to_string(),
                "selected file"
            );
        } else {
            warn!(
                path = entry.path().to_string_lossy().to_string(),
                "ignored file"
            );
        }

        matches
    }
}

fn entry_into_name<T>(args: &Cli) -> impl Fn(DirEntry) -> Option<Name<T>> {
    let base = args.context.clone();

    move |entry| {
        let file_path = match entry.path().strip_prefix(&base) {
            Ok(path) => path.to_string_lossy(),
            Err(err) => {
                error!(error = err.to_string());
                return None;
            }
        };

        let file_name = Name::from_str(&file_path);
        if let Err(error) = &file_name {
            warn!(error = error.to_string(), path = file_path.to_string());
        }

        file_name.ok()
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Cli::parse();

    init_tracing(args.verbose);

    let document_repo = Arc::new(LocalDocumentRepository {
        context: &args.context,
    });

    // TODO: use proper insertion with contrain checking
    let graph_app = Arc::new(GraphApplication {
        graph: DirectedGraph::from_iter(
            Walk::new(&args.context)
                .into_iter()
                .filter_map(errorless_entry(&args))
                .filter(entry_matching_regex(&args))
                .filter_map(entry_into_name(&args))
                .map(Document::new)
                .map(DocumentProxy::<_, AlwaysTrigger>::builder(document_repo)),
        ),
    });

    let document_cli = DocumentCli {
        graph_app: graph_app.clone(),
    };

    let node_cli = NodeCli {
        graph_app: graph_app,
    };

    match args.command {
        CliCommand::Document(command) => document_cli.execute(command).await,
        CliCommand::Node(command) => node_cli.execute(command).await,
    }
}

#[derive(Default)]
struct AlwaysTrigger;

impl ProxyTrigger for AlwaysTrigger {
    fn update(&self) -> bool {
        true
    }
}
