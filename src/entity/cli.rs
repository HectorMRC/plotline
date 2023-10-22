use super::{
    fmt::EntityFmt,
    service::{EntityFilter, EntityRepository, EntityService},
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
    /// The uuid of all the entities to be removed.
    #[arg(short, long)]
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
    /// Given an [EntityCommand] parsed by Clap, executes the corresponding command.
    pub fn execute(&self, entity_cmd: EntityCommand) -> CliResult {
        match entity_cmd.command {
            EntitySubCommand::Create(args) => {
                let entity = self
                    .create(args.name.try_into()?)
                    .with_id(args.id.map(TryInto::try_into).transpose()?)
                    .with_tags(args.tags.try_into()?)
                    .execute()
                    .map_err(CliError::from)?;

                println!("{}", entity.id);
            }

            EntitySubCommand::List => {
                let filter = EntityFilter::default();
                let entities = self.filter().with_filter(filter).execute()?;

                let mut stdout = stdout().lock();
                writeln!(stdout, "{}", EntityFmt::headers()).unwrap();

                entities.into_iter().for_each(|entity| {
                    write!(stdout, "{}", EntityFmt::row(&entity)).unwrap();
                })
            }

            EntitySubCommand::Find(args) => {
                let filter = EntityFilter::default()
                    .with_name(args.name.map(TryInto::try_into).transpose()?)
                    .with_id(args.id.map(TryInto::try_into).transpose()?);

                let entity = self.find().with_filter(filter).execute()?;
                print!("{}", EntityFmt::column(&entity));
            }

            EntitySubCommand::Remove(args) => {
                let mut handles: Vec<_> = Vec::with_capacity(args.ids.len() + args.names.len());
                args.names
                    .into_iter()
                    .map(|name| (name, self.remove()))
                    .for_each(|(name, command)| {
                        handles.push(std::thread::spawn(|| {
                            let filter = EntityFilter::default().with_name(Some(name.try_into()?));
                            command.with_filter(filter).execute()
                        }));
                    });

                args.ids
                    .into_iter()
                    .map(|id| (id, self.remove()))
                    .for_each(|(id, command)| {
                        handles.push(std::thread::spawn(|| {
                            let filter = EntityFilter::default().with_id(Some(id.try_into()?));
                            command.with_filter(filter).execute()
                        }));
                    });

                let mut stdout = stdout().lock();
                let mut stderr = stderr().lock();

                handles.into_iter().for_each(|handle| {
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
