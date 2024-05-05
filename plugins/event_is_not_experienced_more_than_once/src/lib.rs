mod constraint;
pub use constraint::*;

use plotline::{experience::Experience, moment::Moment, period::Period, plugin::PluginError};
use plotline_plugin::PluginKind::BeforeSaveExperience;

type Intv = Period<Moment>;

#[plotline_macros::plugin(
    id("event_is_not_experienced_more_than_once"),
    kind(BeforeSaveExperience),
    version("0.1.0")
)]
fn main(
    subject: &Experience<Intv>,
    timeline: &[Experience<Intv>],
) -> std::result::Result<(), PluginError> {
    timeline.iter().try_fold(
        EventIsNotExperiencedMoreThanOnce::new(subject),
        |constraint, experience| {
            let constraint = constraint.with(experience);
            constraint.result()?;

            Ok(constraint)
        },
    )?;

    Ok(())
}
