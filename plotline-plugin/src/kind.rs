use plotline_proto::plugin;

/// PluginKind determines the kind of a plugin.
#[derive(PartialEq, Eq, Clone, strum_macros::EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum PluginKind {
    /// Plugins of this kind will be executed before saving an experience. Its
    /// result will indicate whether the experience is suitable to be saved or
    /// not.
    BeforeSaveExperience,
}

impl From<plugin::PluginKind> for PluginKind {
    fn from(value: plugin::PluginKind) -> Self {
        match value {
            plugin::PluginKind::BEFORE_SAVE_EXPERIENCE => PluginKind::BeforeSaveExperience,
        }
    }
}
