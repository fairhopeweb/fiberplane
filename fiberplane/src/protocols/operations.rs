use crate::protocols::core::{Cell, NotebookDataSource, TimeRange};
use fp_bindgen::prelude::*;
use serde::{Deserialize, Serialize};

use super::core::Label;

/// An operation is the representation for a mutation to be performed to a notebook.
///
/// Operations are intended to be atomic (they should either be performed in their entirety or not
/// at all), while also capturing the intent of the user.
///
/// For more information, please see RFC 8:
///   https://www.notion.so/fiberplane/RFC-8-Notebook-Operations-f9d18676d0d9437d81de30faa219deb4
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Serializable)]
#[fp(rust_plugin_module = "fiberplane::protocols::operations")]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Operation {
    AddCells(AddCellsOperation),
    MergeCells(MergeCellsOperation),
    MoveCells(MoveCellsOperation),
    RemoveCells(RemoveCellsOperation),
    ReplaceText(ReplaceTextOperation),
    SplitCell(SplitCellOperation),
    UpdateCell(UpdateCellOperation),
    UpdateNotebookTimeRange(UpdateNotebookTimeRangeOperation),
    UpdateNotebookTitle(UpdateNotebookTitleOperation),
    AddDataSource(AddDataSourceOperation),
    UpdateDataSource(UpdateDataSourceOperation),
    RemoveDataSource(RemoveDataSourceOperation),
    AddLabel(AddLabelOperation),
    ReplaceLabel(ReplaceLabelOperation),
    RemoveLabel(RemoveLabelOperation),
}

/// Adds one or more cells at the given position.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Serializable)]
#[fp(rust_plugin_module = "fiberplane::protocols::operations")]
#[serde(rename_all = "camelCase")]
pub struct AddCellsOperation {
    /// The new cells, including their index after adding.
    pub cells: Vec<CellWithIndex>,

    /// Optional, existing cells to which references to the newly added cells have been added.
    ///
    /// This is not something that currently happens from the UI, but is useful to atomically
    /// revert a `RemoveCellsOperation`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub referencing_cells: Option<Vec<CellWithIndex>>,
}

/// Merges the cell immediately after the target cell into it by appending its content.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Serializable)]
#[fp(rust_plugin_module = "fiberplane::protocols::operations")]
#[serde(rename_all = "camelCase")]
pub struct MergeCellsOperation {
    /// Optional text we want to "glue" between the content of the target cell and the source cell.
    /// This is useful if we want to revert a `SplitCellOperation` that contained selected text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub glue_text: Option<String>,

    /// Source cell that will be merged into the target cell. This should be the cell immediately
    /// after the target cell.
    pub source_cell: Cell,

    /// The length of the text content of the target cell right before the merge. This is the index
    /// at which we will want to split the cell if we need to revert the merge.
    ///
    /// Please be aware this length refers to the number of Unicode Scalar Values (non-surrogate
    /// codepoints) in the cell content, which may require additional effort to determine correctly.
    pub target_content_length: u32,

    /// ID of the target cell into which the merge will be performed.
    pub target_cell_id: String,

    /// Optional cells that referenced the source cell and which are affected by the removal of it.
    ///
    /// If a referencing cell *only* references the source cell, it may be removed.
    /// Otherwise, the source cell may simply be unreferenced and the referencing cell will be
    /// retained.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub referencing_cells: Option<Vec<CellWithIndex>>,
}

/// Moves one or more cells.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Serializable)]
#[fp(rust_plugin_module = "fiberplane::protocols::operations")]
#[serde(rename_all = "camelCase")]
pub struct MoveCellsOperation {
    /// IDs of all the cells to be moved.
    ///
    /// These must be adjacent and given in the order they appear in the notebook.
    pub cell_ids: Vec<String>,

    /// Index the cells will be moved from. This is the index of the first cell before the move.
    pub from_index: u32,

    /// Index the cells will be moved to. This is the index of the first cell after the move.
    pub to_index: u32,
}

/// Removes one or more cells.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Serializable)]
#[fp(rust_plugin_module = "fiberplane::protocols::operations")]
#[serde(rename_all = "camelCase")]
pub struct RemoveCellsOperation {
    /// The removed cells, including their index before the removal.
    pub removed_cells: Vec<CellWithIndex>,

    /// Optional cells that referenced the removed cells and which are affected by the removal.
    ///
    /// If a referencing cell *only* references the removed cells, it may be cascade removed.
    /// Otherwise, the removed cells may simply be unreferenced and the referencing cell will be
    /// retained.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub referencing_cells: Option<Vec<CellWithIndex>>,
}

/// Replaces the part of the content in any content type cell or the title of a graph cell.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Serializable)]
#[fp(rust_plugin_module = "fiberplane::protocols::operations")]
#[serde(rename_all = "camelCase")]
pub struct ReplaceTextOperation {
    /// ID of the cell whose text we're modifying.
    pub cell_id: String,

    /// Starting offset where we will be replacing the text.
    ///
    /// Please be aware this offset refers to the position of a Unicode Scalar Value (non-surrogate
    /// codepoint) in the cell text, which may require additional effort to determine correctly.
    pub offset: u32,

    /// The new text value we're inserting.
    pub new_text: String,

    /// The old text that we're replacing.
    pub old_text: String,
}

/// Splits a cell at the given position.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Serializable)]
#[fp(rust_plugin_module = "fiberplane::protocols::operations")]
#[serde(rename_all = "camelCase")]
pub struct SplitCellOperation {
    /// ID of the cell that will be split.
    pub cell_id: String,

    /// The character index inside the cell to split at.
    ///
    /// Please be aware this index refers to the position of a Unicode Scalar Value (non-surrogate
    /// codepoint) in the cell content, which may require additional effort to determine correctly.
    pub split_index: u32,

    /// If any text was selected at the moment of splitting, that selection is removed; only the
    /// part before the selection is retained in the split cell, while only the part after the
    /// selection ends up in the new cell.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub removed_text: Option<String>,

    /// Newly created cell after the split.
    pub new_cell: Cell,

    /// Optional cells to which a reference to the newly added cell should be added. These may be
    /// cells that will need to be newly inserted.
    ///
    /// This is not something that currently happens from the UI, but is useful to atomically
    /// revert a `MergeCellsOperation`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub referencing_cells: Option<Vec<CellWithIndex>>,
}

/// Updates arbitrary properties of a cell.
///
/// **FIXME:** Because this operation is so coarse, it currently breaks assumptions about intent and
///            convergence.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Serializable)]
#[fp(rust_plugin_module = "fiberplane::protocols::operations")]
#[serde(rename_all = "camelCase")]
pub struct UpdateCellOperation {
    /// The old cell with its content, so that we can revert the update if necessary.
    ///
    /// Note this cell may even have a different ID, in which case the old cell is swapped out for
    /// the updated one while maintaining its position.
    pub old_cell: Box<Cell>,

    /// The newly updated cell.
    pub updated_cell: Box<Cell>,
}

/// Updates the notebook time range.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Serializable)]
#[fp(rust_plugin_module = "fiberplane::protocols::operations")]
#[serde(rename_all = "camelCase")]
pub struct UpdateNotebookTimeRangeOperation {
    pub old_time_range: TimeRange,
    pub time_range: TimeRange,
}

/// Updates the notebook title.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Serializable)]
#[fp(rust_plugin_module = "fiberplane::protocols::operations")]
#[serde(rename_all = "camelCase")]
pub struct UpdateNotebookTitleOperation {
    pub old_title: String,
    pub title: String,
}

/// Adds an data-source to an notebook.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Serializable)]
#[fp(rust_plugin_module = "fiberplane::protocols::operations")]
#[serde(rename_all = "camelCase")]
pub struct AddDataSourceOperation {
    /// The identifier of this data-source within the notebook
    pub name: String,

    /// The new data-source
    pub data_source: Box<NotebookDataSource>,
}

/// Updates an data-source in an notebook.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Serializable)]
#[fp(rust_plugin_module = "fiberplane::protocols::operations")]
#[serde(rename_all = "camelCase")]
pub struct UpdateDataSourceOperation {
    /// The identifier of this data-source within the notebook
    pub name: String,

    /// The new data-source content
    pub data_source: Box<NotebookDataSource>,

    /// The old data-source content
    pub old_data_source: Box<NotebookDataSource>,
}

/// Remove an data-source to an notebook.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Serializable)]
#[fp(rust_plugin_module = "fiberplane::protocols::operations")]
#[serde(rename_all = "camelCase")]
pub struct RemoveDataSourceOperation {
    /// The identifier of this data-source within the notebook
    pub name: String,

    /// The previous data-source content
    pub data_source: Box<NotebookDataSource>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Serializable)]
#[fp(rust_plugin_module = "fiberplane::protocols::operations")]
#[serde(rename_all = "camelCase")]
pub struct CellWithIndex {
    pub cell: Cell,
    pub index: u32,
}

/// Add an label to an notebook.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Serializable)]
#[fp(rust_plugin_module = "fiberplane::protocols::operations")]
#[serde(rename_all = "camelCase")]
pub struct AddLabelOperation {
    /// The new label
    pub label: Label,
}

/// Replace an label in an notebook.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Serializable)]
#[fp(rust_plugin_module = "fiberplane::protocols::operations")]
#[serde(rename_all = "camelCase")]
pub struct ReplaceLabelOperation {
    // The previous label
    pub old_label: Label,

    // The new label
    pub new_label: Label,
}

/// Remove an label in an notebook.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Serializable)]
#[fp(rust_plugin_module = "fiberplane::protocols::operations")]
#[serde(rename_all = "camelCase")]
pub struct RemoveLabelOperation {
    pub label: Label,
}
