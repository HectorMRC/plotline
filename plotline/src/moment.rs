use std::fmt::Display;

use serde::{Deserialize, Serialize};

/// A Moment represents a specific moment in time.
#[derive(Debug, Default, Eq, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Moment(usize);

impl Ord for Moment {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

impl PartialOrd for Moment {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Display for Moment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<String> for Moment {
    type Error = std::num::ParseIntError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(Moment(value.parse()?))
    }
}

impl Moment {
    pub fn new(value: usize) -> Self {
        Self(value)
    }
}
