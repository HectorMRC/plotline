use crate::{proto, store::PluginStore, Error, Plugin, PluginFlavor, PluginKind};
use plotline::{
    experience::{
        application::{BeforeSaveExperience, PluginFactory},
        Experience,
    },
    id::Indentify,
    interval::Interval,
};
use plotline_proto::plugin::{BeforeSaveExperienceInput, BeforeSaveExperienceOutput};
use protobuf::{Message, MessageField};
use std::{fmt::Display, ops::Deref};

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

impl<'a, Intv> Indentify for BeforeSaveExperiencePlugin<'a, Intv> {
    type Id = String;

    fn id(&self) -> Self::Id {
        self.plugin.id().into()
    }
}

impl<'a, Intv> PluginFlavor<'a> for BeforeSaveExperiencePlugin<'a, Intv> {
    fn kind() -> PluginKind {
        PluginKind::BeforeSaveExperience
    }
}

impl<'a, Intv> BeforeSaveExperience<'a, Intv> for BeforeSaveExperiencePlugin<'a, Intv>
where
    Intv: Interval,
    Intv::Bound: Display,
{
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

    fn result(&self) -> std::result::Result<(), String> {
        self.result.clone()
    }
}

impl<'a, Intv> BeforeSaveExperiencePlugin<'a, Intv>
where
    Intv: Interval,
    Intv::Bound: Display,
{
    fn run(&self) -> std::result::Result<(), String> {
        let Some(subject) = self.subject else {
            return Err("subject has to be set".to_string());
        };

        let input = BeforeSaveExperienceInput {
            subject: MessageField::some(proto::from_experience(subject)),
            timeline: self
                .timeline
                .iter()
                .map(Deref::deref)
                .map(proto::from_experience)
                .collect(),
            ..Default::default()
        }
        .write_to_bytes()
        .map_err(|err| err.to_string())?;

        let output = BeforeSaveExperienceOutput::parse_from_bytes(&self.plugin.run(&input)?)
            .map_err(|err| err.to_string())?;

        if !output.error.is_empty() {
            return Err(format!("{}\n{}", output.error, output.details));
        }

        Ok(())
    }
}

impl<Intv> PluginFactory for PluginStore<Intv>
where
    Intv: Interval,
    Intv::Bound: Display,
{
    type Intv = Intv;
    type BeforeSaveExperience<'b> = BeforeSaveExperiencePlugin<'b, Intv>
    where
        Self: 'b;

    fn before_save_experience(&self) -> Vec<Self::BeforeSaveExperience<'_>> {
        self.retrieve().unwrap_or_default()
    }
}
