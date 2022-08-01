use std::convert::TryFrom;

use base64::DecodeError;
use fp_bindgen::prelude::Serializable;
use serde::{Deserialize, Serialize};
use serde_bytes::ByteBuf;

/// Binary blob for passing data in arbitrary encodings.
///
/// Binary blobs are both consumed and produced by providers. Note that for many
/// use-cases, we use agreed on MIME types as defined in
/// [RFC 47](https://www.notion.so/fiberplane/RFC-47-Data-Model-for-Providers-2-0-0b5b1716dbc8450f882d33effb388c5b).
/// Providers are able to use custom MIME types if they desire.
///
/// We can also store blobs in cells, but for this we use [EncodedBlob] to allow
/// JSON serialization.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize, Serializable)]
#[fp(
    rust_plugin_module = "fiberplane::protocols::blobs",
    rust_wasmer_runtime_module = "fiberplane::protocols::blobs"
)]
#[serde(rename_all = "camelCase")]
pub struct Blob {
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

impl TryFrom<EncodedBlob> for Blob {
    type Error = DecodeError;

    fn try_from(blob: EncodedBlob) -> Result<Self, Self::Error> {
        Ok(Self {
            data: ByteBuf::from(base64::decode(&blob.data)?),
            mime_type: blob.mime_type,
        })
    }
}

/// base64-encoded version of [Blob].
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize, Serializable)]
#[fp(
    rust_plugin_module = "fiberplane::protocols::blobs",
    rust_wasmer_runtime_module = "fiberplane::protocols::blobs"
)]
#[serde(rename_all = "camelCase")]
pub struct EncodedBlob {
    /// Raw data, encoded using base64 so it can be serialized using JSON.
    pub data: String,

    /// MIME type to use for interpreting the raw data.
    ///
    /// See [Blob::mime_type].
    pub mime_type: String,
}

impl From<Blob> for EncodedBlob {
    fn from(blob: Blob) -> Self {
        Self {
            data: base64::encode(blob.data.as_ref()),
            mime_type: blob.mime_type,
        }
    }
}
