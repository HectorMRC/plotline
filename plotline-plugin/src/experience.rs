use crate::{
    entity::proto_entity, event::proto_event, Error, FlavoredPlugin, Plugin, PluginKind,
    PluginStore,
};
use plotline::{
    experience::{
        application::{BeforeSaveExperience, PluginFactory},
        Experience, Profile,
    },
    id::Identifiable,
    interval::Interval,
};
use plotline_proto::{
    model as proto,
    plugin::{BeforeSaveExperienceInput, BeforeSaveExperienceOutput},
};
use protobuf::{Message, MessageField};
use std::ops::Deref;

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

/// A BeforeSaveExperiencePlugin is a plugin that is executed before saving an
/// [Experience], determining if the experience is suitable to be saved or not.
pub struct BeforeSaveExperiencePlugin<'a, Intv> {
    plugin: &'a dyn Plugin,
    subject: Option<&'a Experience<Intv>>,
    timeline: &'a [&'a Experience<Intv>],
    result: std::result::Result<(), String>,
}

impl<'a, Intv> TryFrom<&'a dyn Plugin> for BeforeSaveExperiencePlugin<'a, Intv> {
    type Error = Error;

    fn try_from(plugin: &'a dyn Plugin) -> Result<Self, Self::Error> {
        if plugin.kind() != PluginKind::BeforeSaveExperience {
            return Err(Error::WrongKind);
        }

        Ok(Self {
            plugin,
            subject: Default::default(),
            timeline: Default::default(),
            result: Err("not executed".to_string()),
        })
    }
}

impl<'a, Intv> FlavoredPlugin<'a> for BeforeSaveExperiencePlugin<'a, Intv> {
    fn kind() -> PluginKind {
        PluginKind::BeforeSaveExperience
    }
}

impl<'a, Intv> BeforeSaveExperience<'a, Intv> for BeforeSaveExperiencePlugin<'a, Intv> {
    fn with_subject(mut self, subject: &'a Experience<Intv>) -> Self {
        self.subject = Some(subject);
        self
    }

    fn with_timeline(mut self, timeline: &'a [&Experience<Intv>]) -> Self {
        self.timeline = timeline;
        self
    }

    async fn execute(mut self) -> Self {
        self.result = self.run();
        self
    }

    fn result(self) -> std::result::Result<(), String> {
        self.result
    }
}

impl<'a, Intv> BeforeSaveExperiencePlugin<'a, Intv> {
    fn run(&self) -> std::result::Result<(), String> {
        let Some(subject) = self.subject else {
            return Err("subject has to be set".to_string());
        };

        let input = BeforeSaveExperienceInput {
            subject: MessageField::some(proto_experience(subject)),
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

impl<Intv> PluginFactory for PluginStore<Intv>
where
    Intv: Interval,
{
    type Intv = Intv;
    type BeforeSaveExperience<'b> = BeforeSaveExperiencePlugin<'b, Intv>
    where
        Self: 'b;

    fn before_save_experience(&self) -> Vec<Self::BeforeSaveExperience<'_>> {
        self.retrieve().unwrap_or_default()
    }
}
