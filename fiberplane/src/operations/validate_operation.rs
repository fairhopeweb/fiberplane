use super::ApplyOperationState;
use crate::{
    protocols::{
        core::Label,
        operations::*,
        realtime::{InvalidLabelRejectReason, RejectReason},
    },
    text_util::{char_count, char_index, char_slice},
};
use std::collections::HashSet;

/// Validates whether an operation may be applied to the given notebook state.
pub fn validate_operation(
    state: &dyn ApplyOperationState,
    operation: &Operation,
) -> Result<(), RejectReason> {
    use Operation::*;
    match operation {
        AddCells(operation) => validate_add_cells_operation(state, operation),
        MergeCells(operation) => validate_merge_cells_operation(state, operation),
        MoveCells(operation) => validate_move_cells_operation(state, operation),
        RemoveCells(operation) => validate_remove_cells_operation(state, operation),
        ReplaceText(operation) => validate_replace_text_operation(state, operation),
        SplitCell(operation) => validate_split_cells_operation(state, operation),
        UpdateCell(operation) => validate_update_cell_operation(state, operation),
        UpdateNotebookTimeRange(operation) => validate_update_notebook_time_range(state, operation),
        UpdateNotebookTitle(operation) => validate_update_notebook_title(state, operation),
        AddDataSource(operation) => validate_add_data_source_operation(state, operation),
        UpdateDataSource(operation) => validate_update_data_source_operation(state, operation),
        RemoveDataSource(operation) => validate_remove_data_source_operation(state, operation),
        AddLabel(operation) => validate_add_label_operation(state, operation),
        ReplaceLabel(operation) => validate_replace_label_operation(state, operation),
        RemoveLabel(operation) => validate_remove_label_operation(state, operation),
    }
}

fn validate_add_cells_operation(
    state: &dyn ApplyOperationState,
    operation: &AddCellsOperation,
) -> Result<(), RejectReason> {
    let all_cell_ids = state.all_cell_ids();
    let mut new_cell_ids = HashSet::new();

    for cell_with_index in operation.cells.iter() {
        let cell_id = cell_with_index.cell.id();
        if all_cell_ids.contains(&cell_id.as_str()) || !new_cell_ids.insert(cell_id) {
            return Err(RejectReason::DuplicateCellId {
                cell_id: cell_id.clone(),
            });
        }
    }

    let mut indices: Vec<_> = operation.cells.iter().map(|cell| cell.index).collect();
    indices.sort_unstable();
    for (i, &index) in indices.iter().enumerate() {
        if index > (all_cell_ids.len() + i) as u32 {
            return Err(RejectReason::CellIndexOutOfBounds);
        }
    }

    validate_referencing_cells(state, &operation.referencing_cells)
}

fn validate_label(label: &Label) -> Result<(), RejectReason> {
    if let Err(validation_error) = label.validate() {
        Err(RejectReason::InvalidLabel(InvalidLabelRejectReason {
            key: label.key.clone(),
            validation_error,
        }))
    } else {
        Ok(())
    }
}

fn validate_merge_cells_operation(
    state: &dyn ApplyOperationState,
    operation: &MergeCellsOperation,
) -> Result<(), RejectReason> {
    let source_cell = match state.cell(operation.source_cell.id()) {
        Some(cell) => cell,
        None => {
            return Err(RejectReason::CellNotFound {
                cell_id: operation.source_cell.id().clone(),
            })
        }
    };

    if source_cell != &operation.source_cell {
        return Err(RejectReason::InconsistentState);
    }

    let target_cell = match state.cell(&operation.target_cell_id) {
        Some(cell) => cell,
        None => {
            return Err(RejectReason::CellNotFound {
                cell_id: operation.target_cell_id.clone(),
            })
        }
    };

    if target_cell.content().map(char_count).unwrap_or_default() != operation.target_content_length
    {
        return Err(RejectReason::InconsistentState);
    }

    validate_referencing_cells(state, &operation.referencing_cells)
}

fn validate_move_cells_operation(
    state: &dyn ApplyOperationState,
    operation: &MoveCellsOperation,
) -> Result<(), RejectReason> {
    let mut cell_ids = HashSet::new();
    for (i, cell_id) in operation.cell_ids.iter().enumerate() {
        if !cell_ids.insert(cell_id) {
            return Err(RejectReason::DuplicateCellId {
                cell_id: cell_id.clone(),
            });
        }

        if let Some(cell_with_index) = state.cell_with_index(cell_id) {
            if cell_with_index.index != operation.from_index + i as u32 {
                return Err(RejectReason::InconsistentState);
            }
        } else {
            return Err(RejectReason::CellNotFound {
                cell_id: cell_id.clone(),
            });
        }
    }

    if operation.to_index > (state.all_cell_ids().len() - operation.cell_ids.len()) as u32 {
        return Err(RejectReason::CellIndexOutOfBounds);
    }

    Ok(())
}

fn validate_referencing_cells(
    state: &dyn ApplyOperationState,
    referencing_cells: &Option<Vec<CellWithIndex>>,
) -> Result<(), RejectReason> {
    if let Some(referencing_cells) = referencing_cells {
        for referencing_cell in referencing_cells {
            let id = referencing_cell.cell.id();
            if state.cell(id).is_none() {
                return Err(RejectReason::CellNotFound {
                    cell_id: id.clone(),
                });
            }
        }
    }

    Ok(())
}

fn validate_remove_cells_operation(
    state: &dyn ApplyOperationState,
    operation: &RemoveCellsOperation,
) -> Result<(), RejectReason> {
    for cell in operation.removed_cells.iter() {
        if let Some(existing_cell) = state.cell_with_index(cell.cell.id()) {
            if existing_cell.cell != &cell.cell || existing_cell.index != cell.index {
                return Err(RejectReason::InconsistentState);
            }
        } else {
            return Err(RejectReason::CellNotFound {
                cell_id: cell.cell.id().clone(),
            });
        }
    }

    validate_referencing_cells(state, &operation.referencing_cells)
}

fn validate_replace_text_operation(
    state: &dyn ApplyOperationState,
    operation: &ReplaceTextOperation,
) -> Result<(), RejectReason> {
    if let Some(cell) = state.cell(&operation.cell_id) {
        if let Some(text) = cell.text() {
            let current_text_at_offset = char_slice(
                text,
                operation.offset as usize,
                (operation.offset + char_count(&operation.old_text)) as usize,
            );
            if operation.old_text != current_text_at_offset {
                return Err(RejectReason::InconsistentState);
            }
        } else {
            return Err(RejectReason::NoTextCell {
                cell_id: operation.cell_id.clone(),
            });
        }
    } else {
        return Err(RejectReason::CellNotFound {
            cell_id: operation.cell_id.clone(),
        });
    }

    Ok(())
}

fn validate_split_cells_operation(
    state: &dyn ApplyOperationState,
    operation: &SplitCellOperation,
) -> Result<(), RejectReason> {
    let cell = match state.cell(&operation.cell_id) {
        Some(cell) => cell,
        None => {
            return Err(RejectReason::CellNotFound {
                cell_id: operation.cell_id.clone(),
            })
        }
    };

    let content = match cell.content() {
        Some(content) => content,
        None => {
            return Err(RejectReason::NoTextCell {
                cell_id: operation.cell_id.clone(),
            })
        }
    };

    if state
        .all_cell_ids()
        .contains(&operation.new_cell.id().as_str())
    {
        return Err(RejectReason::DuplicateCellId {
            cell_id: operation.new_cell.id().clone(),
        });
    }

    let new_cell_content_len = operation
        .new_cell
        .content()
        .map(char_count)
        .unwrap_or_default();
    let removed_text_len = operation
        .removed_text
        .as_ref()
        .map(char_count)
        .unwrap_or_default();
    if char_count(content) != operation.split_index + removed_text_len + new_cell_content_len {
        return Err(RejectReason::InconsistentState);
    }

    if &content[char_index(content, operation.split_index + removed_text_len) as usize..]
        != operation.new_cell.content().unwrap_or_default()
    {
        return Err(RejectReason::InconsistentState);
    }

    validate_referencing_cells(state, &operation.referencing_cells)
}

fn validate_update_cell_operation(
    state: &dyn ApplyOperationState,
    operation: &UpdateCellOperation,
) -> Result<(), RejectReason> {
    if let Some(cell) = state.cell(operation.old_cell.id()) {
        if cell != operation.old_cell.as_ref() {
            return Err(RejectReason::InconsistentState);
        }

        if operation.updated_cell.id() != operation.old_cell.id()
            && state
                .all_cell_ids()
                .contains(&operation.updated_cell.id().as_str())
        {
            return Err(RejectReason::DuplicateCellId {
                cell_id: operation.updated_cell.id().clone(),
            });
        }
    } else {
        return Err(RejectReason::CellNotFound {
            cell_id: operation.old_cell.id().clone(),
        });
    }

    Ok(())
}

fn validate_update_notebook_time_range(
    _: &dyn ApplyOperationState,
    _: &UpdateNotebookTimeRangeOperation,
) -> Result<(), RejectReason> {
    Ok(())
}

fn validate_update_notebook_title(
    _: &dyn ApplyOperationState,
    _: &UpdateNotebookTitleOperation,
) -> Result<(), RejectReason> {
    Ok(())
}

fn validate_add_data_source_operation(
    _: &dyn ApplyOperationState,
    _: &AddDataSourceOperation,
) -> Result<(), RejectReason> {
    Ok(())
}

fn validate_update_data_source_operation(
    _: &dyn ApplyOperationState,
    _: &UpdateDataSourceOperation,
) -> Result<(), RejectReason> {
    Ok(())
}

fn validate_remove_data_source_operation(
    _: &dyn ApplyOperationState,
    _: &RemoveDataSourceOperation,
) -> Result<(), RejectReason> {
    Ok(())
}

fn validate_add_label_operation(
    _: &dyn ApplyOperationState,
    operation: &AddLabelOperation,
) -> Result<(), RejectReason> {
    validate_label(&operation.label)
}

fn validate_replace_label_operation(
    _: &dyn ApplyOperationState,
    operation: &ReplaceLabelOperation,
) -> Result<(), RejectReason> {
    validate_label(&operation.new_label)
}

fn validate_remove_label_operation(
    _: &dyn ApplyOperationState,
    _: &RemoveLabelOperation,
) -> Result<(), RejectReason> {
    Ok(())
}
