#[macro_use]
extern crate prettytable;

pub mod entity;
pub mod event;
pub mod experience;

mod error;
pub use error::*;

use clap::Subcommand;
use plotline::{
    experience::{
        application::ConstraintFactory,
        constraint::{Constraint, LiFoConstraintChain},
        ExperiencedEvent,
    },
    interval::Interval,
};

#[derive(Subcommand, strum_macros::Display)]
pub enum CliCommand {
    /// Manage entities.
    Entity(entity::EntityCommand),
    /// Manage events.
    Event(event::EventCommand),
    /// Manage experiences.
    Experience(experience::ExperienceCommand),
}

impl<Intv> ConstraintFactory<Intv> for CliCommand
where
    Intv: Interval,
{
    fn new<'a>(experienced_event: &'a ExperiencedEvent<'a, Intv>) -> impl Constraint<'a, Intv> {
        LiFoConstraintChain::with_defaults(experienced_event)
    }
}
