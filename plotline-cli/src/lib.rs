pub mod entity;
pub mod event;
pub mod experience;
pub mod display;

mod error;
pub use error::*;

use clap::Subcommand;

#[derive(Subcommand, strum_macros::Display)]
pub enum CliCommand {
    /// Manage entities.
    Entity(entity::EntityCommand),
    /// Manage events.
    Event(event::EventCommand),
    /// Manage experiences.
    Experience(experience::ExperienceCommand),
}
