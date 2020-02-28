use serde::de::Deserializer;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use validator::validate_email;

/// An email address. It *should* conform with
/// [RFC 5322](https://tools.ietf.org/html/rfc5322) but is not heavily validated
/// against the specification (the best way to check is probably to try to send
/// an email address).
#[derive(Debug, Serialize, PartialEq)]
pub(crate) struct Email(String);

pub(crate) struct EmailParseError;

impl std::fmt::Display for EmailParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "failed to parse email")
    }
}

impl Email {
    fn new(s: &str) -> Result<Self, EmailParseError> {
        s.parse()
    }
}

impl FromStr for Email {
    type Err = EmailParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if validate_email(s) {
            Ok(Email(s.to_string()))
        } else {
            Err(EmailParseError)
        }
    }
}

impl<'de> Deserialize<'de> for Email {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        s.parse().map_err(serde::de::Error::custom)
    }
}
