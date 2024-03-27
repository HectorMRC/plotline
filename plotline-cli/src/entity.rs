use crate::{
    display::{display_each_result, display_result, DisplayMany, DisplaySingle},
    Result,
};
use clap::{Args, Subcommand};
use plotline::{
    entity::{
        application::{EntityApplication, EntityRepository},
        Entity,
    },
    id::Id,
};
use prettytable::row;

#[derive(Args)]
struct EntitySaveArgs {
    /// The name of the entity.
    #[arg(long, short)]
    name: Option<String>,
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
    /// The id of the entity.
    entity: Option<String>,
    /// The action to perform.
    #[command(subcommand)]
    command: Option<EntitySubCommand>,
}

pub struct EntityCli<EventRepo> {
    pub entity_app: EntityApplication<EventRepo>,
}

impl<EntityRepo> EntityCli<EntityRepo>
where
    EntityRepo: 'static + EntityRepository + Sync + Send,
{
    /// Given an [EntityCommand], executes the corresponding logic.
    pub async fn execute(&self, entity_cmd: EntityCommand) -> Result {
        let entity_id = entity_cmd.entity.map(TryInto::try_into).transpose()?;
        if let Some(command) = entity_cmd.command {
            return self.execute_subcommand(command, entity_id).await;
        }

        let Some(entity_id) = entity_id else {
            return self.execute_subcommand(EntitySubCommand::List, None).await;
        };

        let entity = self.entity_app.find_entity(entity_id).execute().await?;
        DisplaySingle::new(&entity, |table, entity| {
            table.add_row(row!["ID", entity.id]);
            table.add_row(row!["NAME", entity.name]);
        })
        .show();

        Ok(())
    }

    async fn execute_subcommand(
        &self,
        subcommand: EntitySubCommand,
        entity_id: Option<Id<Entity>>,
    ) -> Result {
        match subcommand {
            EntitySubCommand::Save(args) => {
                let entity_id = entity_id.unwrap_or_default();
                self.entity_app
                    .save_entity(entity_id)
                    .with_name(args.name.map(TryInto::try_into).transpose()?)
                    .execute()
                    .await?;

                println!("{}", entity_id);
            }

            EntitySubCommand::List => {
                let entities = self.entity_app.filter_entities().execute().await?;
                DisplayMany::new(&entities, |table, entity| {
                    table.add_row(row![&entity.id, &entity.name]);
                })
                .with_headers(vec!["ID", "NAME"])
                .show();
            }

            EntitySubCommand::Remove(args) => {
                if let Some(entity_id) = entity_id {
                    display_result(
                        self.entity_app
                            .remove_entity(entity_id)
                            .execute()
                            .await
                            .map(|_| entity_id),
                    );
                } else {
                    display_each_result(args.ids.into_iter(), |id| async {
                        let entity_id = id.try_into()?;
                        self.entity_app
                            .remove_entity(entity_id)
                            .execute()
                            .await
                            .map(|_| entity_id)
                    })
                    .await;
                }
            }
        }

        Ok(())
    }
}
