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
    pub struct IndentifyMock<T> {
        value: T,
    }

    impl<T> Identify for IndentifyMock<T> {
        type Id = T;

        fn id(&self) -> &Self::Id {
            &self.value
        }
    }
}
