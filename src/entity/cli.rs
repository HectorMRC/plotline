use super::service::{EntityRepository, EntityService};
use crate::cli::{CliError, CliResult};
use clap::{
    error::{ContextKind, ContextValue, ErrorKind},
    Args, Subcommand,
};
use std::io::{stdout, Write};

#[derive(Args)]
struct EntityCreateArgs {
    /// The uuid string of the entity.
    #[arg(short, long)]
    id: String,
    /// The name of the entity.
    name: String,
    /// A list of tags to be added to the entity.
    #[arg(short, long)]
    tags: Vec<String>,
}

#[derive(Args)]
struct EntityRemoveArgs {
    /// The name of all the entities to be removed.
    names: Vec<String>,
}

#[derive(Subcommand)]
enum EntitySubCommand {
    /// Create a new entity
    Create(EntityCreateArgs),
    /// Remove one or more entities
    Remove(EntityRemoveArgs),
}

#[derive(Args)]
#[command(arg_required_else_help = true)]
pub struct EntityCommand {
    #[command(subcommand)]
    command: EntitySubCommand,
}

impl<R> EntityService<R>
where
    R: EntityRepository,
{
    /// Given an [EntityCommand] parsed by Clap, executes the corresponding logic.
    pub fn execute(&self, entity_cmd: EntityCommand) -> CliResult {
        match entity_cmd.command {
            EntitySubCommand::Create(args) => {
                let entity = self
                    .create()
                    .with_id(args.id)
                    .with_name(args.name)
                    .with_tags(args.tags)
                    .execute()
                    .map_err(CliError::from)?;

                println!("{}", entity.id);
            }
            EntitySubCommand::Remove(args) => {
                let entities = self
                    .remove()
                    .with_names(args.names)
                    .execute()
                    .map_err(CliError::from)?;

                let mut lock = stdout().lock();
                let errors_str: Vec<String> = entities
                    .into_iter()
                    .filter_map(|entity| writeln!(lock, "{}", entity.id).err())
                    .map(|err| err.to_string())
                    .collect();

                if !errors_str.is_empty() {
                    let mut stderr = clap::Error::new(ErrorKind::Io);
                    stderr.insert(ContextKind::Custom, ContextValue::Strings(errors_str));
                    stderr.print().map_err(CliError::from)?;
                }
            }
        };

        Ok(())
    }
}
