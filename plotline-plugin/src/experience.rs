use crate::{Error, FlavoredPlugin, Plugin, PluginKind, Result};

/// An OnSaveExperience is a plugin that is executed before saving an
/// experience.
pub struct OnSaveExperience<'a> {
    plugin: &'a Box<dyn Plugin>,
}

impl<'a> TryFrom<&'a Box<dyn Plugin>> for OnSaveExperience<'a> {
    type Error = Error;

    fn try_from(plugin: &'a Box<dyn Plugin>) -> Result<Self> {
        if plugin.kind() != PluginKind::OnSaveExperience {
            return Err(Error::WrongKind);
        }

        Ok(Self { plugin })
    }
}

impl<'a> FlavoredPlugin<'a> for OnSaveExperience<'a> {
    fn kind() -> PluginKind {
        PluginKind::OnSaveExperience
    }
}

impl<'a> OnSaveExperience<'a> {
    pub fn execute(&self) {
        todo!()
    }
}
