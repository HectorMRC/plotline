use crate::{
    entity::proto_entity, event::proto_event, Error, FlavoredPlugin, Plugin, PluginKind,
    PluginStore, Result,
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

impl<'a, Intv> FlavoredPlugin<'a> for BeforeSaveExperiencePlugin<'a, Intv> {
    fn kind() -> PluginKind {
        PluginKind::BeforeSaveExperience
    }
}

impl<'a, Intv> TryFrom<&'a dyn Plugin> for BeforeSaveExperiencePlugin<'a, Intv> {
    type Error = Error;

    fn try_from(plugin: &'a dyn Plugin) -> Result<Self> {
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

impl<'a, Intv> BeforeSaveExperiencePlugin<'a, Intv> {
    pub fn with_subject(mut self, subject: &'a Experience<Intv>) -> Self {
        self.subject = Some(subject);
        self
    }

    pub fn with_timeline(mut self, timeline: &'a [&Experience<Intv>]) -> Self {
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
        let Some(subject) = self.subject else {
            return Err("subject has to be set".to_string());
        };

        let input = BeforeSaveExperienceInput {
            subject: MessageField::some(proto_experience(&subject)),
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

pub struct BeforeSaveExperienceGroup<'a, Intv> {
    plugins: Vec<BeforeSaveExperiencePlugin<'a, Intv>>,
}

impl<'a, Intv> Default for BeforeSaveExperienceGroup<'a, Intv> {
    fn default() -> Self {
        Self {
            plugins: Default::default(),
        }
    }
}

impl<'a, Intv> BeforeSaveExperience<Intv> for BeforeSaveExperienceGroup<'a, Intv> {
    fn with_subject(self, subject: &Experience<Intv>) -> Self {
        todo!()
    }

    fn with_timeline(self, timeline: &[&Experience<Intv>]) -> Self {
        todo!()
    }

    fn execute(self) -> Self {
        todo!()
    }

    fn result(self) -> plotline::experience::Result<()> {
        todo!()
    }
}

impl<'a, Intv> BeforeSaveExperienceGroup<'a, Intv> {
    pub fn new(plugins: Vec<BeforeSaveExperiencePlugin<'a, Intv>>) -> Self {
        BeforeSaveExperienceGroup { plugins }
    }
}

impl<Intv> PluginFactory for PluginStore<Intv>
where
    Intv: Interval,
{
    type Intv = Intv;
    type BeforeSaveExperience<'b> = BeforeSaveExperienceGroup<'b, Intv>
    where
        Self: 'b;

    fn before_save_experience(&self) -> Self::BeforeSaveExperience<'_> {
        self.retrieve()
            .map(BeforeSaveExperienceGroup::new)
            .unwrap_or_default()
    }
}
