use crate::{
    entity::proto_entity, event::proto_event, Error, FlavoredPlugin, Plugin, PluginKind, Result,
};
use plotline::{
    experience::{Experience, Profile},
    id::Identifiable,
};
use plotline_proto::{
    model as proto,
    plugin::{BeforeSaveExperienceInput, BeforeSaveExperienceOutput},
};
use protobuf::{Message, MessageField};
use std::ops::Deref;

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
        let input = BeforeSaveExperienceInput {
            subject: MessageField::some(proto_experience(&self.subject)),
            timeline: self
                .timeline
                .iter()
                .map(Deref::deref)
                .map(proto_experience)
                .collect(),
            ..Default::default()
        }
        .write_to_bytes()
        .map_err(|err| err.to_string())?;

        let output =
            BeforeSaveExperienceOutput::parse_from_bytes(&self.plugin.run("main", &input)?)
                .map_err(|err| err.to_string())?;

        if !output.error.is_empty() {
            return Err(output.error);
        }

        Ok(())
    }
}

fn proto_profile(profile: &Profile) -> proto::Profile {
    proto::Profile {
        entity: MessageField::some(proto_entity(&profile.entity)),
        values: profile
            .values
            .iter()
            .map(|(key, value)| proto::KeyValue {
                key: key.to_string(),
                value: value.to_string(),
                ..Default::default()
            })
            .collect(),
        ..Default::default()
    }
}

fn proto_experience<Intv>(experience: &Experience<Intv>) -> proto::Experience {
    proto::Experience {
        id: experience.id().to_string(),
        entity: MessageField::some(proto_entity(&experience.entity)),
        event: MessageField::some(proto_event(&experience.event)),
        profiles: experience.profiles.iter().map(proto_profile).collect(),
        ..Default::default()
    }
}
