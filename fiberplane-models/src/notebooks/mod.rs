use crate::comments::UserSummary;
use crate::data_sources::SelectedDataSources;
pub use crate::labels::Label;
use crate::timestamps::*;
use base64uuid::Base64Uuid;
#[cfg(feature = "fp-bindgen")]
use fp_bindgen::prelude::Serializable;
use serde::{Deserialize, Serialize};

mod cells;
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
    Unknown,
}

impl CreatedBy {
    pub fn name(&self) -> String {
        match self {
            CreatedBy::User(user) => user.name.clone(),
            CreatedBy::Trigger(trigger) => trigger.title.clone(),
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
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
