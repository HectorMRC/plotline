use crate::{
    entity::{self, Entity},
    id::Id,
    profile::Profile,
};

macro_rules! equals_or_return {
    ($option:expr, $subject:expr) => {
        if $option
            .as_ref()
            .map(|want| want != $subject)
            .unwrap_or_default()
        {
            return false;
        }
    };
}

/// Implements the filter query, through which zero o more profiles may be retrived.
#[derive(Default)]
pub struct ProfileFilter {
    entity: Option<Id<Entity>>,
}

impl ProfileFilter {
    pub fn with_entity(mut self, id: Option<Id<Entity>>) -> Self {
        self.entity = id;
        self
    }

    pub fn filter(&self, profile: &Profile) -> bool {
        equals_or_return!(self.entity, &profile.entity);
        true
    }
}
