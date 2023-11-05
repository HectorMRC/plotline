use crate::interval::Interval;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::{fmt::Display, num::ParseIntError, str::FromStr};

const PERIOD_SEPARATOR: &str = ":";

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    ParseIntError(#[from] ParseIntError),
}

/// A Period is the time being between two different moments in time.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Period<M>([M; 2]);

impl<M> Interval for Period<M>
where
    M: Eq + Ord + Copy,
{
    type Bound = M;

    fn lo(&self) -> Self::Bound {
        self.0[0].clone()
    }

    fn hi(&self) -> Self::Bound {
        self.0[1].clone()
    }
}

impl<M> Serialize for Period<M>
where
    M: Serialize + Display,
{
    /// Serializes a [Period] as a string.
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.collect_str(&format!(
            "{}{PERIOD_SEPARATOR}{}",
            self.0[0].to_string(),
            self.0[1].to_string()
        ))
    }
}

impl<'de> Deserialize<'de> for Period<usize> {
    /// Deserializes a [Period] from a string.
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error;

        let value = String::deserialize(deserializer).map_err(Error::custom)?;
        value.try_into().map_err(Error::custom)
    }
}

impl Display for Period<usize> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.0[0], self.0[1])
    }
}

impl TryFrom<String> for Period<usize> {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let bounds: Vec<String> = value
            .split(":")
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();

        let mut bounds = bounds.into_iter();
        let lo = bounds.next().unwrap_or_default().parse::<usize>()?;
        let hi = bounds
            .next()
            .map(|s| s.parse::<usize>())
            .transpose()?
            .unwrap_or(lo.clone());

        Ok(Self([lo, hi]))
    }
}
