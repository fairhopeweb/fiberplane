use base64::DecodeError;
use serde::{de, Deserialize, Serialize, Serializer};
use std::borrow::{Borrow, Cow};
use std::convert::TryFrom;
use std::fmt::{self, Debug, Display, Formatter};
use std::ops::Deref;
use std::str::FromStr;
use thiserror::Error;
use uuid::Uuid;

pub mod serde_uuid;
#[cfg(feature = "fp-bindgen")]
mod serializable;
#[cfg(test)]
mod tests;

/// A Base64Uuid is a base64url-encoded UUID that is
/// used for IDs across Fiberplane's products.
/// The main motivation is that it is a slightly more
/// condensed representation of a UUID and it looks
/// better in URLs.
#[derive(PartialEq, Hash, Eq, Clone, Copy)]
// This tells sqlx to save it to the database as a UUID
#[cfg_attr(feature = "sqlx", derive(sqlx::Type), sqlx(transparent))]
#[repr(transparent)]
pub struct Base64Uuid(pub Uuid);

impl Base64Uuid {
    /// Create a new [`Base64Uuid`].
    ///
    /// The Base64Uuid representation will not start with a dash which could
    /// conflict with cli arguments.
    #[cfg(feature = "creation")]
    pub fn new() -> Self {
        Base64Uuid(Self::new_uuid())
    }

    /// Create a new [`Uuid`] using the [`Base64Uuid`] restrictions
    ///
    /// This means that a Uuid generated by this function will never contain a
    /// dash as the first character when converted to a Base64Uuid.
    #[cfg(feature = "creation")]
    pub fn new_uuid() -> Uuid {
        let uuid = Uuid::new_v4();

        // If the uuid starts with `-`, our cli will think it's an argument so just regenerate them in that case
        match uuid.as_bytes().first() {
            Some(248..=251) | None => Self::new_uuid(),
            _ => uuid,
        }
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

impl Serialize for Base64Uuid {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if serializer.is_human_readable() {
            serializer.serialize_str(&self.to_string())
        } else {
            serializer.serialize_bytes(self.0.as_bytes())
        }
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

            fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Uuid::from_slice(v)
                    .map_err(de::Error::custom)
                    .map(Base64Uuid)
            }
        }

        if deserializer.is_human_readable() {
            deserializer.deserialize_str(Base64UuidVisitor)
        } else {
            deserializer.deserialize_bytes(Base64UuidVisitor)
        }
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
    #[cfg(feature = "creation")]
    fn default() -> Self {
        Self::new()
    }

    #[cfg(not(feature = "creation"))]
    fn default() -> Self {
        Self::nil()
    }
}

impl Deref for Base64Uuid {
    type Target = Uuid;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl PartialEq<Uuid> for Base64Uuid {
    fn eq(&self, other: &Uuid) -> bool {
        self.0 == *other
    }
}

impl PartialEq<Base64Uuid> for Uuid {
    fn eq(&self, other: &Base64Uuid) -> bool {
        *self == other.0
    }
}

impl PartialEq<String> for Base64Uuid {
    fn eq(&self, other: &String) -> bool {
        &self.to_string() == other
    }
}

impl PartialEq<Base64Uuid> for String {
    fn eq(&self, other: &Base64Uuid) -> bool {
        other == self
    }
}

impl PartialEq<str> for Base64Uuid {
    fn eq(&self, other: &str) -> bool {
        *self == other
    }
}

impl PartialEq<Base64Uuid> for str {
    fn eq(&self, other: &Base64Uuid) -> bool {
        other == self
    }
}

impl PartialEq<&str> for Base64Uuid {
    fn eq(&self, other: &&str) -> bool {
        match decode_base64(other) {
            Ok(vec) => Uuid::from_slice(&vec).map_or_else(|_| false, |uuid| self.0 == uuid),
            Err(_) => false,
        }
    }
}

impl PartialEq<Base64Uuid> for &str {
    fn eq(&self, other: &Base64Uuid) -> bool {
        other == self
    }
}

impl<'a> PartialEq<Cow<'a, str>> for Base64Uuid {
    fn eq(&self, other: &Cow<'a, str>) -> bool {
        let str: &str = other.borrow();
        *self == str
    }
}

impl<'a> PartialEq<Base64Uuid> for Cow<'a, str> {
    fn eq(&self, other: &Base64Uuid) -> bool {
        other == self
    }
}

#[derive(Debug, Error, PartialEq, Eq, Clone, Copy)]
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
