use crate::protocols::{
    core::{Cell, Label, NotebookDataSource, TimeRange},
    formatting::Formatting,
};
use fp_bindgen::prelude::Serializable;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Serializable)]
#[fp(rust_plugin_module = "fiberplane::operations")]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Change {
    InsertCell(InsertCellChange),
    DeleteCell(DeleteCellChange),
    MoveCells(MoveCellsChange),
    UpdateCell(UpdateCellChange),
    UpdateCellText(UpdateCellTextChange),
    UpdateNotebookTimeRange(UpdateNotebookTimeRangeChange),
    UpdateNotebookTitle(UpdateNotebookTitleChange),
    AddDataSource(AddDataSourceChange),
    DeleteDataSource(DeleteDataSourceChange),
    UpdateDataSource(UpdateDataSourceChange),
    AddLabel(AddLabelChange),
    ReplaceLabel(ReplaceLabelChange),
    RemoveLabel(RemoveLabelChange),
}

/// Specifies the given cell must be inserted at the given index.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Serializable)]
#[fp(rust_plugin_module = "fiberplane::operations")]
#[serde(rename_all = "camelCase")]
pub struct InsertCellChange {
    pub cell: Cell,
    pub index: u32,
}

/// Specifies the cell with the given ID must be deleted.
#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize, Serializable)]
#[fp(rust_plugin_module = "fiberplane::operations")]
#[serde(rename_all = "camelCase")]
pub struct DeleteCellChange {
    pub cell_id: String,
}

/// Moves the cells with the given IDs to the given index.
#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize, Serializable)]
#[fp(rust_plugin_module = "fiberplane::operations")]
#[serde(rename_all = "camelCase")]
pub struct MoveCellsChange {
    /// One or more IDs of cells to move. If multiple IDs are given, they must be consecutive.
    pub cell_ids: Vec<String>,

    /// The index where the cells will be reinserted.
    ///
    /// This is the index excluding the moved cells themselves. This makes it impossible for the
    /// index to refer to the range of cells being moved.
    pub index: u32,
}

/// Specifies the given cell must be updated.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Serializable)]
#[fp(rust_plugin_module = "fiberplane::operations")]
#[serde(rename_all = "camelCase")]
pub struct UpdateCellChange {
    pub cell: Cell,
}

/// Specifies the text field of the given cell (`content` or `title`, depending on the cell type)
/// must be updated.
///
/// Optionally updates the formatting as well.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Serializable)]
#[fp(rust_plugin_module = "fiberplane::operations")]
#[serde(rename_all = "camelCase")]
pub struct UpdateCellTextChange {
    /// ID of the cell we're updating.
    pub cell_id: String,

    /// Optional cell field that we're updating.
    pub field: Option<String>,

    /// The new text string to store.
    pub text: String,

    /// Optional formatting to store.
    ///
    /// If the formatting is omitted, it means the cell had no formatting and
    /// will continue to not have it. In any other case, formatting will be
    /// provided, even if unchanged.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub formatting: Option<Formatting>,
}

#[deprecated(note = "Please use UpdateNotebookTimeRangeChange instead")]
pub type UpdateNotebookTimeRange = UpdateNotebookTimeRangeChange;

/// Specifies that the time range for a notebook (aka global time) must be updated
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Serializable)]
#[fp(rust_plugin_module = "fiberplane::operations")]
#[serde(rename_all = "camelCase")]
pub struct UpdateNotebookTimeRangeChange {
    pub time_range: TimeRange,
}

/// Specifies the title of the notebook must be updated.
#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize, Serializable)]
#[fp(rust_plugin_module = "fiberplane::operations")]
#[serde(rename_all = "camelCase")]
pub struct UpdateNotebookTitleChange {
    pub title: String,
}

/// Specifies the given data-source must be created with the following name.
#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize, Serializable)]
#[fp(rust_plugin_module = "fiberplane::operations")]
#[serde(rename_all = "camelCase")]
pub struct AddDataSourceChange {
    pub name: String,
    pub data_source: Box<NotebookDataSource>,
}

/// Specifies the data-source with the given name must be deleted.
#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize, Serializable)]
#[fp(rust_plugin_module = "fiberplane::operations")]
#[serde(rename_all = "camelCase")]
pub struct DeleteDataSourceChange {
    pub name: String,
}

/// Specifies the given data-source must be updated.
#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize, Serializable)]
#[fp(rust_plugin_module = "fiberplane::operations")]
#[serde(rename_all = "camelCase")]
pub struct UpdateDataSourceChange {
    pub name: String,
    pub data_source: Box<NotebookDataSource>,
}

/// Specifies the given label must be added.
#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize, Serializable)]
#[fp(rust_plugin_module = "fiberplane::operations")]
#[serde(rename_all = "camelCase")]
pub struct AddLabelChange {
    /// The label that was added.
    pub label: Label,
}

/// Specifies the given label must be updated.
#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize, Serializable)]
#[fp(rust_plugin_module = "fiberplane::operations")]
#[serde(rename_all = "camelCase")]
pub struct ReplaceLabelChange {
    /// The key of the existing label that will be replaced.
    pub key: String,

    /// The new values of the label (Note: it is possible that the key changes).
    pub label: Label,
}

/// Specifies the given label must be removed.
#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize, Serializable)]
#[fp(rust_plugin_module = "fiberplane::operations")]
#[serde(rename_all = "camelCase")]
pub struct RemoveLabelChange {
    /// The label that was removed.
    pub label: Label,
}
