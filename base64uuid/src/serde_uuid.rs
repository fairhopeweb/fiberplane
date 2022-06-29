use crate::{encode_base64, Base64Uuid};
use serde::de::{Error, Unexpected, Visitor};
use serde::{Deserializer, Serializer};
use std::fmt;
use uuid::Uuid;

/// Serializes [Uuid](uuid:Uuid) as [Base64Uuid](crate::Base64Uuid) without the need of manually converting.
/// For usage with Serde's `#[serde(serialize_with = "")]` field attribute.
///
/// # Example
///
/// ```
/// # use uuid::Uuid;
/// # use serde::Serialize;
/// # use base64uuid::Base64Uuid;
/// # use serde_json::Result;
///
/// #[derive(Serialize, Debug)]
/// struct Notebook {
///     #[serde(with = "base64uuid::serde_uuid")] // or #[serde(serialize_with = "base64uuid::serde_uuid::serialize")]
///     id: Uuid
/// }
///
/// fn main() -> Result<()> {
///     let notebook = Notebook {
///         id: Uuid::new_v4()
///     };
///
///     let serialized = serde_json::to_string(&notebook)?;
///     assert_eq!(serialized, format!("{{\"id\":\"{}\"}}", Base64Uuid::from(notebook.id)));
///     # Ok(())
/// }
/// ```
pub fn serialize<S: Serializer>(uuid: &Uuid, serializer: S) -> Result<S::Ok, S::Error> {
    serializer.serialize_str(&encode_base64(uuid.as_bytes()))
}

/// Deserializes [Base64Uuid](crate::Base64Uuid) as [Uuid](uuid:Uuid) without the need of manually converting.
/// For usage with Serde's `#[serde(deserialize_with = "")]` field attribute.
///
/// # Example
///
/// ```
/// # use uuid::Uuid;
/// # use serde::Deserialize;
/// # use serde_json::Result;
///
/// #[derive(Deserialize, Debug)]
/// struct Notebook {
///     #[serde(with = "base64uuid::serde_uuid")] // or #[serde(deserialize_with = "base64uuid::serde_uuid::deserialize")]
///     id: Uuid
/// }
///
/// fn main() -> Result<()> {
///     let input = r#"{"id":"cQQzH_mJQU2aqG_fHfMNiA"}"#;
///
///     let notebook: Notebook = serde_json::from_str(input)?;
///     assert_eq!("7104331f-f989-414d-9aa8-6fdf1df30d88", notebook.id.to_string());
///     # Ok(())
/// }
/// ```
pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Uuid, D::Error> {
    deserializer.deserialize_str(UuidVisitor)
}

struct UuidVisitor;

impl<'de> Visitor<'de> for UuidVisitor {
    type Value = Uuid;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "Base64 UUID")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(Base64Uuid::parse_str(v)
            .map_err(|_| Error::invalid_value(Unexpected::Str(v), &self))?
            .into_uuid())
    }
}

pub mod option {
    use serde::de::Visitor;
    use serde::{Deserializer, Serializer};
    use std::fmt;
    use uuid::Uuid;

    /// Serializes [Uuid](uuid:Uuid) as [Base64Uuid](crate::Base64Uuid) without the need of manually converting.
    /// For usage with Serde's `#[serde(serialize_with = "")]` field attribute.
    ///
    /// # Example
    ///
    /// ```
    /// # use uuid::Uuid;
    /// # use serde::Serialize;
    /// # use base64uuid::Base64Uuid;
    /// # use serde_json::Result;
    ///
    /// #[derive(Serialize, Debug)]
    /// struct Notebook {
    ///     #[serde(with = "base64uuid::serde_uuid::option")] // or #[serde(serialize_with = "base64uuid::serde_uuid::option::serialize")]
    ///     id: Option<Uuid>
    /// }
    ///
    /// fn main() -> Result<()> {
    ///     let notebook = Notebook {
    ///         id: None
    ///     };
    ///
    ///     let serialized = serde_json::to_string(&notebook)?;
    ///     assert_eq!(serialized, "{\"id\":null}");
    ///     # Ok(())
    /// }
    /// ```
    pub fn serialize<S: Serializer>(
        option: &Option<Uuid>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        match option {
            Some(ref value) => super::serialize(value, serializer),
            None => serializer.serialize_none(),
        }
    }

    /// Deserializes an optional [Base64Uuid](crate::Base64Uuid) as [Option](std::Option) of [Uuid](uuid:Uuid) without the need of manually converting.
    /// For usage with Serde's `#[serde(deserialize_with = "")]` field attribute.
    ///
    /// # Example
    ///
    /// ```
    /// # use uuid::Uuid;
    /// # use serde::Deserialize;
    /// # use serde_json::Result;
    ///
    /// #[derive(Deserialize, Debug)]
    /// struct Notebook {
    ///     #[serde(with = "base64uuid::serde_uuid::option")] // or #[serde(deserialize_with = "base64uuid::serde_uuid::option::deserialize")]
    ///     id: Option<Uuid>
    /// }
    ///
    /// fn main() -> Result<()> {
    ///     let input = r#"{"id":"cQQzH_mJQU2aqG_fHfMNiA"}"#;
    ///
    ///     let notebook: Notebook = serde_json::from_str(input)?;
    ///     assert_eq!("7104331f-f989-414d-9aa8-6fdf1df30d88", notebook.id.expect("id should be set").to_string());
    ///     # Ok(())
    /// }
    /// ```
    pub fn deserialize<'a, D: Deserializer<'a>>(deserializer: D) -> Result<Option<Uuid>, D::Error> {
        deserializer.deserialize_option(UuidOptionVisitor)
    }

    struct UuidOptionVisitor;

    impl<'de> Visitor<'de> for UuidOptionVisitor {
        type Value = Option<Uuid>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            write!(formatter, "Optional Base64 UUID")
        }

        fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
        where
            D: Deserializer<'de>,
        {
            deserializer
                .deserialize_str(super::UuidVisitor)
                .map(Option::Some)
        }

        fn visit_none<E>(self) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(None)
        }
    }

    #[cfg(test)]
    mod tests {
        use serde::{Deserialize, Serialize};
        use uuid::Uuid;

        #[test]
        fn deserialize_test() {
            // The struct that will be used to serialize into.
            #[derive(Serialize, Deserialize)]
            struct Notebook {
                #[serde(default, with = "crate::serde_uuid::option")]
                id: Option<Uuid>,
            }

            struct TestCase {
                input: &'static str,
                expected_id: Option<Uuid>,
            }

            let tests = vec![
                TestCase {
                    input: r#"{"id":"cQQzH_mJQU2aqG_fHfMNiA"}"#,
                    expected_id: Some(
                        Uuid::parse_str("7104331f-f989-414d-9aa8-6fdf1df30d88").unwrap(),
                    ),
                },
                TestCase {
                    input: r#"{"id":null}"#,
                    expected_id: None,
                },
                TestCase {
                    input: r#"{}"#,
                    expected_id: None,
                },
            ];

            for test in tests {
                let sample_struct: Notebook =
                    serde_json::from_str(&test.input).expect("unable to deserialize");
                assert_eq!(sample_struct.id, test.expected_id);
            }
        }
    }
}
