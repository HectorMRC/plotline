use futures::future;
use std::{fmt::Display, marker::PhantomData, str::FromStr};

/// Wraps in a single type woth results, the execution one, which determines
/// the successfull execution of a plugin, and the output one, which is the
/// actual result of the plugin.
pub type Result<T> = std::result::Result<std::result::Result<T, PluginError>, ExecutionError>;

/// An ExecutionError may occurs when executing a plugin (e.g. it panics).
#[derive(Clone)]
pub struct ExecutionError(String);

impl<T: AsRef<str>> From<T> for ExecutionError {
    fn from(value: T) -> Self {
        Self(value.as_ref().to_string())
    }
}

impl Display for ExecutionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// An Error that may be returned as the output of a plugin.
#[derive(Debug, Clone)]
pub struct PluginError {
    pub code: String,
    pub message: String,
}

impl PluginError {
    pub fn new(code: impl AsRef<str>) -> Self {
        Self {
            code: code.as_ref().to_string(),
            message: Default::default(),
        }
    }

    pub fn with_message(mut self, msg: impl AsRef<str>) -> Self {
        self.message = msg.as_ref().to_string();
        self
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("invalid plugin id")]
    NotAnId,
    #[error("invalid plugin version: {0}")]
    NotAVersion(#[from] semver::Error),
}

/// A PluginId uniquely identifies a plugin.
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct PluginId(String);

impl FromStr for PluginId {
    type Err = Error;

    /// A PluginId must consist of a single word string.
    fn from_str(value: &str) -> std::result::Result<Self, Self::Err> {
        let is_invalid_char = |c: char| -> bool {
            const INVALID_CHARS: [char; 3] = ['\n', '\r', ' '];
            !c.is_ascii() || INVALID_CHARS.contains(&c)
        };

        if value.is_empty() || value.contains(is_invalid_char) {
            return Err(Error::NotAnId);
        }

        Ok(Self(value.to_string()))
    }
}

impl AsRef<str> for PluginId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

/// PluginVersion represents the semantic version of a plugin.
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

/// A command represents whatever that can be executed and may
/// return a value as result.
#[trait_variant::make]
pub trait Command<T> {
    async fn execute(self) -> Self;
    fn result(&self) -> Result<T>;
}

/// A PluginGroup represents a group of plugins of the same kind.
pub struct PluginGroup<T, R = ()> {
    plugins: Vec<T>,
    _result: PhantomData<R>,
}

impl<T, R> PluginGroup<T, R>
where
    T: Command<R>,
{
    pub async fn execute(mut self) -> Self {
        self.plugins = future::join_all(
            self.plugins
                .into_iter()
                .map(|plugin| async { plugin.execute().await }),
        )
        .await;

        self
    }
}

impl<T, R> PluginGroup<T, R> {
    pub fn new(plugins: Vec<T>) -> Self {
        Self {
            plugins,
            _result: PhantomData,
        }
    }

    pub fn map<F>(mut self, f: F) -> Self
    where
        F: Fn(T) -> T,
    {
        self.plugins = self.plugins.into_iter().map(f).collect();
        self
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::{Error, PluginId};

    #[test]
    fn plugin_id_from_string() {
        struct Test<'a> {
            name: &'a str,
            plugin_id: &'a str,
            must_fail: bool,
        }

        vec![
            Test {
                name: "An emty string is not a valid plugin id",
                plugin_id: "",
                must_fail: true,
            },
            Test {
                name: "An string with line feed is not a valid plugin id",
                plugin_id: "entity\nname",
                must_fail: true,
            },
            Test {
                name: "An string with carriage return is not a valid plugin id",
                plugin_id: "entity\rname",
                must_fail: true,
            },
            Test {
                name: "An string with carriage return plus line feed is not a valid plugin id",
                plugin_id: "entity\r\nname",
                must_fail: true,
            },
            Test {
                name: "An string with line feed plus carriage is not a valid plugin id",
                plugin_id: "entity\n\rname",
                must_fail: true,
            },
            Test {
                name: "A multi word single line string is not a valid plugin id",
                plugin_id: "abc 123#[]-_*&^",
                must_fail: true,
            },
            Test {
                name: "A single word non-ascii string is not a valid plugin id",
                plugin_id: "abc1234Ͼ",
                must_fail: true,
            },
            Test {
                name: "A single word ascii string is a valid plugin id",
                plugin_id: "abc123#[]-_*&^",
                must_fail: false,
            },
        ]
        .into_iter()
        .for_each(|test| {
            let result = PluginId::from_str(test.plugin_id);
            assert_eq!(result.is_err(), test.must_fail, "{}", test.name);

            match result {
                Ok(id) => assert_eq!(id.as_ref(), test.plugin_id, "{}", test.name),
                Err(err) => assert!(matches!(err, Error::NotAnId), "{}", test.name),
            }
        });
    }
}
