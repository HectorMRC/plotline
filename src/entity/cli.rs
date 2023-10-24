use super::{
    fmt::EntityFmt,
    service::{EntityFilter, EntityRepository, EntityService},
};
use crate::cli::CliResult;
use clap::{Args, Subcommand};
use std::{
    io::{stdout, Write},
    sync::mpsc,
};

#[derive(Args)]
struct EntityCreateArgs {
    /// The name of the entity.
    #[arg(num_args(1..))]
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
    /// Given an [EntityCommand] parsed by Clap, executes the corresponding command.
    pub fn execute(&self, entity_cmd: EntityCommand) -> CliResult {
        match entity_cmd.command {
            EntitySubCommand::Create(args) => {
                let entity = self
                    .create(args.name.try_into()?)
                    .with_id(args.id.map(TryInto::try_into).transpose()?)
                    .execute()?;

                println!("{}", entity.id);
            }

            EntitySubCommand::List => {
                let entities = self.filter().execute()?;

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
                let receiver = std::thread::scope(|scope| {
                    let (sender, receiver) = mpsc::channel();
                    args.ids.into_iter().for_each(|id| {
                        let sender = sender.clone();
                        scope.spawn(move || {
                            sender.send(id.try_into().and_then(|id| self.remove(id).execute()))
                        });
                    });

                    receiver
                });

                while let Ok(result) = receiver.recv() {
                    match result {
                        Ok(entity) => println!("{}", entity.id),
                        Err(error) => eprintln!("{error}"),
                    }
                }
            }
        }

        Ok(())
    }
}
