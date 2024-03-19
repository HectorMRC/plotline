use super::{
    BeforeSaveExperience, Command, ExperienceApplication, ExperienceFilter, ExperienceRepository,
    PluginFactory,
};
use crate::{
    entity::{application::EntityRepository, Entity},
    event::{application::EventRepository, Event},
    experience::{query, Error, Experience, ExperienceBuilder, Profile, Result},
    id::{Id, Identifiable},
    interval::Interval,
    transaction::Tx,
};
use std::{ops::Deref, sync::Arc};

/// Implements the save experience transaction.
pub struct SaveExperience<ExperienceRepo, EntityRepo, EventRepo, PluginFcty>
where
    EventRepo: EventRepository,
{
    experience_repo: Arc<ExperienceRepo>,
    entity_repo: Arc<EntityRepo>,
    event_repo: Arc<EventRepo>,
    plugin_factory: Arc<PluginFcty>,
    id: Id<Experience<EventRepo::Intv>>,
    entity: Option<Id<Entity>>,
    event: Option<Id<Event<EventRepo::Intv>>>,
    profiles: Option<Vec<Profile>>,
}

impl<ExperienceRepo, EntityRepo, EventRepo, PluginFcty>
    SaveExperience<ExperienceRepo, EntityRepo, EventRepo, PluginFcty>
where
    EventRepo: EventRepository,
{
    pub fn with_profiles(mut self, profiles: Option<Vec<Profile>>) -> Self {
        self.profiles = profiles;
        self
    }

    pub fn with_entity(mut self, entity: Option<Id<Entity>>) -> Self {
        self.entity = entity;
        self
    }

    pub fn with_event(mut self, event: Option<Id<Event<EventRepo::Intv>>>) -> Self {
        self.event = event;
        self
    }
}

impl<ExperienceRepo, EntityRepo, EventRepo, PluginFcty>
    SaveExperience<ExperienceRepo, EntityRepo, EventRepo, PluginFcty>
where
    ExperienceRepo: ExperienceRepository<Intv = EventRepo::Intv>,
    EntityRepo: EntityRepository,
    EventRepo: EventRepository,
    PluginFcty: PluginFactory<Intv = EventRepo::Intv>,
{
    /// Executes the save experience transaction.
    pub fn execute(self) -> Result<()> {
        match self.experience_repo.find(self.id) {
            Ok(experience_tx) => self.update(experience_tx),
            Err(Error::NotFound) => self.create(),
            Err(err) => Err(err),
        }
    }

    fn create(self) -> Result<()> {
        let entity_tx = self
            .entity_repo
            .find(self.entity.ok_or(Error::MandatoryField("entity"))?)?;

        let entity = entity_tx.read();

        let event_tx = self
            .event_repo
            .find(self.event.ok_or(Error::MandatoryField("event"))?)?;

        let event = event_tx.read();

        let experiences_txs = self
            .experience_repo
            .filter(&ExperienceFilter::default().with_entity(self.entity))?
            .into_iter()
            .collect::<Vec<_>>();

        let mut experiences = Vec::with_capacity(experiences_txs.len());
        for experience_tx in &experiences_txs {
            experiences.push(experience_tx.read())
        }

        let experience = ExperienceBuilder::new(entity.clone(), event.clone())
            .with_id(self.id)
            .with_profiles(self.profiles)
            .with_fallbacks(experiences.iter().map(Deref::deref))
            .build()?;

        self.plugin_factory
            .before_save_experience()
            .with_subject(&experience)
            .with_timeline(&experiences.iter().map(Deref::deref).collect::<Vec<_>>())
            .execute()
            .result()?;

        self.experience_repo.create(&experience)?;
        Ok(())
    }

    fn update(self, _experience_tx: ExperienceRepo::Tx) -> Result<()> {
        Ok(())
    }
}

impl<Intv> ExperienceBuilder<Intv>
where
    Intv: Interval,
{
    /// Tries to compute some value for those fields set to [Option::None].
    fn with_fallbacks<'a, I>(mut self, experiences: I) -> Self
    where
        I: Iterator<Item = &'a Experience<Intv>>,
        Intv: 'a,
    {
        let mut previous = query::SelectPreviousExperience::new(&self.event);
        let mut next = query::SelectNextExperience::new(&self.event);
        for experience in experiences {
            previous = previous.with(experience);
            next = next.with(experience);
        }

        self.profiles = self.profiles.or_else(|| {
            previous
                .value()
                .or_else(|| next.value())
                .and_then(|experience| {
                    experience
                        .profiles
                        .iter()
                        .find(|profile| profile.entity.id() == self.entity.id())
                        .cloned()
                })
                .map(|profile| vec![profile])
        });

        self
    }
}

impl<ExperienceRepo, EntityRepo, EventRepo, PluginFcty>
    ExperienceApplication<ExperienceRepo, EntityRepo, EventRepo, PluginFcty>
where
    ExperienceRepo: ExperienceRepository<Intv = EventRepo::Intv>,
    EntityRepo: EntityRepository,
    EventRepo: EventRepository,
{
    pub fn save_experience(
        &self,
        id: Id<Experience<EventRepo::Intv>>,
    ) -> SaveExperience<ExperienceRepo, EntityRepo, EventRepo, PluginFcty> {
        SaveExperience {
            experience_repo: self.experience_repo.clone(),
            entity_repo: self.entity_repo.clone(),
            event_repo: self.event_repo.clone(),
            plugin_factory: self.plugin_factory.clone(),
            id,
            entity: Default::default(),
            event: Default::default(),
            profiles: Default::default(),
        }
    }
}
