use super::{
    service::{EventRepository, EventService},
    Event,
};
use crate::{
    cli::{display_each_result, CliError, CliResult},
    entity::service::EntityRepository,
    id::Id,
};
use clap::{Args, Subcommand};

#[derive(Args)]
struct EventSaveArgs {
    /// The name of the event.
    #[arg(long, short)]
    name: Option<String>,
    /// The period during which the event takes place.
    #[arg(long, short, num_args(1..=2))]
    interval: Option<Vec<String>>,
    /// The ids of all the entities implicated in the event.
    #[arg(long, short)]
    entities: Option<Vec<String>>,
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
#[clap(subcommand_negates_reqs = true, subcommand_precedence_over_arg = true)]
enum EventSubCommand {
    /// Save an event.
    Save(EventSaveArgs),
    /// Manage event entities.
    Entities(EventEntitiesCommand),
}

#[derive(Args)]
#[command(arg_required_else_help = true)]
pub struct EventCommand {
    event: Option<String>,
    #[command(subcommand)]
    command: Option<EventSubCommand>,
}

impl<EventRepo, EntityRepo> EventService<EventRepo, EntityRepo>
where
    EventRepo: 'static + EventRepository + Sync + Send,
    EntityRepo: 'static + EntityRepository + Sync + Send,
    EventRepo::Interval: TryFrom<Vec<String>> + Sync + Send,
    <EventRepo::Interval as TryFrom<Vec<String>>>::Error: Into<CliError>,
{
    /// Given a [EventCommand], executes the corresponding logic.
    pub fn execute(&self, event_cmd: EventCommand) -> CliResult {
        let event_id = event_cmd.event.map(TryInto::try_into).transpose()?;
        if let Some(command) = event_cmd.command {
            return self.execute_subcommand(command, event_id);
        }

        Ok(())
    }

    fn execute_subcommand(
        &self,
        subcommand: EventSubCommand,
        event_id: Option<Id<Event<EventRepo::Interval>>>,
    ) -> CliResult {
        match subcommand {
            EventSubCommand::Save(args) => {
                let event_id = event_id.unwrap_or_else(|| Id::new());
                self.save_event(event_id)
                    .with_name(args.name.map(TryInto::try_into).transpose()?)
                    .with_interval(
                        args.interval
                            .map(TryInto::try_into)
                            .transpose()
                            .map_err(Into::into)?,
                    )
                    .execute()?;

                println!("{}", event_id);
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
