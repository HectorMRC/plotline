use super::service::{EventRepository, EventService};
use crate::{
    cli::{display_each_result, CliError, CliResult},
    entity::service::EntityRepository,
};
use clap::{Args, Subcommand};

#[derive(Args)]
struct EventCreateArgs {
    /// The name of the event.
    name: String,
    /// The period during which the event takes place.
    #[arg(required = true, num_args(1..=2))]
    interval: Vec<String>,
    /// The uuid string of the event.
    #[arg(short, long)]
    id: Option<String>,
}

#[derive(Args)]
struct EventAddEntitiesArgs {
    /// The ids of all the entities to be added.
    #[arg(required = true, num_args(1..))]
    entities: Vec<String>,
    /// The id of the target event.
    event: String,
}

#[derive(Subcommand)]
enum EventEntitiesSubCommand {
    /// Add a new entity into the event.
    Add(EventAddEntitiesArgs),
}

#[derive(Args)]
#[command(arg_required_else_help = true)]
struct EventEntitiesCommand {
    #[command(subcommand)]
    command: EventEntitiesSubCommand,
}

#[derive(Subcommand)]
enum EventSubCommand {
    /// Create a new event.
    Create(EventCreateArgs),
    /// Manage event entities.
    Entities(EventEntitiesCommand),
}

#[derive(Args)]
#[command(arg_required_else_help = true)]
pub struct EventCommand {
    /// The action to perform.
    #[command(subcommand)]
    command: EventSubCommand,
}

impl<R, E> EventService<R, E>
where
    R: 'static + EventRepository + Sync + Send,
    E: 'static + EntityRepository + Sync + Send,
    R::Interval: TryFrom<Vec<String>> + Sync + Send,
    <R::Interval as TryFrom<Vec<String>>>::Error: Into<CliError>,
{
    /// Given a [EventCommand], executes the corresponding logic.
    pub fn execute(&self, event_cmd: EventCommand) -> CliResult {
        match event_cmd.command {
            EventSubCommand::Create(args) => {
                let event = self
                    .create_event(
                        args.name.try_into()?,
                        args.interval.try_into().map_err(Into::into)?,
                    )
                    .with_id(args.id.map(TryInto::try_into).transpose()?)
                    .execute()?;

                println!("{}", event.id);
            }
            EventSubCommand::Entities(args) => match args.command {
                EventEntitiesSubCommand::Add(args) => {
                    let event_id = args.event.try_into()?;
                    display_each_result(args.entities.into_iter(), |entity| {
                        let entity_id = entity.try_into()?;
                        self.add_entity(entity_id, event_id)
                            .execute()
                            .map(|_| entity_id)
                    })?;
                }
            },
        }

        Ok(())
    }
}
