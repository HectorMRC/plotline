#[macro_use]
extern crate serde;

#[cfg(feature = "cli")]
pub mod cli;
pub mod entity;
pub mod event;
pub mod snapshot;
pub mod timeline;

mod id;
mod interval;
mod name;
