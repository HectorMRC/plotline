use crate::{Error, FlavoredPlugin, Plugin, PluginKind, Result};
use plotline::experience::Experience;
use plotline_proto::plugin::{OnSaveExperienceInput, OnSaveExperienceOutput};
use protobuf::Message;

/// An OnSaveExperiencePlugin is a plugin that is executed before saving
/// an [Experience], determining if the experience is suitable to be saved
/// or not.
pub struct OnSaveExperiencePlugin<'a> {
    plugin: &'a dyn Plugin,
}

impl<'a> TryFrom<&'a dyn Plugin> for OnSaveExperiencePlugin<'a> {
    type Error = Error;

    fn try_from(plugin: &'a dyn Plugin) -> Result<Self> {
        if plugin.kind() != PluginKind::OnSaveExperience {
            return Err(Error::WrongKind);
        }

        Ok(Self { plugin })
    }
}

impl<'a> FlavoredPlugin<'a> for OnSaveExperiencePlugin<'a> {
    fn kind() -> PluginKind {
        PluginKind::OnSaveExperience
    }
}

impl<'a> OnSaveExperiencePlugin<'a> {
    pub fn with_subject<'b, Intv>(
        &self,
        subject: &'b Experience<Intv>,
    ) -> OnSaveExperience<'a, 'b, Intv> {
        OnSaveExperience {
            plugin: self.plugin,
            subject,
            timeline: Default::default(),
            result: Err("not executed".to_string()),
        }
    }
}

/// OnSaveExperience is the [OnSaveExperiencePlugin]'s command.
pub struct OnSaveExperience<'a, 'b, Intv> {
    plugin: &'a dyn Plugin,
    subject: &'b Experience<Intv>,
    timeline: &'a [&'b Experience<Intv>],
    result: std::result::Result<(), String>,
}

impl<'a, 'b, Intv> OnSaveExperience<'a, 'b, Intv> {
    pub fn with_timeline(mut self, timeline: &'a [&'b Experience<Intv>]) -> Self {
        self.timeline = timeline;
        self
    }

    pub fn execute(mut self) -> Self {
        self.result = self.run();
        self
    }

    pub fn result(self) -> std::result::Result<(), String> {
        self.result
    }

    fn run(&self) -> std::result::Result<(), String> {
        let _ = self.subject;

        let input = OnSaveExperienceInput::default();
        let input = input.write_to_bytes().map_err(|err| err.to_string())?;

        let output = self.plugin.run("main", &input)?;
        let output =
            OnSaveExperienceOutput::parse_from_bytes(&output).map_err(|err| err.to_string())?;

        if !output.error.is_empty() {
            return Err(output.error);
        }

        Ok(())
    }
}
