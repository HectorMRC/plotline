mod constraint;
pub use constraint::*;

use plotline::{experience::Experience, moment::Moment, period::Period, plugin::OutputError};
use plotline_plugin::PluginKind::BeforeSaveExperience;

type Intv = Period<Moment>;

#[plotline_macros::plugin(
    id("experience_follows_previous"),
    kind(BeforeSaveExperience),
    version("0.1.0")
)]
fn main(
    subject: &Experience<Intv>,
    timeline: &[Experience<Intv>],
) -> std::result::Result<(), OutputError> {
    let constraint = timeline.iter().fold(
        ExperienceFollowsPrevious::new(subject),
        |constraint, experience| constraint.with(experience),
    );

    constraint.result()
}
