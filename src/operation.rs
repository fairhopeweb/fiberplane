use serde::Deserialize;

use crate::{Cell, CursorPosition};

/// An operation is the representation for a mutation to be performed to a notebook.
///
/// Operations are intended to be atomic (they should either be performed in their entirety or not
/// at all), while also capturing the intent of the user.
///
/// For more information, please see RFC 8:
///   https://www.notion.so/fiberplane/RFC-8-Notebook-Operations-f9d18676d0d9437d81de30faa219deb4
#[derive(Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Operation {
    AddCells(AddCellsOperation),
    MergeCells(MergeCellsOperation),
    RemoveCells(RemoveCellsOperation),
    SplitCell(SplitCellOperation),
    SwapCells(SwapCellsOperation),
    UpdateCell(UpdateCellOperation),
}

/// Adds one or more cells at the given position.
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddCellsOperation {
    pub cells: Vec<Cell>,
    pub position: AddCellsPosition,
}

/// The position where to insert newly added cells. Either before or after the given reference cell.
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddCellsPosition {
    pub reference_id: String,
    pub relation: AddCellsRelation,
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AddCellsRelation {
    Before,
    After,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CellWithIndex {
    pub cell: Cell,
    pub index: u32,
}

/// Merges the source cell into the target cell by appending its content.
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MergeCellsOperation {
    pub source_id: String,
    pub target_id: String,
}

/// State of a notebook to apply an operation to.
///
/// Clients are responsible for making sure all cells that are relevant to a given operation are
/// included in this struct. A naive client may simply include all cells.
#[derive(Deserialize)]
pub struct NotebookState {
    pub cells: Vec<CellWithIndex>,
}

/// Removes one or more cells with the given IDs.
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoveCellsOperation {
    pub cell_ids: Vec<String>,
}

/// Splits a cell at the given cursor position.
///
/// If the cursor position includes an active selection, that selection is removed; only the part
/// before the selection is retained in the split cell, while only the part after the selection ends
/// up in the new cell.
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SplitCellOperation {
    pub cell_id: String,
    pub cursor_position: CursorPosition,
    pub new_cell_id: String,
}

/// Swaps the position of two cells.
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SwapCellsOperation {
    pub cell_id1: String,
    pub cell_id2: String,
}

/// Updates arbitrary properties of a cell.
///
/// **FIXME:** Because this operation is so coarse, it currently breaks assumptions about intent and
///            convergence.
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateCellOperation {
    pub updated_cell: Cell,
}
