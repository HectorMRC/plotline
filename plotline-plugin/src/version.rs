use crate::Error;
use std::{fmt::Display, str::FromStr};

/// PluginVersion represents the semantic version of a plugin.
#[derive(Clone)]
pub struct PluginVersion(semver::Version);

impl FromStr for PluginVersion {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(Self(semver::Version::from_str(s)?))
    }
}

impl Display for PluginVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
