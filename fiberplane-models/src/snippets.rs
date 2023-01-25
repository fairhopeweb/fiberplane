use crate::names::Name;
use crate::timestamps::Timestamp;
use base64uuid::Base64Uuid;
#[cfg(feature = "fp-bindgen")]
use fp_bindgen::prelude::Serializable;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
#[cfg_attr(
    feature = "fp-bindgen",
    derive(Serializable),
    fp(rust_module = "fiberplane_models::snippets")
)]
#[serde(rename_all = "camelCase")]
pub struct Snippet {
    pub id: Base64Uuid,
    pub name: Name,
    pub description: String,
    pub body: String,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
#[cfg_attr(
    feature = "fp-bindgen",
    derive(Serializable),
    fp(rust_module = "fiberplane_models::snippets")
)]
#[serde(rename_all = "camelCase")]
pub struct SnippetSummary {
    pub id: Base64Uuid,
    pub name: Name,
    pub description: String,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[cfg_attr(
    feature = "fp-bindgen",
    derive(Serializable),
    fp(rust_module = "fiberplane_models::snippets")
)]
#[serde(rename_all = "camelCase")]
pub struct NewSnippet {
    pub name: Name,
    #[serde(default)]
    pub description: String,
    pub body: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[cfg_attr(
    feature = "fp-bindgen",
    derive(Serializable),
    fp(rust_module = "fiberplane_models::snippets")
)]
#[serde(rename_all = "camelCase")]
pub struct UpdateSnippet {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
}
