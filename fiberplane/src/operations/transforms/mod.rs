mod cells;
pub(crate) mod convergence;
mod data_sources;
mod get_cell_index_changes;
mod labels;
mod notebook;

use crate::{
    operations::error::*,
    protocols::{core::*, operations::*},
};
use cells::*;
use data_sources::*;
use labels::*;
use notebook::*;

/// Allows `transform_operation()` to query for the state of the notebook as it was at the revision
/// immediately *before* predecessor gets applied.
pub trait TransformOperationState {
    /// Returns a cell by ID.
    fn cell(&self, id: &str) -> Option<&Cell>;

    /// Returns the index of the cell with the given ID, if it exists.
    fn cell_index(&self, id: &str) -> Option<u32>;
}

/// Transforms the successor operation such that it can be correctly applied after the predecessor
/// is already applied.
///
/// The goal is that applying the resulting operation on top of `predecessor` will yield the same
/// notebook as applying `predecessor` on top of `successor`, while preserving as much of
/// `successor`'s *intent* as possible.
///
/// If two operations can be executed in either order, with the same result, we achieve convergence.
/// In practice however, many of our operations are too coarse to achieve this, and we rely on the
/// fact the server will reject outdated operations so that everything will be applied in the same
/// order. For those operations, we explicitly exclude them from convergence tests.
pub fn transform_operation(
    state: &dyn TransformOperationState,
    successor: &Operation,
    predecessor: &Operation,
) -> Result<Option<Operation>, Error> {
    use Operation::*;
    match successor {
        MoveCells(op) => transform_move_cells_operation(state, op, predecessor),
        ReplaceCells(op) => transform_replace_cells_operation(state, op, predecessor),
        ReplaceText(op) => transform_replace_text_operation(state, op, predecessor),
        UpdateNotebookTimeRange(op) => Ok(transform_update_notebook_time_range_operation(
            state,
            op,
            predecessor,
        )),
        UpdateNotebookTitle(op) => Ok(transform_update_notebook_title_operation(
            state,
            op,
            predecessor,
        )),
        SetSelectedDataSource(op) => Ok(transform_set_selected_data_source_operation(
            state,
            op,
            predecessor,
        )),
        AddLabel(op) => Ok(transform_add_label_operation(state, op, predecessor)),
        ReplaceLabel(op) => Ok(transform_replace_label_operation(state, op, predecessor)),
        RemoveLabel(op) => Ok(transform_remove_label_operation(state, op, predecessor)),
    }
}
