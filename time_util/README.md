# time-util

This crate contains some time related functions.

## serde_rfc3339

Serialize or deserialize `time::OffsetDateTime` using the RFC3339 format. To
use this, override the serialize/deserializer used by serde:

```rust
#[derive(serde::Deserialize, serde::Serialize)]
struct Foobar {
    #[serde(with = "time_util::serde_rfc3339")]
    date: time::OffsetDateTime,
}
```
