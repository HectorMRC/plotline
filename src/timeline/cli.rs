use super::service::{TimelineRepository, TimelineService};
use crate::cli::CliResult;
use clap::{Args, Subcommand};

#[derive(Args)]
struct TimelineCreateArgs {
    /// The name of the timeline.
    #[arg(num_args(1..))]
    name: Vec<String>,
    /// The uuid string of the timeline.
    #[arg(short, long)]
    id: Option<String>,
}

#[derive(Subcommand)]
enum TimelineSubCommand {
    /// Create a new timeline.
    Create(TimelineCreateArgs),
}

#[derive(Args)]
#[command(arg_required_else_help = true)]
pub struct TimelineCommand {
    #[command(subcommand)]
    command: TimelineSubCommand,
}

impl<R> TimelineService<R>
where
    R: 'static + TimelineRepository + Sync + Send,
{
    /// Given a [TimelineCommand], executes the corresponding logic.
    pub fn execute(&self, timeline_cmd: TimelineCommand) -> CliResult {
        match timeline_cmd.command {
            TimelineSubCommand::Create(args) => {
                let timeline = self
                    .create_timeline(args.name.join(" ").try_into()?)
                    .with_id(args.id.map(TryInto::try_into).transpose()?)
                    .execute()?;

                println!("{}", timeline.id);
            }
        }

        Ok(())
    }
}
