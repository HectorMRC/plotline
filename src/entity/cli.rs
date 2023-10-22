use super::{
    fmt::EntityFmt,
    service::{EntityRepository, EntityService},
};
use crate::cli::{CliError, CliResult};
use clap::{Args, Subcommand};
use std::io::{stderr, stdout, Write};

#[derive(Args)]
struct EntityCreateArgs {
    /// The name of the entity.
    name: String,
    /// The uuid string of the entity.
    #[arg(short, long)]
    id: Option<String>,
    /// A list of tags to be added to the entity.
    #[arg(short, long)]
    tags: Vec<String>,
}

#[derive(Args)]
struct EntityRemoveArgs {
    /// The name of all the entities to be removed.
    #[arg(short, long)]
    names: Vec<String>,
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
    /// Given an [EntityCommand] parsed by Clap, executes the corresponding command.
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

            EntitySubCommand::List => {
                let entities = self.list().execute()?;

                let mut stdout = stdout().lock();
                writeln!(stdout, "{}", EntityFmt::headers()).unwrap();

                entities.into_iter().for_each(|entity| {
                    write!(stdout, "{}", EntityFmt::row(&entity)).unwrap();
                })
            }

            EntitySubCommand::Remove(args) => {
                let mut stdout = stdout().lock();
                let mut stderr = stderr().lock();

                args.names
                    .into_iter()
                    .map(|name| self.remove_by_name().with_name(name))
                    .map(|command| std::thread::spawn(move || command.execute()))
                    .collect::<Vec<_>>()
                    .into_iter()
                    .for_each(|handle| {
                        let command_result = match handle.join() {
                            Ok(result) => result,
                            Err(error) => {
                                writeln!(stderr, "{:?}", error).unwrap();
                                return;
                            }
                        };

                        match command_result {
                            Ok(entity) => writeln!(stdout, "{}", entity.id).unwrap(),
                            Err(error) => writeln!(stderr, "{error}").unwrap(),
                        }
                    });
            }
        }

        Ok(())
    }
}
