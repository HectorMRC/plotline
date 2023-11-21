/// Given an option and a value, compares them both if, and only if, the option is [Option::Some].
/// This macro calls to `return false;` if, and only if, the compared values are different.
/// Otherwise does nothing.
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

pub(crate) use equals_or_return;
