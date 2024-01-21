use super::{
    application::{ConstraintFactory, ExperienceApplication, ExperienceRepository},
    Profile,
};
use crate::{
    cli::{CliError, CliResult},
    entity::{application::EntityRepository, Entity},
    event::{application::EventRepository, Event},
    id::{Id, Result as IdResult},
};
use clap::{Args, Subcommand};

#[derive(Args)]
struct ExperienceSaveArgs {
    /// The id of the entities resulting from the experience.
    #[clap(short, long, value_delimiter = ',')]
    after: Vec<String>,
    /// Mark the experience as terminal.
    #[clap(long, conflicts_with = "after")]
    terminal: bool,
}

#[derive(Subcommand)]
#[clap(subcommand_negates_reqs = true, subcommand_precedence_over_arg = true)]
enum ExperienceSubCommand {
    /// Save an experience.
    Save(ExperienceSaveArgs),
}

#[derive(Args)]
#[command(arg_required_else_help = true)]
pub struct ExperienceCommand {
    /// The id of the experience.
    #[clap(value_parser, num_args = 2, value_delimiter = ' ')]
    experience: Option<Vec<String>>,
    /// The action to perform.
    #[command(subcommand)]
    command: Option<ExperienceSubCommand>,
}

impl<ExperienceRepo, EntityRepo, EventRepo, CnstFactory>
    ExperienceApplication<ExperienceRepo, EntityRepo, EventRepo, CnstFactory>
where
    ExperienceRepo: 'static + ExperienceRepository<Interval = EventRepo::Interval> + Sync + Send,
    EntityRepo: 'static + EntityRepository + Sync + Send,
    EventRepo: 'static + EventRepository + Sync + Send,
    CnstFactory: 'static + ConstraintFactory<EventRepo::Interval> + Sync + Send,
{
    /// Given an [ExperienceCommand], executes the corresponding logic.
    pub fn execute(&self, experience_cmd: ExperienceCommand) -> CliResult {
        let (entity_id, event_id) = experience_cmd
            .experience
            .map(Vec::into_iter)
            .map(|mut id_components| -> IdResult<_> {
                Ok((
                    id_components.next().unwrap_or_default().try_into()?,
                    id_components.next().unwrap_or_default().try_into()?,
                ))
            })
            .transpose()?
            .ok_or(CliError::MissingArgument("experience id"))?;

        if let Some(command) = experience_cmd.command {
            return self.execute_subcommand(command, entity_id, event_id);
        }

        todo!()
    }

    fn execute_subcommand(
        &self,
        subcommand: ExperienceSubCommand,
        entity_id: Id<Entity>,
        event_id: Id<Event<EventRepo::Interval>>,
    ) -> CliResult {
        match subcommand {
            ExperienceSubCommand::Save(args) => {
                self.save_experience(entity_id, event_id).with_after(
                    args.terminal
                        .then_some(Ok(Vec::default()))
                        .or_else(|| {
                            if args.after.is_empty() {
                                return None;
                            }

                            Some(
                                args.after
                                    .into_iter()
                                    .map(|entity_id| Ok(Profile::new(entity_id.try_into()?)))
                                    .collect::<IdResult<Vec<_>>>(),
                            )
                        })
                        .transpose()?,
                );
                
                println!("{} {}", entity_id, event_id);
            }
        }
        
        Ok(())
    }
}
