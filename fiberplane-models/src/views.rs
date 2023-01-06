use crate::labels::Label;
use crate::names::Name;
use crate::sorting::{PaginatedSearch, ViewSortFields};
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
        rust_plugin_module = "fiberplane_models::views",
        rust_wasmer_runtime_module = "fiberplane_models::views"
    )
)]
#[serde(rename_all = "camelCase")]
pub struct View {
    pub id: Base64Uuid,
    pub name: Name,
    pub display_name: String,
    pub description: String,
    pub labels: Vec<Label>,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    pub updated_at: OffsetDateTime,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ViewQuery {
    #[serde(flatten)]
    pub search: PaginatedSearch<ViewSortFields>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[cfg_attr(
    feature = "fp-bindgen",
    derive(Serializable),
    fp(
        rust_plugin_module = "fiberplane_models::views",
        rust_wasmer_runtime_module = "fiberplane_models::views"
    )
)]
#[serde(rename_all = "camelCase")]
pub struct NewView {
    pub name: Name,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    pub description: String,
    pub labels: Vec<Label>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[cfg_attr(
    feature = "fp-bindgen",
    derive(Serializable),
    fp(
        rust_plugin_module = "fiberplane_models::views",
        rust_wasmer_runtime_module = "fiberplane_models::views"
    )
)]
#[serde(rename_all = "camelCase")]
pub struct UpdateView {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<Label>>,
}
