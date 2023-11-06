use super::{
    fmt::EntityFmt,
    service::{EntityFilter, EntityRepository, EntityService},
};
use crate::cli::{display_all, CliResult};
use clap::{Args, Subcommand};
use std::io::{stdout, Write};

#[derive(Args)]
struct EntityCreateArgs {
    /// The name of the entity.
    name: String,
    /// The uuid string of the entity.
    #[arg(short, long)]
    id: Option<String>,
}

#[derive(Args)]
struct EntityRemoveArgs {
    /// The uuid of all the entities to be removed.
    ids: Vec<String>,
}

#[derive(Args)]
struct EntityFindArgs {
    /// The name of the entity.
    #[arg(conflicts_with = "id")]
    name: Option<String>,
    /// The uuid string of the entity.
    #[arg(short, long, conflicts_with = "name")]
    id: Option<String>,
}

#[derive(Subcommand)]
enum EntitySubCommand {
    /// Create a new entity
    Create(EntityCreateArgs),
    /// List entities in plotfile
    #[command(alias("ls"))]
    List,
    /// Remove one or more entities
    #[command(alias("rm"))]
    Remove(EntityRemoveArgs),
    /// Displays the information of the entity.
    Find(EntityFindArgs),
}

#[derive(Args)]
#[command(arg_required_else_help = true)]
pub struct EntityCommand {
    #[command(subcommand)]
    command: EntitySubCommand,
}

impl<R> EntityService<R>
where
    R: 'static + EntityRepository + Sync + Send,
{
    /// Given an [EntityCommand], executes the corresponding logic.
    pub fn execute(&self, entity_cmd: EntityCommand) -> CliResult {
        match entity_cmd.command {
            EntitySubCommand::Create(args) => {
                let entity = self
                    .create_entity(args.name.try_into()?)
                    .with_id(args.id.map(TryInto::try_into).transpose()?)
                    .execute()?;

                println!("{}", entity.id);
            }

            EntitySubCommand::List => {
                let entities = self.filter_entities().execute()?;

                let mut stdout = stdout().lock();
                writeln!(stdout, "{}", EntityFmt::headers())?;

                entities
                    .into_iter()
                    .try_for_each(|entity| write!(stdout, "{}", EntityFmt::row(&entity)))?
            }

            EntitySubCommand::Find(args) => {
                let filter = EntityFilter::default()
                    .with_name(args.name.map(TryInto::try_into).transpose()?)
                    .with_id(args.id.map(TryInto::try_into).transpose()?);

                let entity = self.find_entity().with_filter(filter).execute()?;
                print!("{}", EntityFmt::column(&entity));
            }

            EntitySubCommand::Remove(args) => display_all(args.ids.into_iter(), |id| {
                self.remove_entity(id.try_into()?)
                    .execute()
                    .map(|entity| entity.id)
            })?,
        }

        Ok(())
    }
}
