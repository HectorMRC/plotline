mod constraint;
mod error;

use constraint::ExperienceIsNotSimultaneous;
use error::Error;
use plotline::{moment::Moment, period::Period};
use plotline_plugin::proto;
use plotline_proto::plugin::{
    BeforeSaveExperienceInput, BeforeSaveExperienceOutput, GetPluginId, GetPluginKind, PluginKind,
};
use protobuf::Message;

#[no_mangle]
fn id() -> *const u8 {
    let output = GetPluginId {
        id: "experience_is_not_simultaneous".to_string(),
        ..Default::default()
    };

    let output_bytes = output.write_to_bytes().unwrap();
    let output_len = (output_bytes.len() as u32).to_le_bytes();
    let output_bytes = [&output_len[..], &output_bytes].concat();
    output_bytes.as_ptr()
}

#[no_mangle]
fn kind() -> *const u8 {
    let output = GetPluginKind {
        kind: PluginKind::BEFORE_SAVE_EXPERIENCE.into(),
        ..Default::default()
    };

    let output_bytes = output.write_to_bytes().unwrap();
    let output_len = (output_bytes.len() as u32).to_le_bytes();
    let output_bytes = [&output_len[..], &output_bytes].concat();
    output_bytes.as_ptr()
}

#[no_mangle]
fn run(ptr: u32) -> *const u8 {
    let input = unsafe {
        let len = *(ptr as *const u32);
        let bytes = (ptr + 4) as *const u8;
        let slice = core::slice::from_raw_parts(bytes, len as usize);
        BeforeSaveExperienceInput::parse_from_bytes(slice).unwrap()
    };

    let output: BeforeSaveExperienceOutput = match execute(input) {
        Ok(_) => Default::default(),
        Err(err) => err.into(),
    };

    let output_bytes = output.write_to_bytes().unwrap();
    let output_len = (output_bytes.len() as u32).to_le_bytes();
    let output_bytes = [&output_len[..], &output_bytes].concat();
    output_bytes.as_ptr()
}

fn execute(input: BeforeSaveExperienceInput) -> std::result::Result<(), Error> {
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
