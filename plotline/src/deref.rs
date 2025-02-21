//! Deref definition.

/// A type that may be immutably dereferenced.
pub trait TryDeref {
    type Target;

    /// Tries to dereferences the value.
    fn try_deref(&self) -> Option<&Self::Target>;
}

/// A type that may be mutably dereferenced.
pub trait TryDerefMut: TryDeref {
    /// Tries to dereferences the value.
    fn try_deref_mut(&mut self) -> Option<&mut Self::Target>;
}

/// Represents a value that can be acquired for reading.
pub trait ReadOnly {
    type Target;
    type Guard<'a>: TryDeref<Target = Self::Target>
    where
        Self: 'a;

    /// Acquires the read-only guard of this value.
    fn read(&self) -> Self::Guard<'_>;
}

/// Represents a value that can be acquired for reading and writing.
pub trait ReadWrite {
    type Target;
    type Guard<'a>: TryDeref<Target = Self::Target> + TryDerefMut
    where
        Self: 'a;

    /// Acquires the read-only guard of this value.
    fn write(&self) -> Self::Guard<'_>;
}

/// Represents a value that can be accessed within a scope.
pub trait With {
    type Target<'a>;

    /// Gets a read-only access to the resource and executes the given closure.
    fn with<F, R>(&self, f: F) -> Option<R>
    where
        F: for<'a> FnOnce(Self::Target<'a>) -> R;
}

/// Represents a value that can be accessed and mutated within a scope.
pub trait WithMut {
    type Target<'a>;

    /// Gets a read-write access to the resouce and executes the given closure.
    fn with_mut<F, R>(&self, f: F) -> Option<R>
    where
        F: for<'a> FnOnce(Self::Target<'a>) -> R;
}

impl<T, U> With for T
where
    T: ReadOnly<Target = U>,
    T::Target: 'static,
{
    type Target<'a> = &'a U;

    fn with<F, R>(&self, f: F) -> Option<R>
    where
        F: FnOnce(&T::Target) -> R,
    {
        self.read().try_deref().map(f)
    }
}

impl<T, U> WithMut for T
where
    T: ReadWrite<Target = U>,
    T::Target: 'static,
{
    type Target<'a> = &'a mut U;

    fn with_mut<F, R>(&self, f: F) -> Option<R>
    where
        F: FnOnce(&mut T::Target) -> R,
    {
        self.write().try_deref_mut().map(f)
    }
}

macro_rules! impl_with {
    ($($t:ident $i:tt),*) => {
        #[allow(non_snake_case)]
        impl<$($t,)*> With for ($($t,)*)
        where
            $($t: ReadOnly, $t::Target: 'static,)*
        {
            type Target<'a> = ($(&'a $t::Target,)*);

            fn with<_F, _R>(&self, f: _F) -> Option<_R>
            where
                _F: FnOnce(Self::Target<'_>) -> _R
            {
                $(let $t = self.$i.read();)*

                let target = (
                    $($t.try_deref()?),*
                );

                Some(f(target))
            }
        }

        #[allow(non_snake_case)]
        impl<$($t,)*> WithMut for ($($t,)*)
        where
            $($t: ReadWrite, $t::Target: 'static,)*
        {
            type Target<'a> = ($(&'a mut $t::Target,)*);

            fn with_mut<_F, _R>(&self, f: _F) -> Option<_R>
            where
                _F: FnOnce(Self::Target<'_>) -> _R
            {
                $(let mut $t = self.$i.write();)*

                let target = (
                    $($t.try_deref_mut()?),*
                );

                Some(f(target))
            }
        }
    };
}

impl_with!(A 0, B 1);
impl_with!(A 0, B 1, C 2);
impl_with!(A 0, B 1, C 2, D 3);
impl_with!(A 0, B 1, C 2, D 3, E 4);
impl_with!(A 0, B 1, C 2, D 3, E 4, F 5);
