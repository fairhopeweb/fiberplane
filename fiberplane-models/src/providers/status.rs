use super::Error;
#[cfg(feature = "fp-bindgen")]
use fp_bindgen::prelude::Serializable;
use serde::{Deserialize, Serialize};

/// Response type for status requests.
///
/// To be serialized using the `application/vnd.fiberplane.provider-status`
/// MIME type.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[cfg_attr(
    feature = "fp-bindgen",
    derive(Serializable),
    fp(rust_plugin_module = "fiberplane_models::providers")
)]
#[serde(rename_all = "camelCase")]
pub struct ProviderStatus {
    /// Indicates whether the provider is available to be queried.
    pub status: Result<(), Error>,

    /// Version string of the provider.
    ///
    /// Arbitrary strings may be used, such as commit hashes, but release
    /// versions of providers are expected to report semver versions.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub version: String,

    /// Human-readable timestamp at which the provider was built.
    ///
    /// Only used for diagnostics.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub built_at: String,
}
