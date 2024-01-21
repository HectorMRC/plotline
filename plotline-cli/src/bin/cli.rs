use clap::{error::ErrorKind, Parser, Subcommand};
use once_cell::sync::Lazy;
use plotline::{
    entity::application::EntityApplication,
    event::application::EventApplication,
    experience::{
        application::{ConstraintFactory, ExperienceApplication},
        constraint::{Constraint, LiFoConstraintChain},
        ExperiencedEvent,
    },
    interval::Interval,
    snapshot::Snapshot,
};
use plotline_cli::{
    entity::{EntityCli, EntityCommand},
    event::{EventCli, EventCommand},
    experience::{ExperienceCli, ExperienceCommand},
};
use std::{
    ffi::OsString,
    fmt::Display,
    fs,
    io::{BufReader, BufWriter, Write},
    marker::PhantomData,
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
    /// Manage experiences.
    Experience(ExperienceCommand),
}

impl<Intv> ConstraintFactory<Intv> for CliCommand
where
    Intv: Interval,
{
    fn new<'a>(experienced_event: &'a ExperiencedEvent<'a, Intv>) -> impl Constraint<'a, Intv> {
        LiFoConstraintChain::with_defaults(experienced_event)
    }
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
            let f = unwrap_or_exit(filepath.to_string_lossy(), fs::File::open(filepath));
            let reader = BufReader::new(f);
            unwrap_or_exit("yaml reader", serde_yaml::from_reader(reader))
        })
    } else {
        Snapshot::default()
    };

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
            cnst_factory: PhantomData::<CliCommand>,
        },
    };

    // Execute command
    unwrap_or_exit(
        format!("{}", args.command),
        match args.command {
            CliCommand::Entity(command) => entity_cli.execute(command),
            CliCommand::Event(command) => event_cli.execute(command),
            CliCommand::Experience(command) => experience_cli.execute(command),
        },
    );

    // Persist data into YAML file
    if let Some(parent) = filepath.parent() {
        if !parent.exists() {
            unwrap_or_exit(parent.to_string_lossy(), fs::create_dir_all(parent));
        }
    }

    let f = unwrap_or_exit(filepath.to_string_lossy(), fs::File::create(filepath));

    let mut writer = BufWriter::new(f);
    unwrap_or_exit("yaml writer", serde_yaml::to_writer(&mut writer, &snapshot));
    unwrap_or_exit("io writer", writer.flush());
}
