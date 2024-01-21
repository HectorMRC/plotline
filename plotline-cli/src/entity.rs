use crate::{display_each_result, display_result, CliResult};
use clap::{Args, Subcommand};
use plotline::{
    entity::{
        application::{EntityApplication, EntityRepository},
        Entity,
    },
    id::Id,
};
use std::io::{stdout, Write};
use std::{fmt::Display, marker::PhantomData};

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
    pub fn execute(&self, entity_cmd: EntityCommand) -> CliResult {
        let entity_id = entity_cmd.entity.map(TryInto::try_into).transpose()?;
        if let Some(command) = entity_cmd.command {
            return self.execute_subcommand(command, entity_id);
        }

        let Some(entity_id) = entity_id else {
            return self.execute_subcommand(EntitySubCommand::List, None);
        };

        let entity = self.entity_app.find_entity(entity_id).execute()?;
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
                self.entity_app
                    .save_entity(entity_id, args.name.try_into()?)
                    .execute()?;

                println!("{}", entity_id);
            }

            EntitySubCommand::List => {
                let entities = self.entity_app.filter_entities().execute()?;

                let mut stdout = stdout().lock();
                writeln!(stdout, "{}", EntityFmt::headers())?;

                entities
                    .into_iter()
                    .try_for_each(|entity| write!(stdout, "{}", EntityFmt::row(&entity)))?;
            }

            EntitySubCommand::Remove(args) => {
                if let Some(entity_id) = entity_id {
                    display_result(
                        self.entity_app
                            .remove_entity(entity_id)
                            .execute()
                            .map(|_| entity_id),
                    );
                } else {
                    display_each_result(args.ids.into_iter(), |id| {
                        let entity_id = id.try_into()?;
                        self.entity_app
                            .remove_entity(entity_id)
                            .execute()
                            .map(|_| entity_id)
                    })?;
                }
            }
        }

        Ok(())
    }
}

macro_rules! row_format {
    () => {
        "{: <15} {: <40}"
    };
}

/// Displays the [Entity] in a single line.
struct Row;
/// Displays the [Entity] in different lines.
struct Column;

/// Implements diffent strategies of [Display] for [Entity].
struct EntityFmt<'a, S> {
    style: PhantomData<S>,
    entity: &'a Entity,
}

impl<'a> Display for EntityFmt<'a, Row> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            row_format!(),
            self.entity.name.to_string(),
            self.entity.id.to_string(),
        )
    }
}

impl<'a> Display for EntityFmt<'a, Column> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{: <10} {}", "NAME", self.entity.name)?;
        writeln!(f, "{: <10} {}", "UUID", self.entity.id)
    }
}

impl<'a> EntityFmt<'a, Row> {
    /// Returns the string of headers corresponding to the row-like display.
    pub fn headers() -> String {
        format!(row_format!(), "NAME", "UUID")
    }

    /// Returns an instance of [EntityFmt] that displays the given entity in a single line.
    pub fn row(entity: &'a Entity) -> Self {
        EntityFmt {
            style: PhantomData,
            entity,
        }
    }
}

impl<'a> EntityFmt<'a, Column> {
    /// Returns an instance of [EntityFmt] that displays the given entity in different lines.
    pub fn column(entity: &'a Entity) -> Self {
        EntityFmt {
            style: PhantomData,
            entity,
        }
    }
}
