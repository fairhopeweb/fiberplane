use crate::debug_print_bytes;
use base64::DecodeError;
use bytes::Bytes;
#[cfg(feature = "fp-bindgen")]
use fp_bindgen::prelude::Serializable;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::fmt::{self, Debug, Formatter};

/// Binary blob for passing data in arbitrary encodings.
///
/// Binary blobs are both consumed and produced by providers. Note that for many
/// use-cases, we use agreed on MIME types as defined in
/// [RFC 47](https://www.notion.so/fiberplane/RFC-47-Data-Model-for-Providers-2-0-0b5b1716dbc8450f882d33effb388c5b).
/// Providers are able to use custom MIME types if they desire.
///
/// We can also store blobs in cells, but for this we use [EncodedBlob] to allow
/// JSON serialization.
#[derive(Clone, Default, Deserialize, Eq, PartialEq, Serialize)]
#[cfg_attr(
    feature = "fp-bindgen",
    derive(Serializable),
    fp(
        rust_plugin_module = "fiberplane_models::blobs",
        rust_wasmer_runtime_module = "fiberplane_models::blobs"
    )
)]
#[serde(rename_all = "camelCase")]
pub struct Blob {
    /// Raw data.
    pub data: Bytes,

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
            data: base64::decode(&blob.data)?.into(),
            mime_type: blob.mime_type,
        })
    }
}

impl TryFrom<&EncodedBlob> for Blob {
    type Error = DecodeError;

    fn try_from(blob: &EncodedBlob) -> Result<Self, Self::Error> {
        Ok(Self {
            data: base64::decode(&blob.data)?.into(),
            mime_type: blob.mime_type.clone(),
        })
    }
}

impl Debug for Blob {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Blob")
            .field("mime_type", &self.mime_type)
            .field("data_length", &self.data.len())
            .field("data", &debug_print_bytes(&self.data))
            .finish()
    }
}

/// base64-encoded version of [Blob].
#[derive(Clone, Default, Deserialize, Eq, PartialEq, Serialize)]
#[cfg_attr(
    feature = "fp-bindgen",
    derive(Serializable),
    fp(
        rust_plugin_module = "fiberplane_models::blobs",
        rust_wasmer_runtime_module = "fiberplane_models::blobs"
    )
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

impl From<&Blob> for EncodedBlob {
    fn from(blob: &Blob) -> Self {
        Self {
            data: base64::encode(blob.data.as_ref()),
            mime_type: blob.mime_type.clone(),
        }
    }
}

impl Debug for EncodedBlob {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("EncodedBlob")
            .field("mime_type", &self.mime_type)
            .field("data_length", &self.data.len())
            .field("data", &debug_print_bytes(self.data.as_bytes()))
            .finish()
    }
}
