use crate::{Error, FlavoredPlugin, Plugin, PluginKind, Result};
use plotline::experience::Experience;
use plotline_proto::plugin::{BeforeSaveExperienceInput, BeforeSaveExperienceOutput};
use protobuf::Message;

/// An BeforeSaveExperiencePlugin is a plugin that is executed before saving an
/// [Experience], determining if the experience is suitable to be saved or not.
pub struct BeforeSaveExperiencePlugin<'a> {
    plugin: &'a dyn Plugin,
}

impl<'a> TryFrom<&'a dyn Plugin> for BeforeSaveExperiencePlugin<'a> {
    type Error = Error;

    fn try_from(plugin: &'a dyn Plugin) -> Result<Self> {
        if plugin.kind() != PluginKind::BeforeSaveExperience {
            return Err(Error::WrongKind);
        }

        Ok(Self { plugin })
    }
}

impl<'a> FlavoredPlugin<'a> for BeforeSaveExperiencePlugin<'a> {
    fn kind() -> PluginKind {
        PluginKind::BeforeSaveExperience
    }
}

impl<'a> BeforeSaveExperiencePlugin<'a> {
    pub fn with_subject<'b, Intv>(
        &self,
        subject: &'b Experience<Intv>,
    ) -> BeforeSaveExperience<'a, 'b, Intv> {
        BeforeSaveExperience {
            plugin: self.plugin,
            subject,
            timeline: Default::default(),
            result: Err("not executed".to_string()),
        }
    }
}

/// BeforeSaveExperience is the [BeforeSaveExperiencePlugin]'s command.
pub struct BeforeSaveExperience<'a, 'b, Intv> {
    plugin: &'a dyn Plugin,
    subject: &'b Experience<Intv>,
    timeline: &'a [&'b Experience<Intv>],
    result: std::result::Result<(), String>,
}

impl<'a, 'b, Intv> BeforeSaveExperience<'a, 'b, Intv> {
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

        let input = BeforeSaveExperienceInput::default();
        let input = input.write_to_bytes().map_err(|err| err.to_string())?;

        let output = self.plugin.run("main", &input)?;
        let output =
            BeforeSaveExperienceOutput::parse_from_bytes(&output).map_err(|err| err.to_string())?;

        if !output.error.is_empty() {
            return Err(output.error);
        }

        Ok(())
    }
}
