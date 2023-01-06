use base64uuid::Base64Uuid;
#[cfg(feature = "fp-bindgen")]
use fp_bindgen::prelude::Serializable;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[cfg_attr(
    feature = "fp-bindgen",
    derive(Serializable),
    fp(
        rust_plugin_module = "fiberplane_models::tokens",
        rust_wasmer_runtime_module = "fiberplane_models::tokens"
    )
)]
#[serde(rename_all = "camelCase")]
pub struct TokenSummary {
    pub id: Base64Uuid,
    pub title: String,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
    #[serde(default, with = "time::serde::rfc3339::option")]
    pub expires_at: Option<OffsetDateTime>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[cfg_attr(
    feature = "fp-bindgen",
    derive(Serializable),
    fp(
        rust_plugin_module = "fiberplane_models::tokens",
        rust_wasmer_runtime_module = "fiberplane_models::tokens"
    )
)]
#[serde(rename_all = "camelCase")]
pub struct NewToken {
    pub title: String,
}

impl NewToken {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[cfg_attr(
    feature = "fp-bindgen",
    derive(Serializable),
    fp(
        rust_plugin_module = "fiberplane_models::tokens",
        rust_wasmer_runtime_module = "fiberplane_models::tokens"
    )
)]
#[serde(rename_all = "camelCase")]
pub struct Token {
    pub id: Base64Uuid,
    pub title: String,
    pub token: String,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
    #[serde(default, with = "time::serde::rfc3339::option")]
    pub expires_at: Option<OffsetDateTime>,
}
