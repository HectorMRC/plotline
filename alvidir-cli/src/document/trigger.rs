use super::ProxyTrigger;

pub struct AlwaysTrigger;

impl Default for AlwaysTrigger {
    fn default() -> Self {
        Self
    }
}

impl ProxyTrigger for AlwaysTrigger {
    fn update(&self) -> bool {
        true
    }
}