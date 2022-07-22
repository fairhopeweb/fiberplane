use fp_bindgen::prelude::Serializable;
use serde::{Deserialize, Serialize};
use serde_bytes::ByteBuf;

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize, Serializable)]
#[fp(
    rust_plugin_module = "fiberplane::protocols::providers",
    rust_wasmer_runtime_module = "fiberplane::protocols::providers"
)]
#[serde(rename_all = "camelCase")]
pub struct ProviderResponse {
    /// Raw data.
    pub data: ByteBuf,

    /// MIME type to use for interpreting the raw data.
    ///
    /// We keep track of this, so that we can elide unnecessary calls to
    /// `extract_data()`, and are able to perform migrations on data specified
    /// in any of the `application/vnd.fiberplane.*` types. For other types of
    /// data, providers are responsible for migrations, and they are able to
    /// include version numbers in their MIME type strings, if desired.
    pub mime_type: String,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize, Serializable)]
#[fp(
    rust_plugin_module = "fiberplane::protocols::providers",
    rust_wasmer_runtime_module = "fiberplane::protocols::providers"
)]
#[serde(rename_all = "camelCase")]
pub struct StoredProviderResponse {
    /// Raw data, encoded using base64 so it can be serialized using JSON.
    pub data: String,

    /// MIME type to use for interpreting the raw data.
    ///
    /// See [ProviderResponse::mime_type].
    pub mime_type: String,
}
