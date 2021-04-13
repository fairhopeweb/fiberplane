use crate::protocols::core::Cell;
use crate::protocols::core::TimeRange;
use serde::{Deserialize, Serialize};

/// An operation is the representation for a mutation to be performed to a notebook.
///
/// Operations are intended to be atomic (they should either be performed in their entirety or not
/// at all), while also capturing the intent of the user.
///
/// For more information, please see RFC 8:
///   https://www.notion.so/fiberplane/RFC-8-Notebook-Operations-f9d18676d0d9437d81de30faa219deb4
#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Operation {
    AddCells(AddCellsOperation),
    MergeCells(MergeCellsOperation),
    MoveCells(MoveCellsOperation),
    RemoveCells(RemoveCellsOperation),
    SplitCell(SplitCellOperation),
    UpdateCell(UpdateCellOperation),
    UpdateNotebookTimeRange(UpdateNotebookTimeRangeOperation),
    UpdateNotebookTitle(UpdateNotebookTitleOperation),
}

/// Adds one or more cells at the given position.
#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
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
#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MergeCellsOperation {
    /// Optional text we want to "glue" between the content of the target cell and the source cell.
    /// This is useful if we want to revert a `SplitCellOperation` that contained selected text.
    pub glue_text: Option<String>,

    /// ID of the source cell that will be merged into the target cell. This must be the cell
    /// immediately after the target cell. This ID is explicitly specified to be able to reuse
    /// the same ID if the merge operation is reverted.
    pub source_cell: Cell,

    /// The length of the text content of the target cell right before the merge. This is the index
    /// at which we will want to split the cell if we need to revert the merge.
    pub target_content_length: usize,

    /// ID of the target cell into which the merge will be performed.
    pub target_cell_id: String,
}

/// Moves one or more cells.
#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MoveCellsOperation {
    /// IDs of all the cells to be moved.
    ///
    /// These must be adjacent and given in the order they appear in the notebook.
    pub cell_ids: Vec<String>,

    /// Index the cells will be moved from. This is the index of the first cell before the move.
    pub from_index: usize,

    /// Index the cells will be moved to. This is the index of the first cell after the move.
    pub to_index: usize,
}

/// Removes one or more cells.
#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RemoveCellsOperation {
    /// The removed cells, including their index before the removal.
    pub removed_cells: Vec<CellWithIndex>,

    /// Optional cells that referenced the removed cells and which are affected by the removal.
    ///
    /// If a referencing cell *only* references the removed cells, it may be cascade removed.
    /// Otherwise, the removed cells may simply be unreferenced and the cell will be retained.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub referencing_cells: Option<Vec<CellWithIndex>>,
}

/// Splits a cell at the given position.
#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SplitCellOperation {
    /// ID of the cell that will be split.
    pub cell_id: String,

    /// The character index inside the cell to split at.
    pub split_index: usize,

    /// If any text was selected at the moment of splitting, that selection is removed; only the
    /// part before the selection is retained in the split cell, while only the part after the
    /// selection ends up in the new cell.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub removed_text: Option<String>,

    /// Newly created cell after the split.
    pub new_cell: Cell,
}

/// Updates arbitrary properties of a cell.
///
/// **FIXME:** Because this operation is so coarse, it currently breaks assumptions about intent and
///            convergence.
#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
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
#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UpdateNotebookTimeRangeOperation {
    pub old_time_range: TimeRange,
    pub time_range: TimeRange,
}

/// Updates the notebook title.
#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UpdateNotebookTitleOperation {
    pub old_title: String,
    pub title: String,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CellWithIndex {
    pub cell: Cell,
    pub index: usize,
}
