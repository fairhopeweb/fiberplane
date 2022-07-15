use fp_bindgen::prelude::Serializable;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize, Serializable)]
#[fp(
    rust_plugin_module = "fiberplane::protocols::providers",
    rust_wasmer_runtime_module = "fiberplane::protocols::providers"
)]
#[serde(rename_all = "camelCase")]
pub struct ProviderResponse {
    /// base64-encoded representation of the raw data.
    pub data: String,

    /// MIME type to use for interpreting the raw data.
    ///
    /// We keep track of this, so that we can elide unnecessary calls to
    /// `extract_data()`, and are able to perform migrations on data specified
    /// in any of the `application/vnd.fiberplane.*` types. For other types of
    /// data, providers are responsible for migrations, and they are able to
    /// include version numbers in their MIME type strings, if desired.
    pub mime_type: String,
}
