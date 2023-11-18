mod filter;
pub use filter::*;

use super::{error::Result, Profile};
use crate::{id::Id, transaction::Tx};
use std::sync::Arc;

pub trait ProfileRepository {
    type Tx: Tx<Profile>;

    fn filter(&self, filter: &ProfileFilter) -> Result<Vec<Self::Tx>>;
    fn create(&self, profile: &Profile) -> Result<()>;
}
