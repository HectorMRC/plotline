use clap::Subcommand;
use node::NodeCommand;

pub mod node;
pub mod repository;

#[derive(Subcommand)]
pub enum CliCommand {
    Node(NodeCommand),
}
