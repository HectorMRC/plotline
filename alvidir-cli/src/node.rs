use clap::{Args, Subcommand};

#[derive(Subcommand)]
#[clap(subcommand_negates_reqs = true, subcommand_precedence_over_arg = true)]
enum NodeSubCommand {
    /// List all entities.
    #[command(alias("ls"))]
    List,
}

#[derive(Args)]
pub struct NodeCommand {
    /// The id of the node.
    node: Option<String>,
    /// The action to perform.
    #[command(subcommand)]
    command: Option<NodeSubCommand>,
}
