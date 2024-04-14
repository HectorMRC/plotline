use crate::{Error, Result};
use std::str::FromStr;

/// A PluginId uniquely identifies a plugin.
#[derive(Hash, PartialEq, Eq, Clone)]
pub struct PluginId(String);

impl FromStr for PluginId {
    type Err = Error;

    fn from_str(value: &str) -> Result<Self> {
        let is_invalid_char = |c: char| -> bool {
            const INVALID_CHARS: [char; 3] = ['\n', '\r', ' '];
            INVALID_CHARS.contains(&c)
        };

        if value.is_empty() || value.contains(is_invalid_char) {
            return Err(Error::NotAPluginId);
        }

        Ok(PluginId(value.to_string()))
    }
}

impl From<PluginId> for String {
    fn from(value: PluginId) -> Self {
        value.0
    }
}
