use crate::protocols::core::{Cell, NotebookDataSource, TimeRange};
use fp_bindgen::prelude::Serializable;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Serializable)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Change {
    InsertCell(InsertCellChange),
    DeleteCell(DeleteCellChange),
    MoveCells(MoveCellsChange),
    UpdateCell(UpdateCellChange),
    UpdateNotebookTimeRange(UpdateNotebookTimeRangeChange),
    UpdateNotebookTitle(UpdateNotebookTitleChange),
    AddDataSource(AddDataSourceChange),
    DeleteDataSource(DeleteDataSourceChange),
    UpdateDataSource(UpdateDataSourceChange),
}

/// Specifies the given cell must be inserted at the given index.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Serializable)]
#[serde(rename_all = "camelCase")]
pub struct InsertCellChange {
    pub cell: Cell,
    pub index: u32,
}

/// Specifies the cell with the given ID must be deleted.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Serializable)]
#[serde(rename_all = "camelCase")]
pub struct DeleteCellChange {
    pub cell_id: String,
}

/// Moves the cells with the given IDs to the given index.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Serializable)]
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
#[serde(rename_all = "camelCase")]
pub struct UpdateCellChange {
    pub cell: Cell,
}

#[deprecated(note = "Please use UpdateNotebookTimeRangeChange instead")]
pub type UpdateNotebookTimeRange = UpdateNotebookTimeRangeChange;

/// Specifies that the time range for a notebook (aka global time) must be updated
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Serializable)]
#[serde(rename_all = "camelCase")]
pub struct UpdateNotebookTimeRangeChange {
    pub time_range: TimeRange,
}

/// Specifies the title of the notebook must be updated.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Serializable)]
#[serde(rename_all = "camelCase")]
pub struct UpdateNotebookTitleChange {
    pub title: String,
}

/// Specifies the given data-source must be created with the following name.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Serializable)]
#[serde(rename_all = "camelCase")]
pub struct AddDataSourceChange {
    pub name: String,
    pub data_source: Box<NotebookDataSource>,
}

/// Specifies the data-source with the given name must be deleted.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Serializable)]
#[serde(rename_all = "camelCase")]
pub struct DeleteDataSourceChange {
    pub name: String,
}

/// Specifies the given data-source must be updated.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Serializable)]
#[serde(rename_all = "camelCase")]
pub struct UpdateDataSourceChange {
    pub name: String,
    pub data_source: Box<NotebookDataSource>,
}
