use clap::Subcommand;

pub mod document;
pub mod node;
pub mod repository;
pub mod trigger;

#[derive(Subcommand, strum_macros::Display)]
pub enum CliCommand {
    /// Manage documents.
    Document(document::DocumentCommand),
    /// Manage nodes.
    Node(node::NodeCommand),
}