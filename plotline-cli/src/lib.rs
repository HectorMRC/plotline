use clap::Subcommand;
use document::DocumentCommand;

pub mod document;
pub mod repository;

#[derive(Subcommand)]
pub enum CliCommand {
    Doc(DocumentCommand),
}
