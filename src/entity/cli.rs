use super::service::{EntityRepository, EntityService};
use clap::{Args, Subcommand};

#[derive(Args)]
struct EntityCreateArgs {
    /// The name of the entity to be created.
    name: String,
    /// The tags to be added to the entity.
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
    pub fn run(&self, entity_cmd: EntityCommand) {
        match entity_cmd.command {
            EntitySubCommand::Create(args) => {
                let entity = self.create(args.name).with_tags(args.tags).execute();
            }
            EntitySubCommand::Remove(args) => {
                let entities = self.remove(args.names).execute();
            }
        };
    }
}
