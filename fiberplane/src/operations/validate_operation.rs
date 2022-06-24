use super::{
    utils::is_annotation_included_in_old_cell_formatting, ApplyOperationState, CellRefWithIndex,
};
use crate::{
    protocols::{
        core::{Cell, Label},
        operations::*,
        realtime::{InvalidLabelRejectReason, RejectReason},
    },
    text_util::{char_count, char_slice},
};
use std::{cmp::Ordering, collections::HashSet};

/// Validates whether an operation may be applied to the given notebook state.
pub fn validate_operation(
    state: &dyn ApplyOperationState,
    operation: &Operation,
) -> Result<(), RejectReason> {
    use Operation::*;
    match operation {
        MoveCells(operation) => validate_move_cells_operation(state, operation),
        ReplaceCells(operation) => validate_replace_cells_operation(state, operation),
        ReplaceText(operation) => validate_replace_text_operation(state, operation),
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

    if operation.cell_ids.is_empty() {
        return Err(RejectReason::FailedPrecondition {
            message: "`cell_ids` may not be empty".to_owned(),
        });
    }

    if operation.to_index > (state.all_cell_ids().len() - operation.cell_ids.len()) as u32 {
        return Err(RejectReason::CellIndexOutOfBounds);
    }

    Ok(())
}

fn validate_replace_cells_operation(
    state: &dyn ApplyOperationState,
    operation: &ReplaceCellsOperation,
) -> Result<(), RejectReason> {
    let all_cell_ids = state.all_cell_ids();

    // IDs of added or replaced cells, including new referencing cells.
    let mut new_cell_ids = HashSet::new();

    // Vector with the indices of all new cells (including new referencing
    // cells), sorted in ascending order.
    let mut new_cell_indices: Vec<u32> = Vec::new();

    // Make sure the new cells do not contain duplicate IDs and their indices
    // form a cohesive range.
    for cell_with_index in operation.new_cells.iter() {
        let cell_id = cell_with_index.id();
        // A cell ID may only be added if we're replacing an existing cell, or
        // adding a non-existing one. Otherwise, we'd introduce a duplicate ID.
        let may_add_id = if all_cell_ids.contains(&cell_id) {
            operation
                .old_cells
                .iter()
                .any(|old_cell_with_index| old_cell_with_index.id() == cell_id)
        } else {
            true
        };
        if !may_add_id || !new_cell_ids.insert(cell_id) {
            return Err(RejectReason::DuplicateCellId {
                cell_id: cell_id.to_owned(),
            });
        }

        if let Some(last_new_index) = new_cell_indices.last() {
            if cell_with_index.index != last_new_index + 1 {
                return Err(RejectReason::CellIndexOutOfBounds);
            }
        }
        new_cell_indices.push(cell_with_index.index);
    }

    // Make sure the new referencing cells do not contain duplicate IDs or
    // indices either.
    for cell_with_index in operation.new_referencing_cells.iter() {
        let cell_id = cell_with_index.id();
        // A cell ID may only be added if we're replacing an existing cell, or
        // adding a non-existing one.
        let may_add_id = if all_cell_ids.contains(&cell_id) {
            operation
                .old_referencing_cells
                .iter()
                .any(|old_cell_with_index| old_cell_with_index.id() == cell_id)
        } else {
            true
        };
        if !may_add_id || !new_cell_ids.insert(cell_id) {
            return Err(RejectReason::DuplicateCellId {
                cell_id: cell_id.to_owned(),
            });
        }

        match new_cell_indices.binary_search(&cell_with_index.index) {
            Ok(_) => {
                return Err(RejectReason::CellIndexOutOfBounds);
            }
            Err(insertion_index) => {
                new_cell_indices.insert(insertion_index, cell_with_index.index);
            }
        }
    }

    // Make sure the old cells all exist and their indices form a cohesive
    // range.
    let mut old_cell_index = None;
    for (i, cell_with_index) in operation.old_cells.iter().enumerate() {
        if let Some(existing_cell) = state.cell_with_index(cell_with_index.id()) {
            let cell_matches = if let Some(cell_to_compare) =
                get_existing_cell_to_compare(&existing_cell, i, operation)?
            {
                cell_with_index.cell == cell_to_compare
            } else {
                cell_with_index.cell == *existing_cell.cell
            };
            if !cell_matches || existing_cell.index != cell_with_index.index {
                return Err(RejectReason::InconsistentState);
            }
        } else {
            return Err(RejectReason::CellNotFound {
                cell_id: cell_with_index.id().to_owned(),
            });
        }

        if let Some(index) = old_cell_index {
            if cell_with_index.index != index + 1 {
                return Err(RejectReason::CellIndexOutOfBounds);
            }
        }
        old_cell_index = Some(cell_with_index.index);
    }

    // Make sure the old referencing cells do not contain duplicate IDs or
    // indices either.
    let mut old_cell_indices: Vec<u32> =
        operation.old_cells.iter().map(|cell| cell.index).collect();
    for cell_with_index in operation.old_referencing_cells.iter() {
        if let Some(existing_cell) = state.cell_with_index(cell_with_index.id()) {
            if existing_cell.cell != &cell_with_index.cell
                || existing_cell.index != cell_with_index.index
            {
                return Err(RejectReason::InconsistentState);
            }
        } else {
            return Err(RejectReason::CellNotFound {
                cell_id: cell_with_index.id().to_owned(),
            });
        }

        match old_cell_indices.binary_search(&cell_with_index.index) {
            Ok(_) => {
                return Err(RejectReason::DuplicateCellId {
                    cell_id: cell_with_index.id().to_owned(),
                })
            }
            Err(insertion_index) => {
                old_cell_indices.insert(insertion_index, cell_with_index.index);
            }
        }
    }

    // Validate the split offset.
    if operation.split_offset.is_some() {
        if operation.new_cells.is_empty() {
            return Err(RejectReason::FailedPrecondition {
                message: "`split_offset` cannot be set when `new_cells` is empty".to_owned(),
            });
        } else if operation.old_cells.is_empty() {
            return Err(RejectReason::FailedPrecondition {
                message: "`split_offset` cannot be set when `old_cells` is empty".to_owned(),
            });
        }
    }

    // Validate the merge offset.
    if operation.merge_offset.is_some() {
        if operation.new_cells.is_empty() {
            return Err(RejectReason::FailedPrecondition {
                message: "`merge_offset` cannot be set when `new_cells` is empty".to_owned(),
            });
        } else if operation.old_cells.is_empty() {
            return Err(RejectReason::FailedPrecondition {
                message: "`merge_offset` cannot be set when `old_cells` is empty".to_owned(),
            });
        }
    }

    if operation.new_cells.is_empty() && operation.old_cells.is_empty() {
        return Err(RejectReason::FailedPrecondition {
            message: "`new_cells` and `old_cells` cannot be both empty".to_owned(),
        });
    }

    validate_replace_cell_indices(
        operation,
        all_cell_ids.len(),
        old_cell_indices,
        new_cell_indices,
    )
}

/// Validates that all collected indices from a ReplaceText operation are
/// correct.
fn validate_replace_cell_indices(
    operation: &ReplaceCellsOperation,
    mut num_cells: usize,
    mut old_cell_indices: Vec<u32>,
    new_cell_indices: Vec<u32>,
) -> Result<(), RejectReason> {
    // Subtract the amount of old cells from `num_cells` to make sure early
    // iterations of the loop cannot reference indices that are out of range
    // for them:
    num_cells -= operation.old_cells.len() + operation.old_referencing_cells.len();

    // The delta keeps track exactly where new cells should be located,
    // relative to their old cells.
    let mut delta: i64 = 0;

    for (i, &index) in new_cell_indices.iter().enumerate() {
        // Update the delta and make sure `old_cell_indices` only contains
        // the following indices for the next iteration:
        let matching_old_index = ((index as i64) - delta) as u32;
        let num_skipped_old_indices = old_cell_indices
            .iter()
            .filter(|&&old_index| old_index < matching_old_index)
            .count();
        old_cell_indices.retain(|&old_index| old_index > matching_old_index);
        delta -= num_skipped_old_indices as i64;

        if let Some(new_cell_id) = operation
            .new_cells
            .iter()
            .find(|cell| cell.index == index)
            .map(CellWithIndex::id)
        {
            if let Some(old_cell_index) = operation
                .old_cells
                .iter()
                .find(|cell| cell.id() == new_cell_id)
                .map(|cell| cell.index)
            {
                if old_cell_index != ((index as i64) - delta) as u32 {
                    return Err(RejectReason::CellIndexOutOfBounds);
                }
            } else if !operation
                .old_cells
                .iter()
                .any(|old_cell| old_cell.index == matching_old_index)
            {
                delta += 1;
            }
        } else if let Some(new_referencing_cell_id) = operation
            .new_referencing_cells
            .iter()
            .find(|cell| cell.index == index)
            .map(CellWithIndex::id)
        {
            if let Some(old_referencing_cell_index) = operation
                .old_referencing_cells
                .iter()
                .find(|cell| cell.id() == new_referencing_cell_id)
                .map(|cell| cell.index)
            {
                if old_referencing_cell_index != ((index as i64) - delta) as u32 {
                    return Err(RejectReason::CellIndexOutOfBounds);
                }
            } else {
                delta += 1;
            }
        } else {
            return Err(RejectReason::FailedPrecondition {
                message: "Unexpected validation error".to_owned(),
            });
        }

        if index > (num_cells + i) as u32 {
            return Err(RejectReason::CellIndexOutOfBounds);
        }
    }

    Ok(())
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

/// Returns a cell to compare the operation's old cell against. This takes care
/// of correctly slicing cells in the presence of a split or merge offset in a
/// `ReplaceText` operation. Returns `None` if the old cell can be compared to
/// the original (non-sliced) existing cell.
pub fn get_existing_cell_to_compare(
    existing_cell: &CellRefWithIndex,
    i: usize,
    operation: &ReplaceCellsOperation,
) -> Result<Option<Cell>, RejectReason> {
    let split_offset = if i == 0 { operation.split_offset } else { None };
    let merge_offset = if i == operation.old_cells.len() - 1 {
        operation.merge_offset
    } else {
        None
    };

    if split_offset.is_none() && merge_offset.is_none() {
        return Ok(None);
    }

    let text = match existing_cell.text() {
        Some(text) => text,
        None => {
            return Err(RejectReason::NoTextCell {
                cell_id: existing_cell.id().to_owned(),
            })
        }
    };
    let text_len = char_count(text);

    if let Some(merge_offset) = merge_offset {
        if merge_offset > text_len {
            return Err(RejectReason::FailedPrecondition {
                message: "`merge_offset` is outside of target cell's text length".to_owned(),
            });
        }
    };

    if let Some(split_offset) = split_offset {
        if split_offset > text_len {
            return Err(RejectReason::FailedPrecondition {
                message: "`split_offset` is outside of target cell's text length".to_owned(),
            });
        }

        let end_offset = match merge_offset {
            Some(merge_offset) => {
                if merge_offset < split_offset {
                    return Err(RejectReason::FailedPrecondition {
                        message: "`merge_offset` cannot be before `split_offset`".to_owned(),
                    });
                }

                merge_offset
            }
            None => text_len,
        };

        let split_text = char_slice(text, split_offset as usize, end_offset as usize);
        let cell_to_compare = if let Some(formatting) = existing_cell.formatting() {
            let split_formatting = formatting
                .iter()
                .filter(|&annotation| {
                    if annotation.offset > split_offset
                        && (annotation.offset < end_offset
                            || (merge_offset.is_none() && annotation.offset == end_offset))
                    {
                        true
                    } else if annotation.offset == split_offset
                        || Some(annotation.offset) == merge_offset
                    {
                        is_annotation_included_in_old_cell_formatting(
                            &annotation.annotation,
                            (annotation.offset as i64 - (split_offset as i64)) as u32,
                            operation,
                            existing_cell.id(),
                        )
                    } else {
                        false
                    }
                })
                .map(|annotation| annotation.translate(-(split_offset as i64)))
                .collect();
            existing_cell
                .cell
                .with_rich_text(split_text, split_formatting)
        } else {
            existing_cell.cell.with_text(split_text)
        };

        return Ok(Some(cell_to_compare));
    }

    if let Some(merge_offset) = merge_offset {
        let merge_text = char_slice(text, 0, merge_offset as usize);
        let cell_to_compare = if let Some(formatting) = existing_cell.formatting() {
            let merge_formatting = formatting
                .iter()
                .filter(|&annotation| match annotation.offset.cmp(&merge_offset) {
                    Ordering::Less => true,
                    Ordering::Equal => is_annotation_included_in_old_cell_formatting(
                        &annotation.annotation,
                        annotation.offset,
                        operation,
                        existing_cell.id(),
                    ),
                    Ordering::Greater => false,
                })
                .cloned()
                .collect();
            existing_cell
                .cell
                .with_rich_text(merge_text, merge_formatting)
        } else {
            existing_cell.cell.with_text(merge_text)
        };

        return Ok(Some(cell_to_compare));
    }

    unreachable!()
}
