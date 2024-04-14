use crate::{display::DisplayTable, Error, Result};
use clap::{Args, Subcommand};
use plotline::{
    event::{
        application::{EventApplication, EventRepository},
        Event,
    },
    id::Id,
    interval::{Bound, Interval, IntervalFactory},
};
use prettytable::row;
use std::{fmt::Display, str::FromStr};

fn parse_interval<Intv>(bounds: Vec<String>) -> std::result::Result<Intv, Error>
where
    Intv: IntervalFactory,
    Intv::Bound: TryFrom<String>,
    <Intv::Bound as TryFrom<String>>::Error: Display,
{
    let parse_boundary = |value: String| -> std::result::Result<Intv::Bound, Error> {
        let bound: std::result::Result<Intv::Bound, _> = value.try_into();
        bound.map_err(|err| Error::ParseInterval(err.to_string()))
    };

    let mut bounds = bounds.into_iter();
    let lo = parse_boundary(bounds.next().unwrap_or_default())?;

    let hi = bounds
        .next()
        .map(parse_boundary)
        .transpose()?
        .unwrap_or(lo.clone());

    Ok(Intv::new(lo, hi))
}

#[derive(Args)]
struct EventSaveArgs {
    /// The name of the event.
    #[arg(long, short)]
    name: Option<String>,
    /// The period during which the event takes place.
    #[arg(long, short, num_args(1..=2))]
    interval: Option<Vec<String>>,
}

#[derive(Subcommand)]
#[clap(subcommand_negates_reqs = true, subcommand_precedence_over_arg = true)]
enum EventSubCommand {
    /// Save an event.
    Save(EventSaveArgs),
    /// List all events.
    #[command(alias("ls"))]
    List,
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

pub struct EventCli<EventRepo> {
    pub event_app: EventApplication<EventRepo>,
}

impl<EventRepo> EventCli<EventRepo>
where
    EventRepo: 'static + EventRepository + Sync + Send,
    EventRepo::Intv: IntervalFactory + Sync + Send,
    <EventRepo::Intv as IntervalFactory>::Bound: TryFrom<String> + Bound,
    <<EventRepo::Intv as IntervalFactory>::Bound as TryFrom<String>>::Error: Display,
    <EventRepo::Intv as Interval>::Bound: Display,
{
    /// Given a [EventCommand], executes the corresponding logic.
    pub async fn execute(&self, event_cmd: EventCommand) -> Result {
        let event_id = event_cmd.event.map(TryInto::try_into).transpose()?;
        if let Some(command) = event_cmd.command {
            return self.execute_subcommand(command, event_id).await;
        }

        let Some(event_id) = event_id else {
            return self.execute_subcommand(EventSubCommand::List, None).await;
        };

        let event = self.event_app.find_event(event_id).execute().await?;
        DisplayTable::new(&event).show(|table, event| {
            table.add_row(row!["ID", event.id]);
            table.add_row(row!["NAME", event.name]);
            table.add_row(row!["START", event.interval.lo()]);
            table.add_row(row!["END", event.interval.hi()]);
        });

        Ok(())
    }

    async fn execute_subcommand(
        &self,
        subcommand: EventSubCommand,
        event_id: Option<Id<Event<EventRepo::Intv>>>,
    ) -> Result {
        match subcommand {
            EventSubCommand::Save(args) => {
                let event_id = event_id.unwrap_or_default();
                self.event_app
                    .save_event(event_id)
                    .with_name(args.name.as_deref().map(FromStr::from_str).transpose()?)
                    .with_interval(args.interval.map(parse_interval).transpose()?)
                    .execute()
                    .await?;

                println!("{}", event_id);
            }
            EventSubCommand::List => {
                let events = self.event_app.filter_events().execute().await?;
                DisplayTable::new(&events).show(|table, events| {
                    table.add_row(row!["ID", "NAME", "START", "END"]);
                    events.iter().for_each(|event| {
                        table.add_row(row![
                            &event.id,
                            &event.name,
                            &event.interval.lo(),
                            &event.interval.hi()
                        ]);
                    })
                });
            }
        }

        Ok(())
    }
}
