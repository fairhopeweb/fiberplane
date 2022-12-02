use super::{Error, FORM_ENCODED_MIME_TYPE};
use crate::blobs::Blob;
#[cfg(feature = "fp-bindgen")]
use fp_bindgen::prelude::Serializable;
use serde::{Deserialize, Serialize};

/// A request for a provider to provide auto-suggestions.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[cfg_attr(
    feature = "fp-bindgen",
    derive(Serializable),
    fp(rust_plugin_module = "fiberplane_models::providers")
)]
pub struct AutoSuggestRequest {
    /// The query being typed by the user, up to the focus offset.
    pub query: String,

    /// The query type of the provider we're requesting suggestions for.
    pub query_type: String,

    /// The field in the query form we're requesting suggestions for.
    pub field: String,
}

impl AutoSuggestRequest {
    pub fn from_query_data(blob: &Blob) -> Result<Self, Error> {
        if blob.mime_type != FORM_ENCODED_MIME_TYPE {
            return Err(Error::UnsupportedRequest);
        }

        let mut query = String::new();
        let mut query_type = String::new();
        let mut field = String::new();
        for (key, value) in form_urlencoded::parse(&blob.data) {
            match key.as_ref() {
                "query" => query = value.to_string(),
                "query_type" => query_type = value.to_string(),
                "field" => field = value.to_string(),
                _ => {}
            }
        }

        Ok(AutoSuggestRequest {
            query,
            query_type,
            field,
        })
    }
}

/// A suggestion for a provider's auto-suggest functionality.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[cfg_attr(
    feature = "fp-bindgen",
    derive(Serializable),
    fp(rust_plugin_module = "fiberplane_models::providers")
)]
#[serde(rename_all = "camelCase")]
pub struct Suggestion {
    pub text: String,
    pub description: Option<String>,
}
