//! The application service.

mod save;
pub use save::*;

mod filter;
pub use filter::*;

mod find;
pub use find::*;

mod remove;
pub use remove::*;

use super::{error::Result, Entity};
use crate::{id::Indentify, transaction::Tx};
use std::sync::Arc;

#[trait_variant::make]
pub trait EntityRepository {
    type Tx: Tx<Entity>;

    async fn find(&self, id: <Entity as Indentify>::Id) -> Result<Self::Tx>;
    async fn filter(&self, filter: &EntityFilter) -> Result<Vec<Self::Tx>>;
    async fn create(&self, entity: &Entity) -> Result<()>;
    async fn delete(&self, id: <Entity as Indentify>::Id) -> Result<()>;
}

pub struct EntityApplication<EntityRepo> {
    pub entity_repo: Arc<EntityRepo>,
}
