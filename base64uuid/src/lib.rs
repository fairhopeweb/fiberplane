use base64::DecodeError;
use serde::{de, Deserialize, Serialize};
use std::convert::TryFrom;
use std::fmt::{self, Debug, Display, Formatter};
use std::str::FromStr;
use thiserror::Error;
use uuid::Uuid;

#[cfg(test)]
mod tests;

/// A Base64Uuid is a base64url-encoded UUID that is
/// used for IDs across Fiberplane's products.
/// The main motivation is that it is a slightly more
/// condensed representation of a UUID and it looks
/// better in URLs.
#[derive(PartialEq, Hash, Eq, Serialize, Clone, Copy)]
// Force serde to go through the String representation to ensure
// it is serialized in the base64url-encoded format
#[serde(into = "String")]
pub struct Base64Uuid(Uuid);

impl Base64Uuid {
    pub fn new() -> Self {
        let uuid = Uuid::new_v4();
        Base64Uuid(uuid)
    }

    pub fn nil() -> Self {
        Base64Uuid(Uuid::nil())
    }

    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }

    pub fn to_uuid(self) -> Uuid {
        self.0
    }

    pub fn into_uuid(self) -> Uuid {
        self.0
    }

    /// Parse an ID either from the normal UUID string representation
    /// or from the base64url-encoded representation
    pub fn parse_str(s: &str) -> Result<Self, InvalidId> {
        let uuid = if s.len() == 22 {
            // This will return an error if the string is invalid
            let bytes = decode_base64(s).map_err(|_| InvalidId)?;
            Uuid::from_slice(&bytes).map_err(|_| InvalidId)?
        } else if s.len() == 36 {
            Uuid::from_str(s).map_err(|_| InvalidId)?
        } else {
            return Err(InvalidId);
        };

        Ok(Base64Uuid(uuid))
    }
}

impl<'de> Deserialize<'de> for Base64Uuid {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct Base64UuidVisitor;

        impl<'vi> de::Visitor<'vi> for Base64UuidVisitor {
            type Value = Base64Uuid;

            fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
                formatter.write_str("a base64url-encoded UUID")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Base64Uuid::parse_str(value).map_err(de::Error::custom)
            }

            fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                self.visit_str(&value)
            }
        }

        deserializer.deserialize_str(Base64UuidVisitor)
    }
}

impl AsRef<Uuid> for Base64Uuid {
    fn as_ref(&self) -> &Uuid {
        &self.0
    }
}

impl From<Uuid> for Base64Uuid {
    fn from(uuid: Uuid) -> Self {
        Base64Uuid(uuid)
    }
}

impl From<Base64Uuid> for Uuid {
    fn from(base64: Base64Uuid) -> Self {
        base64.into_uuid()
    }
}

impl From<Base64Uuid> for String {
    fn from(uuid: Base64Uuid) -> Self {
        uuid.to_string()
    }
}

impl FromStr for Base64Uuid {
    type Err = InvalidId;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Base64Uuid::parse_str(s)
    }
}

impl TryFrom<&str> for Base64Uuid {
    type Error = InvalidId;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Base64Uuid::parse_str(s)
    }
}

impl Display for Base64Uuid {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(&encode_base64(&self.0.as_bytes()))
    }
}

impl Debug for Base64Uuid {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(&encode_base64(&self.0.as_bytes()))
    }
}

impl Default for Base64Uuid {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Error)]
#[error("Invalid Base64Uuid")]
pub struct InvalidId;

/// Encode input to a base64 string using the UrlSafe characters set and no
/// padding.
fn encode_base64<T: AsRef<[u8]>>(input: T) -> String {
    let config = base64::Config::new(base64::CharacterSet::UrlSafe, false);
    base64::encode_config(input, config)
}

/// Decode a base64 input with the UrlSafe character set and no padding.
fn decode_base64<T: AsRef<[u8]>>(input: T) -> Result<Vec<u8>, DecodeError> {
    let config = base64::Config::new(base64::CharacterSet::UrlSafe, false);
    base64::decode_config(input, config)
}
