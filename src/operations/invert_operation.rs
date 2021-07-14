use crate::protocols::operations::*;

/// Inverts an operation.
///
/// When the resulting operation is applied, it effectively reverts the original
/// operation.
///
/// Note there is no guarantee for a full preservation on a round-trip, so
/// inverting an inverted  operation does not necessarily need to result in an
/// operation that is exactly the same as the original.
pub fn invert_operation(operation: &Operation) -> Operation {
    use Operation::*;
    match operation {
        AddCells(operation) => invert_add_cells_operation(operation),
        MergeCells(operation) => invert_merge_cells_operation(operation),
        MoveCells(operation) => invert_move_cells_operation(operation),
        RemoveCells(operation) => invert_remove_cells_operation(operation),
        SplitCell(operation) => invert_split_cells_operation(operation),
        UpdateCell(operation) => invert_update_cell_operation(operation),
        UpdateNotebookTimeRange(operation) => invert_update_notebook_time_range(operation),
        UpdateNotebookTitle(operation) => invert_update_notebook_title(operation),
        AddDataSource(operation) => invert_add_data_source_operation(operation),
        UpdateDataSource(operation) => invert_update_data_source_operation(operation),
        RemoveDataSource(operation) => invert_remove_data_source_operation(operation),
    }
}

fn invert_add_cells_operation(operation: &AddCellsOperation) -> Operation {
    Operation::RemoveCells(RemoveCellsOperation {
        referencing_cells: operation.referencing_cells.as_ref().cloned(),
        removed_cells: operation.cells.clone(),
    })
}

fn invert_merge_cells_operation(operation: &MergeCellsOperation) -> Operation {
    Operation::SplitCell(SplitCellOperation {
        cell_id: operation.target_cell_id.clone(),
        new_cell: operation.source_cell.clone(),
        referencing_cells: operation.referencing_cells.clone(),
        removed_text: operation.glue_text.clone(),
        split_index: operation.target_content_length,
    })
}

fn invert_move_cells_operation(operation: &MoveCellsOperation) -> Operation {
    Operation::MoveCells(MoveCellsOperation {
        cell_ids: operation.cell_ids.clone(),
        from_index: operation.to_index,
        to_index: operation.from_index,
    })
}

fn invert_remove_cells_operation(operation: &RemoveCellsOperation) -> Operation {
    let mut added_cells: Vec<CellWithIndex> = operation.removed_cells.clone();
    let mut newly_referencing_cells: Vec<CellWithIndex> = vec![];

    if let Some(referencing_cells) = &operation.referencing_cells {
        for referencing_cell in referencing_cells {
            let mut source_ids = referencing_cell.cell.source_ids();
            for removed_cell in &operation.removed_cells {
                if let Some(index) = source_ids
                    .iter()
                    .position(|id| id == removed_cell.cell.id())
                {
                    source_ids.remove(index);
                }
            }
            if source_ids.is_empty() {
                added_cells.push(referencing_cell.clone());
            } else {
                newly_referencing_cells.push(referencing_cell.clone());
            }
        }
    }

    Operation::AddCells(AddCellsOperation {
        cells: added_cells,
        referencing_cells: if newly_referencing_cells.is_empty() {
            None
        } else {
            Some(newly_referencing_cells)
        },
    })
}

fn invert_split_cells_operation(operation: &SplitCellOperation) -> Operation {
    Operation::MergeCells(MergeCellsOperation {
        glue_text: operation.removed_text.clone(),
        referencing_cells: operation.referencing_cells.clone(),
        source_cell: operation.new_cell.clone(),
        target_cell_id: operation.cell_id.clone(),
        target_content_length: operation.split_index,
    })
}

fn invert_update_cell_operation(operation: &UpdateCellOperation) -> Operation {
    Operation::UpdateCell(UpdateCellOperation {
        updated_cell: operation.old_cell.clone(),
        old_cell: operation.updated_cell.clone(),
    })
}

fn invert_update_notebook_time_range(operation: &UpdateNotebookTimeRangeOperation) -> Operation {
    Operation::UpdateNotebookTimeRange(UpdateNotebookTimeRangeOperation {
        time_range: operation.old_time_range.clone(),
        old_time_range: operation.time_range.clone(),
    })
}

fn invert_update_notebook_title(operation: &UpdateNotebookTitleOperation) -> Operation {
    Operation::UpdateNotebookTitle(UpdateNotebookTitleOperation {
        title: operation.old_title.clone(),
        old_title: operation.title.clone(),
    })
}

fn invert_add_data_source_operation(operation: &AddDataSourceOperation) -> Operation {
    Operation::RemoveDataSource(RemoveDataSourceOperation {
        name: operation.name.clone(),
        data_source: operation.data_source.clone(),
    })
}

fn invert_update_data_source_operation(operation: &UpdateDataSourceOperation) -> Operation {
    Operation::UpdateDataSource(UpdateDataSourceOperation {
        name: operation.name.clone(),
        data_source: operation.old_data_source.clone(),
        old_data_source: operation.data_source.clone(),
    })
}

fn invert_remove_data_source_operation(operation: &RemoveDataSourceOperation) -> Operation {
    Operation::AddDataSource(AddDataSourceOperation {
        name: operation.name.clone(),
        data_source: operation.data_source.clone(),
    })
}
