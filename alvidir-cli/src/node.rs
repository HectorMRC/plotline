use std::{
    error::Error,
    fmt::Debug,
    io::{self, Write},
    str::FromStr,
    sync::Arc,
};

use alvidir::{id::Identify, schema::Schema};
use anyhow::Result;
use clap::{Args, Subcommand};

#[derive(Args)]
struct NodeSaveArgs {
    /// The content of the node.
    content: Option<String>,
}

#[derive(Subcommand)]
#[clap(subcommand_negates_reqs = true, subcommand_precedence_over_arg = true)]
enum NodeSubCommand {
    /// List all nodes.
    #[command(alias("ls"))]
    List,
    /// Save a node.
    Save(NodeSaveArgs),
}

#[derive(Args)]
pub struct NodeCommand {
    /// The id of the node.
    node: Option<String>,
    /// The action to perform.
    #[command(subcommand)]
    subcommand: NodeSubCommand,
}

pub struct NodeCli<T>
where
    T: Identify,
{
    pub schema: Arc<Schema<T>>,
}

impl<T> NodeCli<T>
where
    T: Identify,
    T::Id: FromStr + Debug,
    <<T as Identify>::Id as FromStr>::Err: 'static + Error + Sync + Send,
{
    pub fn execute(&self, command: NodeCommand) -> Result<()> {
        let _node_id = command.node.map(|id| T::Id::from_str(&id)).transpose()?;

        match command.subcommand {
            NodeSubCommand::List => {
                let mut stdout = io::stdout().lock();
                self.schema
                    .read()
                    .into_iter()
                    .for_each(|node| writeln!(stdout, "{:?}", node.id()).unwrap());
            }
            NodeSubCommand::Save(_) => todo!(),
        }

        Ok(())
    }
}
