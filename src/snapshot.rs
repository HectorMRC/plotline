use crate::entity::repository::InMemoryEntityRepository;
use std::sync::Arc;

pub trait Serderable: serde::ser::Serialize + serde::de::DeserializeOwned {}

#[derive(Serialize, Deserialize)]
pub struct Snapshot<E> {
    pub entities: Arc<E>,
}

impl Snapshot<InMemoryEntityRepository> {
    pub fn parse<D>(de: D) -> Self
    where
        D: Fn() -> Self,
    {
        de()
    }
}
