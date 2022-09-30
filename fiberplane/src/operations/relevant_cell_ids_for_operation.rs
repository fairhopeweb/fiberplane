use crate::protocols::operations::*;

/// Returns the IDs of all the cells that should be included as part of the `ApplyOperationState`.
pub fn relevant_cell_ids_for_operation(operation: &Operation) -> Vec<&str> {
    match operation {
        Operation::MoveCells(MoveCellsOperation { cell_ids, .. }) => {
            cell_ids.iter().map(String::as_str).collect()
        }
        Operation::ReplaceCells(operation) => {
            operation.all_old_cells().map(CellWithIndex::id).collect()
        }
        Operation::ReplaceText(ReplaceTextOperation { cell_id, .. }) => vec![cell_id],
        Operation::UpdateNotebookTimeRange(_)
        | Operation::UpdateNotebookTitle(_)
        | Operation::SetSelectedDataSource(_)
        | Operation::AddLabel(_)
        | Operation::ReplaceLabel(_)
        | Operation::RemoveLabel(_) => vec![],
    }
}
