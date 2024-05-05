mod constraint;
pub use constraint::*;

use plotline::{experience::Experience, moment::Moment, period::Period, plugin::OutputError};
use plotline_plugin::PluginKind::BeforeSaveExperience;

type Intv = Period<Moment>;

#[plotline_macros::plugin(
    id("experience_is_not_simultaneous"),
    kind(BeforeSaveExperience),
    version("0.1.0")
)]
fn main(
    subject: &Experience<Intv>,
    timeline: &[Experience<Intv>],
) -> std::result::Result<(), OutputError> {
    timeline.iter().try_fold(
        ExperienceIsNotSimultaneous::new(subject),
        |constraint, experience| {
            let constraint = constraint.with(experience);
            constraint.result()?;

            Ok(constraint)
        },
    )?;

    Ok(())
}
