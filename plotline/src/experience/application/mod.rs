//! The application service.

mod save;
pub use save::*;

mod filter;
pub use filter::*;

mod find;
pub use find::*;

use super::{constraint::Constraint, error::Result, ExperiencedEvent};
use crate::{experience::Experience, id::Id, interval::Interval, transaction::Tx};
use std::{marker::PhantomData, sync::Arc};

pub trait ExperienceRepository {
    type Interval: Interval;
    type Tx: Tx<Experience<Self::Interval>>;

    fn find(&self, id: Id<Experience<Self::Interval>>) -> Result<Self::Tx>;
    fn filter(&self, filter: &ExperienceFilter<Self::Interval>) -> Result<Vec<Self::Tx>>;
    fn create(&self, experience: &Experience<Self::Interval>) -> Result<()>;
    fn delete(&self, id: Id<Experience<Self::Interval>>) -> Result<()>;
}

pub trait ConstraintFactory<Intv> {
    fn new<'a>(event: &'a ExperiencedEvent<'a, Intv>) -> impl Constraint<'a, Intv>;
}

pub struct ExperienceApplication<ExperienceRepo, EntityRepo, EventRepo, CnstFactory> {
    pub experience_repo: Arc<ExperienceRepo>,
    pub entity_repo: Arc<EntityRepo>,
    pub event_repo: Arc<EventRepo>,
    pub cnst_factory: PhantomData<CnstFactory>,
}
