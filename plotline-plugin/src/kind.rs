use plotline_proto::plugin as proto;
use protobuf::EnumOrUnknown;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("invalid plugin kind")]
    NotAKind,
}

/// PluginKind determines the kind of a plugin.
#[derive(PartialEq, Eq, Clone)]
pub enum PluginKind {
    /// Plugins of this kind will be executed before saving an experience. Its
    /// result will indicate whether the experience is suitable to be saved or
    /// not.
    BeforeSaveExperience,
}

impl From<proto::PluginKind> for PluginKind {
    fn from(value: proto::PluginKind) -> Self {
        match value {
            proto::PluginKind::BeforeSaveExperience => PluginKind::BeforeSaveExperience,
        }
    }
}

impl TryFrom<EnumOrUnknown<proto::PluginKind>> for PluginKind {
    type Error = Error;

    fn try_from(value: EnumOrUnknown<proto::PluginKind>) -> std::result::Result<Self, Self::Error> {
        value
            .enum_value()
            .map(PluginKind::from)
            .map_err(|_| Error::NotAKind)
    }
}

impl From<PluginKind> for proto::PluginKind {
    fn from(value: PluginKind) -> Self {
        match value {
            PluginKind::BeforeSaveExperience => proto::PluginKind::BeforeSaveExperience,
        }
    }
}
