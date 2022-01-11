use crate::protocols::operations::*;

/// Returns the IDs of all the cells that should be included in the notebook state passed to
/// `apply_operation()`.
///
/// For performance reasons, you might want to reimplement this function elsewhere. For instance,
/// Studio uses a TypeScript implementation in order to avoid having to serialize the operation
/// to JSON unnecessarily.
///
/// Still, this implementation should be treated as the authoritative reference implementation,
/// as it is the one that is used by the test harness, and therefore most likely to be correct.
pub fn relevant_cell_ids_for_operation(operation: &Operation) -> Vec<String> {
    match operation {
        Operation::AddCells(_) => vec![],
        Operation::MergeCells(MergeCellsOperation { target_cell_id, .. }) => {
            vec![target_cell_id.clone()]
        }
        Operation::MoveCells(MoveCellsOperation { cell_ids, .. }) => cell_ids.clone(),
        Operation::RemoveCells(RemoveCellsOperation {
            referencing_cells,
            removed_cells,
            ..
        }) => {
            let mut relevant_cell_ids: Vec<String> =
                removed_cells.iter().map(|c| c.cell.id()).cloned().collect();
            if let Some(referencing_cells) = referencing_cells {
                for c in referencing_cells {
                    relevant_cell_ids.push(c.cell.id().clone());
                }
            }
            relevant_cell_ids
        }
        Operation::ReplaceText(ReplaceTextOperation { cell_id, .. }) => vec![cell_id.clone()],
        Operation::SplitCell(SplitCellOperation {
            cell_id,
            referencing_cells,
            ..
        }) => {
            let mut relevant_cell_ids = vec![cell_id.clone()];
            if let Some(referencing_cells) = referencing_cells {
                for c in referencing_cells {
                    relevant_cell_ids.push(c.cell.id().clone());
                }
            }
            relevant_cell_ids
        }
        Operation::UpdateCell(UpdateCellOperation { updated_cell, .. }) => {
            vec![updated_cell.id().clone()]
        }
        Operation::UpdateNotebookTimeRange(_)
        | Operation::UpdateNotebookTitle(_)
        | Operation::AddDataSource(_)
        | Operation::UpdateDataSource(_)
        | Operation::RemoveDataSource(_) => vec![],
    }
}
