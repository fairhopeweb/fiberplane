use crate::protocols::operations::*;

/// Returns the IDs of all the cells that should be included as part of the `ApplyOperationState`.
pub fn relevant_cell_ids_for_operation(operation: &Operation) -> Vec<String> {
    match operation {
        Operation::AddCells(AddCellsOperation {
            referencing_cells, ..
        }) => referencing_cells
            .as_ref()
            .map(|cells| cells.iter().map(|cell| cell.cell.id().clone()).collect())
            .unwrap_or_default(),
        Operation::MergeCells(MergeCellsOperation {
            referencing_cells,
            source_cell,
            target_cell_id,
            ..
        }) => {
            let mut relevant_cell_ids = vec![source_cell.id().clone(), target_cell_id.clone()];
            if let Some(referencing_cells) = referencing_cells {
                for c in referencing_cells {
                    relevant_cell_ids.push(c.cell.id().clone());
                }
            }
            relevant_cell_ids
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
        | Operation::RemoveDataSource(_)
        | Operation::AddLabel(_)
        | Operation::ReplaceLabel(_)
        | Operation::RemoveLabel(_) => vec![],
    }
}
