use alvidir::document::proxy::ProxyTrigger;

#[derive(Default)]
pub struct AlwaysTrigger;

impl ProxyTrigger for AlwaysTrigger {
    fn update(&self) -> bool {
        true
    }
}
