use plotline::plugin;
use plotline_proto::plugin::{BeforeSaveExperienceOutput, PluginError};

#[derive(Debug)]
pub struct Error(pub plugin::Error);

impl From<plugin::Error> for Error {
    fn from(value: plugin::Error) -> Self {
        Self(value)
    }
}

impl From<Error> for PluginError {
    fn from(value: Error) -> Self {
        PluginError {
            code: value.0.code,
            message: value.0.message,
            ..Default::default()
        }
    }
}

impl From<Error> for BeforeSaveExperienceOutput {
    fn from(value: Error) -> Self {
        BeforeSaveExperienceOutput {
            error: protobuf::MessageField::some(value.into()),
            ..Default::default()
        }
    }
}
