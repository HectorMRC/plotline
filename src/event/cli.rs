use super::{
    application::{EventApplication, EventRepository},
    Event,
};
use crate::{
    cli::{CliError, CliResult},
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
    #[arg(long, short, use_value_delimiter = true, value_delimiter = ',')]
    entities: Option<Vec<String>>,
}

#[derive(Subcommand)]
#[clap(subcommand_negates_reqs = true, subcommand_precedence_over_arg = true)]
enum EventSubCommand {
    /// Save an event.
    Save(EventSaveArgs),
}

#[derive(Args)]
#[command(arg_required_else_help = true)]
pub struct EventCommand {
    /// The id of the event.
    event: Option<String>,
    /// The action to perform.
    #[command(subcommand)]
    command: Option<EventSubCommand>,
}

impl<EventRepo> EventApplication<EventRepo>
where
    EventRepo: 'static + EventRepository + Sync + Send,
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
                let event_id = event_id.unwrap_or_default();
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
        }

        Ok(())
    }
}
