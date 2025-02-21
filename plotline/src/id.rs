//! Identity definition.

/// An entity that can be uniquely identified.
pub trait Identify {
    type Id;

    fn id(&self) -> &Self::Id;
}

#[cfg(any(test, feature = "fixtures"))]
pub mod fixtures {
    use super::Identify;

    /// A mock implementation of the [`Identify`] trait.
    pub struct IndentifyMock<'a, Id> {
        pub id_fn: Option<fn() -> &'a Id>,
    }

    impl<Id> Identify for IndentifyMock<'_, Id> {
        type Id = Id;

        fn id(&self) -> &Self::Id {
            self.id_fn.expect("id method should be set")()
        }
    }
}
