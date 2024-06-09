mod constraint;
pub use constraint::*;

use plotline::{experience::Experience, moment::Moment, period::Period, plugin::OutputError};
use plotline_plugin::PluginKind::BeforeSaveExperience;

type Intv = Period<Moment>;

#[plotline_macros::plugin(
    id("experience_precedes_next"),
    kind(BeforeSaveExperience),
    version("0.1.0")
)]
pub fn main(
    subject: &Experience<Intv>,
    timeline: &[Experience<Intv>],
) -> std::result::Result<(), OutputError> {
    let constraint = timeline.iter().fold(
        ExperiencePrecedesNext::new(subject),
        |constraint, experience| constraint.with(experience),
    );

    constraint.result()
}
