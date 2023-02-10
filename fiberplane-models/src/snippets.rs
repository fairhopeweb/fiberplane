use crate::names::Name;
use crate::timestamps::Timestamp;
use base64uuid::Base64Uuid;
#[cfg(feature = "fp-bindgen")]
use fp_bindgen::prelude::Serializable;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, TypedBuilder)]
#[cfg_attr(
    feature = "fp-bindgen",
    derive(Serializable),
    fp(rust_module = "fiberplane_models::snippets")
)]
#[non_exhaustive]
#[serde(rename_all = "camelCase")]
pub struct Snippet {
    pub id: Base64Uuid,
    pub name: Name,
    pub description: String,
    pub body: String,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, TypedBuilder)]
#[cfg_attr(
    feature = "fp-bindgen",
    derive(Serializable),
    fp(rust_module = "fiberplane_models::snippets")
)]
#[non_exhaustive]
#[serde(rename_all = "camelCase")]
pub struct SnippetSummary {
    pub id: Base64Uuid,
    pub name: Name,
    pub description: String,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
}

#[derive(Debug, Deserialize, Serialize, Clone, TypedBuilder)]
#[cfg_attr(
    feature = "fp-bindgen",
    derive(Serializable),
    fp(rust_module = "fiberplane_models::snippets")
)]
#[non_exhaustive]
#[serde(rename_all = "camelCase")]
pub struct NewSnippet {
    pub name: Name,
    #[builder(default, setter(into))]
    #[serde(default)]
    pub description: String,
    #[builder(setter(into))]
    pub body: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, TypedBuilder)]
#[cfg_attr(
    feature = "fp-bindgen",
    derive(Serializable),
    fp(rust_module = "fiberplane_models::snippets")
)]
#[non_exhaustive]
#[serde(rename_all = "camelCase")]
pub struct UpdateSnippet {
    #[builder(default, setter(into))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[builder(default, setter(into))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
}
