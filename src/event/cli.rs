use super::service::{EventRepository, EventService};
use crate::cli::{CliError, CliResult};
use clap::{Args, Subcommand};

#[derive(Args)]
struct EventCreateArgs {
    /// The name of the event.
    #[arg(short, long, num_args(1..), required = true)]
    name: Vec<String>,
    /// The period during which the event takes place.
    #[arg(short, long, num_args(1..), required = true)]
    period: Vec<String>,
    /// The uuid string of the event.
    #[arg(short, long)]
    id: Option<String>,
}

#[derive(Subcommand)]
enum EventSubCommand {
    /// Create a new event.
    Create(EventCreateArgs),
}

#[derive(Args)]
#[command(arg_required_else_help = true)]
pub struct EventCommand {
    #[command(subcommand)]
    command: EventSubCommand,
}

impl<R> EventService<R>
where
    R: 'static + EventRepository + Sync + Send,
    R::Interval: TryFrom<String>,
    <R::Interval as TryFrom<String>>::Error: Into<CliError>,
{
    /// Given a [EventCommand], executes the corresponding logic.
    pub fn execute(&self, event_cmd: EventCommand) -> CliResult {
        match event_cmd.command {
            EventSubCommand::Create(args) => {
                let event = self
                    .create_event(
                        args.name.join(" ").try_into()?,
                        args.period.join(" ").try_into().map_err(Into::into)?,
                    )
                    .with_id(args.id.map(TryInto::try_into).transpose()?)
                    .execute()?;

                println!("{}", event.id);
            }
        }

        Ok(())
    }
}
