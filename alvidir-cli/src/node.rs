use std::{fmt::Display, io::Write, str::FromStr, sync::Arc};

use alvidir::{graph::{application::GraphApplication, Node}, id::Identify};
use clap::{Args, Subcommand};

#[derive(Subcommand)]
#[clap(subcommand_negates_reqs = true, subcommand_precedence_over_arg = true)]
enum NodeSubCommand {
    /// List all documents.
    #[command(alias("ls"))]
    List,
}

#[derive(Args)]
#[command(arg_required_else_help = true)]
pub struct NodeCommand {
    /// The id of the node.
    node: Option<String>,
    /// The action to perform.
    #[command(subcommand)]
    command: Option<NodeSubCommand>,
}

pub struct NodeCli<T: Identify> {
    pub graph_app: Arc<GraphApplication<T>>,
}


impl<T> NodeCli<T>
where 
    T: Identify + Node,
    T::Id: Display + FromStr + Clone,
    <T::Id as FromStr>::Err: 'static + std::error::Error + Sync + Send,
{
    pub async fn execute(&self, cli: NodeCommand) -> anyhow::Result<()> {
        let node_id = cli.node.map(|s| T::Id::from_str(&s)).transpose()?;
        if let Some(command) = cli.command {
            return self.execute_subcommand(command, node_id).await;
        }

        Ok(())
    }

    async fn execute_subcommand(
        &self,
        subcommand: NodeSubCommand,
        _node_id: Option<T::Id>,
    ) -> anyhow::Result<()> {
        match subcommand {
            NodeSubCommand::List => {
                let mut stdout = std::io::stdout().lock();
                for node in self.graph_app.graph.nodes() {
                    writeln!(stdout, "{}", node.id())?;
                }
            }
        };

        Ok(())
    }
}
