//! This module provides a OffsetDateTime parser using the RFC 3339 format. It
//! is intended to be used with clap, but it should be possible to use this in
//! other places as well.
//!
//! ```
//! use time_util::clap_rfc3339;
//! #[derive(clap::Parser)]
//! pub struct AddArgs {
//!   #[clap(long, parse(try_from_str = clap_rfc3339::parse_rfc3339))]
//!   date: Option<time::OffsetDateTime>,
//! }
//! ```

use anyhow::anyhow;
use time::format_description::well_known::Rfc3339;
use time::OffsetDateTime;

pub fn parse_rfc3339(src: &str) -> anyhow::Result<OffsetDateTime> {
    OffsetDateTime::parse(src, &Rfc3339).map_err(|_| anyhow!("invalid rfc3339 input"))
}

// We only test if the result is valid or not, we do not do any actual parsing
// in this file.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn invalid_input() {
        assert!(parse_rfc3339("invalid input").is_err());
    }

    #[test]
    fn valid_input() {
        assert!(parse_rfc3339("2022-01-10T11:15:16+00:00").is_ok());
    }
}
