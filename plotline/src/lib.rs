pub mod entity;
pub mod error;
pub mod event;
pub mod experience;
pub mod id;
pub mod interval;
pub mod name;
#[cfg(feature = "in_memory")]
pub mod snapshot;

mod macros;
mod period;
#[cfg(feature = "in_memory")]
mod resource;
#[cfg(feature = "in_memory")]
mod serde;
mod transaction;

/// Given an [Option] and a variable, sets to that variable the inner value of
/// the [Option::Some], if it is so. Otherwise the variable gets unchanged.
#[inline]
fn assign_some_or_ignore<T>(from: Option<T>, to: &mut T) {
    if let Some(value) = from {
        *to = value;
    }
}
