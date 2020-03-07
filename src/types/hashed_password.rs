use serde::de::Deserializer;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

pub(crate) const HASHED_PASSWORD_LENGTH: usize = 32;

/// A hashed password. The client is responsible for sending the entered
/// password in hashed form. Recommended to use `SHA-256` or stronger hash
/// function.
///
/// It is required that the hashed password have exactly 32-bytes (exactly 32
/// ASCII 1-byte characters, so length must be 32).
#[derive(Debug, Serialize, PartialEq, Clone)]
pub(crate) struct HashedPassword(String);

pub(crate) struct HashedPasswordParseError;

impl std::fmt::Display for HashedPasswordParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "failed to parse hashed password")
    }
}

impl HashedPassword {
    /// Attempt to construct a new `HashedPassword` from a given `str`. It is
    /// required that `s.len() == HASHED_PASSWORD_LENGTH`,
    /// where `HASHED_PASSWORD_LENGTH == 32`.
    pub(crate) fn new(s: &str) -> Result<Self, HashedPasswordParseError> {
        s.parse()
    }

    /// Extract the password.
    pub(crate) fn password(&self) -> &str {
        &self.0
    }
}

impl FromStr for HashedPassword {
    type Err = HashedPasswordParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() == HASHED_PASSWORD_LENGTH {
            Ok(HashedPassword(s.to_string()))
        } else {
            Err(HashedPasswordParseError)
        }
    }
}

impl<'de> Deserialize<'de> for HashedPassword {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        s.parse().map_err(serde::de::Error::custom)
    }
}
