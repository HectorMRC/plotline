mod constraint;
pub use constraint::*;

mod error;
pub use error::*;

use plotline::{experience::Experience, moment::Moment, period::Period};
use plotline_plugin::PluginKind::BeforeSaveExperience;

type Intv = Period<Moment>;

#[plotline_macros::plugin(
    id("ExperienceIsNotSimultaneous"),
    kind(BeforeSaveExperience),
    version("0.1.0")
)]
fn main(
    subject: &Experience<Intv>,
    timeline: &[Experience<Intv>],
) -> std::result::Result<(), Error> {
    timeline.iter().try_fold(
        ExperienceIsNotSimultaneous::new(subject),
        |constraint, experience| -> Result<_, Error> {
            let constraint = constraint.with(experience);
            constraint.result()?;

            Ok(constraint)
        },
    )?;

    Ok(())
}
