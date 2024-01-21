/// Given an option and a value, compares them both if, and only if, the option
/// is [Option::Some]. This macro calls to `return false;` if, and only if, the
/// compared values are different. Otherwise does nothing.
macro_rules! equals_or_return {
    ($option:expr, $subject:expr) => {
        if $option
            .as_ref()
            .map(|want| want != $subject)
            .unwrap_or_default()
        {
            return false;
        }
    };
}

/// Given the constraint for a type that implements the [Interval] trait,
/// implements the [Ord] and [PartialOrd] traits for that same type.
macro_rules! impl_interval_based_ord_for {
    ($type:ty where $generic:ident: $trait:ident) => {
        impl<$generic> Ord for $type
        where
            Self: Interval,
            $generic: $trait,
        {
            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                self.lo().cmp(&other.lo())
            }
        }

        impl<$generic> PartialOrd for $type
        where
            Self: Interval,
            $generic: $trait,
        {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }
    };
}

pub(crate) use equals_or_return;
pub(crate) use impl_interval_based_ord_for;
