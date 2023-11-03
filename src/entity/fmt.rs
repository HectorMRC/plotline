use super::Entity;
use std::{fmt::Display, marker::PhantomData};

macro_rules! row_format {
    () => {
        "{: <15} {: <40}"
    };
}

/// Displays the [Entity] in a single line.
pub struct Row;
/// Displays the [Entity] in different lines.
pub struct Column;

/// Implements diffent strategies of [Display] for [Entity].
pub struct EntityFmt<'a, S> {
    style: PhantomData<S>,
    entity: &'a Entity,
}

impl<'a> Display for EntityFmt<'a, Row> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            row_format!(),
            self.entity.name.to_string(),
            self.entity.id.to_string(),
        )
    }
}

impl<'a> Display for EntityFmt<'a, Column> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{: <10} {}", "NAME", self.entity.name)?;
        writeln!(f, "{: <10} {}", "UUID", self.entity.id)
    }
}

impl<'a> EntityFmt<'a, Row> {
    /// Returns the string of headers corresponding to the row-like display.
    pub fn headers() -> String {
        format!(row_format!(), "NAME", "UUID")
    }

    /// Returns an instance of [EntityFmt] that displays the given entity in a single line.
    pub fn row(entity: &'a Entity) -> Self {
        EntityFmt {
            style: PhantomData,
            entity,
        }
    }
}

impl<'a> EntityFmt<'a, Column> {
    /// Returns an instance of [EntityFmt] that displays the given entity in different lines.
    pub fn column(entity: &'a Entity) -> Self {
        EntityFmt {
            style: PhantomData,
            entity,
        }
    }
}
