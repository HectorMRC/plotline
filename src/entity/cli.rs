use super::{
    application::{EntityApplication, EntityRepository},
    fmt::EntityFmt,
    Entity,
};
use crate::{
    cli::{display_each_result, display_result, CliResult},
    id::Id,
};
use clap::{Args, Subcommand};
use std::io::{stdout, Write};

#[derive(Args)]
struct EntitySaveArgs {
    /// The name of the entity.
    name: String,
}

#[derive(Args)]
struct EntityRemoveArgs {
    /// The uuid of all the entities to be removed.
    ids: Vec<String>,
}

#[derive(Subcommand)]
#[clap(subcommand_negates_reqs = true, subcommand_precedence_over_arg = true)]
enum EntitySubCommand {
    /// Save an entity.
    Save(EntitySaveArgs),
    /// List all entities.
    #[command(alias("ls"))]
    List,
    /// Remove one or more entities.
    #[command(alias("rm"))]
    Remove(EntityRemoveArgs),
}

#[derive(Args)]
#[command(arg_required_else_help = true)]
pub struct EntityCommand {
    entity: Option<String>,
    #[command(subcommand)]
    command: Option<EntitySubCommand>,
}

impl<EntityRepo> EntityApplication<EntityRepo>
where
    EntityRepo: 'static + EntityRepository + Sync + Send,
{
    /// Given an [EntityCommand], executes the corresponding logic.
    pub fn execute(&self, entity_cmd: EntityCommand) -> CliResult {
        let entity_id = entity_cmd.entity.map(TryInto::try_into).transpose()?;
        if let Some(command) = entity_cmd.command {
            return self.execute_subcommand(command, entity_id);
        }

        let Some(entity_id) = entity_id else {
            return self.execute_subcommand(EntitySubCommand::List, None);
        };

        let entity = self.find_entity(entity_id).execute()?;
        print!("{}", EntityFmt::column(&entity));

        Ok(())
    }

    fn execute_subcommand(
        &self,
        subcommand: EntitySubCommand,
        entity_id: Option<Id<Entity>>,
    ) -> CliResult {
        match subcommand {
            EntitySubCommand::Save(args) => {
                let entity_id = entity_id.unwrap_or_default();
                self.save_entity(entity_id, args.name.try_into()?)
                    .execute()?;

                println!("{}", entity_id);
            }

            EntitySubCommand::List => {
                let entities = self.filter_entities().execute()?;

                let mut stdout = stdout().lock();
                writeln!(stdout, "{}", EntityFmt::headers())?;

                entities
                    .into_iter()
                    .try_for_each(|entity| write!(stdout, "{}", EntityFmt::row(&entity)))?;
            }

            EntitySubCommand::Remove(args) => {
                if let Some(entity_id) = entity_id {
                    display_result(self.remove_entity(entity_id).execute().map(|_| entity_id))?;
                } else {
                    display_each_result(args.ids.into_iter(), |id| {
                        let entity_id = id.try_into()?;
                        self.remove_entity(entity_id).execute().map(|_| entity_id)
                    })?;
                }
            }
        }

        Ok(())
    }
}
