use fp_bindgen::prelude::Serializable;
use serde::{
    de::{self, Visitor},
    Deserialize, Serialize,
};
use std::fmt::{self, Display};
use std::str::FromStr;
use std::{convert::TryFrom, ops::Deref};
use thiserror::Error;

const MAX_LENGTH: usize = 63;
const MIN_LENGTH: usize = 1;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum InvalidName {
    #[error("name is too long")]
    TooLong,
    #[error("name contains invalid characters (names can only include lowercase ASCII letters, numbers, and dashes)")]
    InvalidCharacters,
    #[error("name cannot be an empty string")]
    TooShort,
    #[error("name must start and end with an alphanumeric character")]
    NonAlphanumericStartOrEnd,
}

/// This is a user-specified name for a Fiberplane resource.
///
/// Names must:
/// - be between 1 and 63 characters long
/// - start and end with an alphanumeric character
/// - contain only lowercase alphanumeric ASCII characters and dashes
///
/// Names must be unique within a namespace such as a Workspace.
#[derive(Debug, Clone, Serialize, Hash, PartialEq, Eq, Serializable)]
#[cfg_attr(feature = "sqlx", derive(sqlx::Type), sqlx(transparent))]
#[fp(
    rust_plugin_module = "fiberplane::protocols::names",
    rust_wasmer_runtime_module = "fiberplane::protocols::names"
)]
pub struct Name(String);

impl Name {
    pub fn new(name: impl Into<String>) -> Result<Self, InvalidName> {
        let name = name.into();
        Self::validate(&name).map(|()| Name(name))
    }

    pub fn into_string(self) -> String {
        self.0
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Creates a name from a static string.
    ///
    /// # Panics
    ///
    /// This function panics if the name is invalid.
    pub fn from_static(name: &'static str) -> Self {
        Name::new(name).expect("Invalid name")
    }

    pub fn validate(name: &str) -> Result<(), InvalidName> {
        // Check the length
        if name.len() < MIN_LENGTH {
            return Err(InvalidName::TooShort);
        }
        if name.len() > MAX_LENGTH {
            return Err(InvalidName::TooLong);
        }

        // Check the characters
        if name
            .chars()
            .any(|c| !c.is_ascii_lowercase() && !c.is_numeric() && c != '-')
        {
            return Err(InvalidName::InvalidCharacters);
        }

        // Check the first and last characters
        let first = name.chars().next().unwrap();
        let last = name.chars().last().unwrap();
        if !first.is_ascii_alphanumeric() || !last.is_ascii_alphanumeric() {
            return Err(InvalidName::NonAlphanumericStartOrEnd);
        }

        Ok(())
    }
}

struct NameVisitor;

impl<'de> Visitor<'de> for NameVisitor {
    type Value = Name;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a valid name to identify the resource")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match Name::validate(value) {
            Ok(()) => Ok(Name(value.to_owned())),
            Err(error) => Err(de::Error::custom(error.to_string())),
        }
    }
}

impl<'de> Deserialize<'de> for Name {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(NameVisitor)
    }
}

impl Display for Name {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl TryFrom<String> for Name {
    type Error = InvalidName;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl TryFrom<&str> for Name {
    type Error = InvalidName;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl FromStr for Name {
    type Err = InvalidName;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

impl From<Name> for String {
    fn from(name: Name) -> Self {
        name.0
    }
}

impl Deref for Name {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl PartialEq<str> for Name {
    fn eq(&self, other: &str) -> bool {
        self.0 == other
    }
}

impl PartialEq<&str> for Name {
    fn eq(&self, other: &&str) -> bool {
        self.0 == *other
    }
}

impl PartialEq<Name> for &str {
    fn eq(&self, other: &Name) -> bool {
        *self == other.0
    }
}

impl PartialEq<Name> for str {
    fn eq(&self, other: &Name) -> bool {
        self == other.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn name_creation() {
        assert!(Name::new("abcdefghijklmnopqrstuvwxyz-1234567890").is_ok());
        assert!(Name::new("a".repeat(63)).is_ok());

        assert_eq!(Name::new("a".repeat(64)), Err(InvalidName::TooLong));
        assert_eq!(Name::new(""), Err(InvalidName::TooShort));
        assert_eq!(Name::new("a_b"), Err(InvalidName::InvalidCharacters));
        assert_eq!(Name::new("ABC"), Err(InvalidName::InvalidCharacters));
        assert_eq!(Name::new("hi\n there"), Err(InvalidName::InvalidCharacters));
        assert_eq!(Name::new("hi:there"), Err(InvalidName::InvalidCharacters));
        assert_eq!(Name::new("a\u{00A7}b"), Err(InvalidName::InvalidCharacters));
        assert_eq!(
            Name::new("-hi-there"),
            Err(InvalidName::NonAlphanumericStartOrEnd)
        );
    }

    #[test]
    fn name_serialization_deserialization() {
        let name = Name::new("abcdefghijklmnopqrstuvwxyz-1234567890").unwrap();
        let serialized = serde_json::to_string(&name).unwrap();
        let deserialized: Name = serde_json::from_str(&serialized).unwrap();
        assert_eq!(name, deserialized);

        serde_json::from_str::<Name>("\"hi:there\"").unwrap_err();
        serde_json::from_str::<Name>(r#""hi_there""#).unwrap_err();
    }

    #[test]
    fn name_deserialization_error() {
        assert_eq!(
            serde_json::from_str::<Name>("\"-hi-there\"").map_err(|error| error.to_string()),
            Err(
                "name must start and end with an alphanumeric character at line 1 column 11"
                    .to_owned()
            )
        );
    }
}
