//! Chain of arbitrary types.

/// A last-in first-out chain.
#[derive(Debug, Default)]
pub struct LiFoChain<T, Head> {
    /// The precursor value in the chain.
    ///
    /// This field must be of the type of [`LiFoChain`] for non terminal instances.
    /// Any other value will break the continuity of the chain.
    pub head: Head,
    /// The subject value in this chain's link.
    pub value: T,
}

impl<T> LiFoChain<T, ()> {
    /// Returns a terminal chain with the given value.
    ///
    /// A chain is terminal if, and only if, it's head type is other than [`LiFoChain`].
    /// By default, the unitary is being used.
    pub fn terminal(value: T) -> Self {
        LiFoChain { head: (), value }
    }
}

impl<T, Head> LiFoChain<T, Head> {
    /// Chains the given value with self.
    pub fn chain<U>(self, value: U) -> LiFoChain<U, Self> {
        LiFoChain { head: self, value }
    }
}

#[cfg(test)]
mod tests {
    use super::LiFoChain;

    #[test]
    #[allow(clippy::unit_cmp)]
    fn lifo_chain_handles_arbitrary_types() {
        #[derive(Debug, PartialEq, Eq)]
        struct Foo;

        let chain = LiFoChain::terminal("").chain(123).chain(true).chain(Foo);

        assert_eq!(chain.value, Foo);
        assert!(chain.head.value);
        assert_eq!(chain.head.head.value, 123);
        assert_eq!(chain.head.head.head.value, "");
        assert_eq!(chain.head.head.head.head, ());
    }
}
