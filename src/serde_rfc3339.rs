//This code is copied from https://github.com/time-rs/time/tree/serde-rfc3339 (non-perma-link)
//https://github.com/time-rs/time/blob/7dcd89ef6b0f8ee4bbe794a72c80c76639193102/src/serde/rfc3339.rs
//Once https://github.com/time-rs/time/issues/387 is closed this file can be deleted.

use serde::de::Error as _;
use serde::ser::Error as _;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use time::format_description::well_known::Rfc3339;
use time::OffsetDateTime;

/// Serialize an [`OffsetDateTime`] using the well-known RFC3339 format.
pub(crate) fn serialize<S: Serializer>(
    datetime: &OffsetDateTime,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    datetime
        .format(&Rfc3339)
        .map_err(S::Error::custom)?
        .serialize(serializer)
}

/// Deserialize an [`OffsetDateTime`] from its RFC3339 representation.
pub(crate) fn deserialize<'a, D: Deserializer<'a>>(
    deserializer: D,
) -> Result<OffsetDateTime, D::Error> {
    OffsetDateTime::parse(<_>::deserialize(deserializer)?, &Rfc3339).map_err(D::Error::custom)
}

#[cfg(test)]
mod test {
    use serde::{Deserialize, Serialize};
    use time::OffsetDateTime;

    #[test]
    fn can_deserialize_rfc3339() {
        #[derive(Deserialize, Serialize, Debug, PartialEq, Eq)]
        struct Foobar {
            #[serde(with = "crate::serde_rfc3339")]
            date: OffsetDateTime,
        }

        let ts = Foobar {
            date: OffsetDateTime::from_unix_timestamp(1638280964).unwrap(),
        };
        let json_string = serde_json::to_string(&ts).unwrap();

        assert_eq!(r#"{"date":"2021-11-30T14:02:44Z"}"#, &json_string);
    }
}
