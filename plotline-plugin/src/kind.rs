use plotline_proto::plugin;

/// PluginKind determines the kind of a plugin.
#[derive(PartialEq, Eq, Clone, strum_macros::EnumString, strum_macros::VariantNames)]
pub enum PluginKind {
    /// Plugins of this kind will be executed before saving an experience. Its
    /// result will indicate whether the experience is suitable to be saved or
    /// not.
    BeforeSaveExperience,
}

impl From<plugin::PluginKind> for PluginKind {
    fn from(value: plugin::PluginKind) -> Self {
        match value {
            plugin::PluginKind::BeforeSaveExperience => PluginKind::BeforeSaveExperience,
        }
    }
}

impl From<PluginKind> for plugin::PluginKind {
    fn from(value: PluginKind) -> Self {
        match value {
            PluginKind::BeforeSaveExperience => plugin::PluginKind::BeforeSaveExperience,
        }
    }
}
