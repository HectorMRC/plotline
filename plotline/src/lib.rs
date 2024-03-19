pub mod entity;
pub mod error;
pub mod event;
pub mod experience;
pub mod id;
pub mod interval;
pub mod name;
#[cfg(feature = "in_memory")]
pub mod snapshot;

mod command;
mod macros;
mod period;
#[cfg(feature = "in_memory")]
mod resource;
#[cfg(feature = "in_memory")]
mod serde;
mod transaction;

/// Given a mutable reference of T, and an [Option] of the same type, updates
/// the reference with the inner value of the [Option::Some], if it is so.
/// Otherwise leaves the reference unchanged.
#[inline]
fn update_if_some<T>(to: &mut T, from: Option<T>) {
    if let Some(value) = from {
        *to = value;
    }
}
