use crate::{proto, store::PluginStore, Error, PluginFlavor, PluginKind, RawPlugin};
use plotline::{
    experience::{
        application::{BeforeSaveExperience, PluginFactory},
        Experience,
    },
    id::Indentify,
    interval::Interval,
    plugin::{Error as PluginError, OutputError, Plugin, PluginGroup, PluginId, Result},
};
use plotline_proto::plugin::{BeforeSaveExperienceInput, BeforeSaveExperienceOutput};
use protobuf::{Message, MessageField};
use std::{fmt::Display, ops::Deref};

/// A BeforeSaveExperiencePlugin is a [Plugin] that is executed before saving
/// an [Experience], determining if the experience is suitable to be saved or
/// not.
pub struct BeforeSaveExperiencePlugin<'a, Intv> {
    plugin: &'a dyn RawPlugin,
    subject: Option<&'a Experience<Intv>>,
    timeline: &'a [&'a Experience<Intv>],
    result: Result<()>,
}

impl<'a, Intv> TryFrom<&'a dyn RawPlugin> for BeforeSaveExperiencePlugin<'a, Intv> {
    type Error = Error;

    fn try_from(plugin: &'a dyn RawPlugin) -> std::result::Result<Self, Self::Error> {
        if plugin.kind() != PluginKind::BeforeSaveExperience {
            return Err(Error::WrongKind);
        }

        Ok(Self {
            plugin,
            subject: Default::default(),
            timeline: Default::default(),
            result: Err(PluginError::Execution {
                plugin: plugin.id(),
                error: "not executed".into(),
            }),
        })
    }
}

impl<'a, Intv> Indentify for BeforeSaveExperiencePlugin<'a, Intv> {
    type Id = PluginId;

    fn id(&self) -> Self::Id {
        self.plugin.id()
    }
}

impl<'a, Intv> Plugin<()> for BeforeSaveExperiencePlugin<'a, Intv>
where
    Intv: Interval,
    Intv::Bound: Display,
{
    async fn execute(mut self) -> Self {
        self.result = self.run();
        self
    }

    fn result(&self) -> Result<()> {
        self.result.clone()
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
}

impl<'a, Intv> BeforeSaveExperiencePlugin<'a, Intv>
where
    Intv: Interval,
    Intv::Bound: Display,
{
    fn run(&self) -> Result<()> {
        let Some(subject) = self.subject else {
            return Err(PluginError::Execution {
                plugin: self.id(),
                error: "subject has to be set".into(),
            });
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
        .map_err(|err| err.to_string())
        .map_err(PluginError::execution(self.id()))?;

        let result = self
            .plugin
            .run(&input)
            .map_err(PluginError::execution(self.id()))?;

        let output = BeforeSaveExperienceOutput::parse_from_bytes(&result)
            .map_err(|err| err.to_string())
            .map_err(PluginError::execution(self.id()))?;

        if let Some(error) = output.error.0 {
            return Err(PluginError::Output {
                plugin: self.id(),
                error: OutputError::new(error.code).with_message(error.message),
            });
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

    fn before_save_experience(&self) -> PluginGroup<Self::BeforeSaveExperience<'_>> {
        PluginGroup::new(self.retrieve().unwrap_or_default())
    }
}
