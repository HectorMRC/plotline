mod constraint;
mod error;

use constraint::ExperienceIsNotSimultaneous;
use error::Error;
use plotline::{moment::Moment, period::Period};
use plotline_macros::plugin;
use plotline_plugin::{kind::PluginKind::BeforeSaveExperience, proto};
use plotline_proto::plugin::{
    BeforeSaveExperienceInput, BeforeSaveExperienceOutput, GetPluginId, GetPluginKind,
};
use protobuf::{EnumOrUnknown, Message};

#[plugin(id("experience_is_not_simultaneous"), kind(BeforeSaveExperience))]
fn main(input: BeforeSaveExperienceInput) -> std::result::Result<(), Error> {
    let subject = proto::into_experience(&input.subject).unwrap();

    input
        .timeline
        .into_iter()
        .try_fold(
            ExperienceIsNotSimultaneous::<Period<Moment>>::new(&subject),
            |constraint, exp| {
                let experience = proto::into_experience(&exp).unwrap();
                let constraint = constraint.with(&experience);

                constraint.result().map(|_| constraint)
            },
        )
        .map(|_| ())
}
