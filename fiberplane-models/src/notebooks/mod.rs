use crate::comments::UserSummary;
use crate::data_sources::SelectedDataSources;
pub use crate::labels::Label;
use crate::timestamps::*;
use base64uuid::Base64Uuid;
#[cfg(feature = "fp-bindgen")]
use fp_bindgen::prelude::Serializable;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use std::collections::HashMap;
use strum_macros::Display;
use time::OffsetDateTime;
use url::Url;

mod cells;
use crate::names::Name;
pub use cells::*;

pub mod operations;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[cfg_attr(
    feature = "fp-bindgen",
    derive(Serializable),
    fp(
        rust_plugin_module = "fiberplane_models::notebooks",
        rust_wasmer_runtime_module = "fiberplane_models::notebooks"
    )
)]
#[serde(rename_all = "camelCase")]
pub struct Notebook {
    pub id: String,
    pub workspace_id: Base64Uuid,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
    pub time_range: TimeRange,
    pub title: String,
    pub cells: Vec<Cell>,
    pub revision: u32,
    pub visibility: NotebookVisibility,
    pub read_only: bool,
    pub created_by: CreatedBy,

    #[serde(default)]
    pub selected_data_sources: SelectedDataSources,

    #[serde(default)]
    pub labels: Vec<Label>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[cfg_attr(
    feature = "fp-bindgen",
    derive(Serializable),
    fp(
        rust_plugin_module = "fiberplane_models::notebooks",
        rust_wasmer_runtime_module = "fiberplane_models::notebooks"
    )
)]
#[serde(rename_all = "camelCase")]
pub struct NewNotebook {
    pub title: String,
    pub cells: Vec<Cell>,
    pub time_range: NewTimeRange,

    #[serde(default)]
    pub selected_data_sources: SelectedDataSources,

    #[serde(default)]
    pub labels: Vec<Label>,
}

impl From<Notebook> for NewNotebook {
    fn from(notebook: Notebook) -> Self {
        NewNotebook {
            title: notebook.title,
            cells: notebook.cells,
            time_range: notebook.time_range.into(),
            selected_data_sources: notebook.selected_data_sources,
            labels: notebook.labels,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
#[cfg_attr(
    feature = "fp-bindgen",
    derive(Serializable),
    fp(
        rust_plugin_module = "fiberplane_models::notebooks",
        rust_wasmer_runtime_module = "fiberplane_models::notebooks"
    )
)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum CreatedBy {
    User(UserSummary),
    Trigger(TriggerSummary),
    Onboarding,
    Unknown,
}

impl CreatedBy {
    pub fn name(&self) -> String {
        match self {
            CreatedBy::User(user) => user.name.clone(),
            CreatedBy::Trigger(trigger) => trigger.title.clone(),
            CreatedBy::Onboarding => "Onboarding".to_string(),
            CreatedBy::Unknown => String::from("Unknown"),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[cfg_attr(
    feature = "fp-bindgen",
    derive(Serializable),
    fp(
        rust_plugin_module = "fiberplane_models::notebooks",
        rust_wasmer_runtime_module = "fiberplane_models::notebooks"
    )
)]
#[serde(rename_all = "camelCase")]
pub struct TriggerSummary {
    pub id: Base64Uuid,
    pub title: String,
    pub template_id: Base64Uuid,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize, Display)]
#[cfg_attr(
    feature = "fp-bindgen",
    derive(Serializable),
    fp(
        rust_plugin_module = "fiberplane_models::notebooks",
        rust_wasmer_runtime_module = "fiberplane_models::notebooks"
    )
)]
#[serde(rename_all = "snake_case")]
pub enum NotebookVisibility {
    Private,
    Public,
}

impl Default for NotebookVisibility {
    fn default() -> Self {
        Self::Private
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[cfg_attr(
    feature = "fp-bindgen",
    derive(Serializable),
    fp(
        rust_plugin_module = "fiberplane_models::notebooks",
        rust_wasmer_runtime_module = "fiberplane_models::notebooks"
    )
)]
#[serde(rename_all = "camelCase")]
pub struct NotebookPatch {
    pub visibility: NotebookVisibility,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[cfg_attr(
    feature = "fp-bindgen",
    derive(Serializable),
    fp(
        rust_plugin_module = "fiberplane_models::notebooks",
        rust_wasmer_runtime_module = "fiberplane_models::notebooks"
    )
)]
#[serde(rename_all = "camelCase")]
pub struct NotebookCopyDestination {
    pub title: String,
    pub workspace_id: Base64Uuid,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[cfg_attr(
    feature = "fp-bindgen",
    derive(Serializable),
    fp(
        rust_plugin_module = "fiberplane_models::notebooks",
        rust_wasmer_runtime_module = "fiberplane_models::notebooks"
    )
)]
#[serde(rename_all = "camelCase")]
pub struct NewPinnedNotebook {
    /// The ID of the notebook that is being pinned.
    pub notebook_id: Base64Uuid,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Trigger {
    pub id: Base64Uuid,
    pub title: String,
    pub template_id: Base64Uuid,
    pub secret_key: Option<String>,
    pub default_arguments: Option<Map<String, Value>>,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    pub updated_at: OffsetDateTime,
}

pub type TemplateExpandPayload = Map<String, Value>;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TriggerInvokeResponse {
    pub notebook_title: String,
    pub notebook_id: Base64Uuid,
    pub notebook_url: Url,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[cfg_attr(
    feature = "fp-bindgen",
    derive(Serializable),
    fp(
        rust_plugin_module = "fiberplane_models::notebooks",
        rust_wasmer_runtime_module = "fiberplane_models::notebooks"
    )
)]
#[serde(rename_all = "camelCase")]
pub struct NotebookSummary {
    pub id: Base64Uuid,
    pub workspace_id: Base64Uuid,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    pub updated_at: OffsetDateTime,
    pub title: String,
    pub visibility: NotebookVisibility,
    pub created_by: CreatedBy,
    pub labels: Vec<Label>,
}

/// Notebook search parameters
#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize, Serialize)]
#[cfg_attr(
    feature = "fp-bindgen",
    derive(Serializable),
    fp(
        rust_plugin_module = "fiberplane_models::notebooks",
        rust_wasmer_runtime_module = "fiberplane_models::notebooks"
    )
)]
#[serde(rename_all = "camelCase")]
pub struct NotebookSearch {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labels: Option<HashMap<String, Option<String>>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub view: Option<Name>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[cfg_attr(
    feature = "fp-bindgen",
    derive(Serializable),
    fp(
        rust_plugin_module = "fiberplane_models::notebooks",
        rust_wasmer_runtime_module = "fiberplane_models::notebooks"
    )
)]
#[serde(rename_all = "camelCase")]
pub struct TemplateSummary {
    pub id: Base64Uuid,
    pub name: Name,
    pub description: String,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    pub updated_at: OffsetDateTime,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[cfg_attr(
    feature = "fp-bindgen",
    derive(Serializable),
    fp(
        rust_plugin_module = "fiberplane_models::notebooks",
        rust_wasmer_runtime_module = "fiberplane_models::notebooks"
    )
)]
#[serde(rename_all = "camelCase")]
pub struct NewTemplate {
    pub name: Name,
    pub description: String,
    pub body: String,
}

impl NewTemplate {
    pub fn new(name: Name, description: impl Into<String>, body: impl Into<String>) -> Self {
        Self {
            name,
            description: description.into(),
            body: body.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[cfg_attr(
    feature = "fp-bindgen",
    derive(Serializable),
    fp(
        rust_plugin_module = "fiberplane_models::notebooks",
        rust_wasmer_runtime_module = "fiberplane_models::notebooks"
    )
)]
#[serde(rename_all = "camelCase")]
pub struct UpdateTemplate {
    pub description: Option<String>,
    pub body: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NewTrigger {
    pub title: String,
    pub template_name: Name,
    pub default_arguments: Option<Map<String, Value>>,
}
