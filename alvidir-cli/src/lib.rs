use clap::Subcommand;

pub mod document;

#[derive(Subcommand, strum_macros::Display)]
pub enum CliCommand {
    /// Manage documents.
    Document(document::DocumentCommand),
}