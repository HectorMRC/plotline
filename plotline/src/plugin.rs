use futures::future;
use std::{fmt::Display, marker::PhantomData};

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

impl PartialEq for PluginError {
    fn eq(&self, other: &Self) -> bool {
        self.code == other.code
    }
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
