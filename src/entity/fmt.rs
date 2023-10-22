use super::Entity;
use std::{fmt::Display, marker::PhantomData};

macro_rules! row_format {
    () => {
        "{: <15} {: <40} {: <20}"
    };
}

/// Displays a single row with the information of the [Entity].
pub struct Row;
/// Displays every field of the [Entity] in its own row.
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
            self.entity.tags.to_string()
        )
    }
}

impl<'a> Display for EntityFmt<'a, Column> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{: <10} {}", "NAME", self.entity.name)?;
        writeln!(f, "{: <10} {}", "UUID", self.entity.id)?;
        writeln!(f, "{: <10} {}", "TAGS", self.entity.tags)
    }
}

impl<'a> EntityFmt<'a, Row> {
    /// Returns a string containing the headers.
    pub fn headers() -> String {
        format!(row_format!(), "NAME", "UUID", "TAGS")
    }

    pub fn row(entity: &'a Entity) -> Self {
        EntityFmt {
            style: PhantomData,
            entity,
        }
    }
}

impl<'a> EntityFmt<'a, Column> {
    pub fn column(entity: &'a Entity) -> Self {
        EntityFmt {
            style: PhantomData,
            entity,
        }
    }
}
