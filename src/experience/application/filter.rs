use crate::{
    entity::Entity, event::Event, experience::Experience, id::Id, macros::equals_or_return,
};

/// Implements the filter query, through which zero o more experiences may be
/// retrived.
pub struct ExperienceFilter<Intv> {
    /// Determines the [Entity] involved in the experience, no matter it is
    /// before or after.
    pub(crate) entity: Option<Id<Entity>>,
    /// Determines the [Event] causing the [Experience].
    pub(crate) event: Option<Id<Event<Intv>>>,
}

impl<Intv> Default for ExperienceFilter<Intv> {
    fn default() -> Self {
        Self {
            entity: Default::default(),
            event: Default::default(),
        }
    }
}

impl<Intv> ExperienceFilter<Intv> {
    pub fn with_entity(mut self, id: Option<Id<Entity>>) -> Self {
        self.entity = id;
        self
    }

    pub fn with_event(mut self, id: Option<Id<Event<Intv>>>) -> Self {
        self.event = id;
        self
    }

    pub fn filter(&self, experience: &Experience<Intv>) -> bool {
        equals_or_return!(self.event, &experience.event);
        equals_or_return!(self.entity, &experience.entity);
        true
    }
}
