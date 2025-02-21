//! A subset of imports.

pub use crate::deref::{ReadOnly, ReadWrite, TryDeref, TryDerefMut, With, WithMut};
pub use crate::id::Identify;
pub use crate::property::Property;
pub use crate::schema::{
    ops::{
        delete::{AfterDelete, BeforeDelete},
        save::{AfterSave, BeforeSave},
    },
    plugin::Plugin,
    resource::Res,
    transaction::{Ctx, Target, Transaction},
    Error, Result, Schema,
};
