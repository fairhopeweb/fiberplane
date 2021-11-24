use crate::operations::error::*;
use crate::protocols::{core::*, operations::*};
use std::cmp::Ordering;

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
        AddCells(o) => transform_add_cells_operation(state, o, predecessor),
        MergeCells(o) => transform_merge_cells_operation(state, o, predecessor),
        MoveCells(o) => transform_move_cells_operation(state, o, predecessor),
        RemoveCells(o) => transform_remove_cells_operation(state, o, predecessor),
        SplitCell(o) => transform_split_cell_operation(state, o, predecessor),
        UpdateCell(o) => transform_update_cell_operation(state, o, predecessor),
        UpdateNotebookTimeRange(o) => Ok(transform_update_notebook_time_range_operation(
            state,
            o,
            predecessor,
        )),
        UpdateNotebookTitle(o) => Ok(transform_update_notebook_title_operation(
            state,
            o,
            predecessor,
        )),
        AddDataSource(o) => Ok(transform_add_data_source_operation(state, o, predecessor)),
        UpdateDataSource(o) => Ok(transform_update_data_source_operation(
            state,
            o,
            predecessor,
        )),
        RemoveDataSource(o) => Ok(transform_remove_data_source_operation(
            state,
            o,
            predecessor,
        )),
    }
}

pub fn transform_add_cells_operation(
    state: &dyn TransformOperationState,
    successor: &AddCellsOperation,
    predecessor: &Operation,
) -> Result<Option<Operation>, Error> {
    let operation = match predecessor {
        Operation::AddCells(predecessor) => {
            let predecessor_cell_ids: Vec<&String> =
                predecessor.cells.iter().map(|c| c.cell.id()).collect();
            if successor
                .cells
                .iter()
                .any(|c| predecessor_cell_ids.contains(&c.cell.id()))
            {
                // Both operations try to add the same cell(s). It's a legal possibility if one or
                // more cells got removed and then multiple clients try to restore them
                // simultaneously. In that case, we discard `successor`.
                None
            } else {
                let mut transformed = successor.clone();
                for predecessor_cell in &predecessor.cells {
                    for transformed_cell in &mut transformed.cells {
                        if predecessor_cell.index < transformed_cell.index
                            || (predecessor_cell.index == transformed_cell.index
                                && predecessor_cell.cell.id() < transformed_cell.cell.id())
                        {
                            transformed_cell.index += 1;
                        }
                    }
                }
                if let Some(referencing_cells) = transformed.referencing_cells.as_mut() {
                    adjust_indices_for_added_cells(referencing_cells, predecessor);
                }
                Some(Operation::AddCells(transformed))
            }
        }
        Operation::MergeCells(predecessor) => Some(Operation::AddCells(AddCellsOperation {
            cells: with_adjusted_indices_for_merged_cells(state, &successor.cells, predecessor)?,
            referencing_cells: match successor.referencing_cells.as_ref() {
                Some(referencing_cells) => Some(with_adjusted_indices_for_merged_cells(
                    state,
                    referencing_cells,
                    predecessor,
                )?),
                None => None,
            },
        })),
        Operation::MoveCells(predecessor) => Some(Operation::AddCells(AddCellsOperation {
            cells: with_adjusted_indices_for_moved_cells(&successor.cells, predecessor),
            referencing_cells: successor
                .referencing_cells
                .as_ref()
                .map(|cells| with_adjusted_indices_for_moved_cells(cells, predecessor)),
        })),
        Operation::RemoveCells(predecessor) => Some(Operation::AddCells(AddCellsOperation {
            cells: with_adjusted_indices_for_removed_cells(&successor.cells, predecessor),
            referencing_cells: successor
                .referencing_cells
                .as_ref()
                .map(|cells| with_adjusted_indices_for_removed_cells(cells, predecessor)),
        })),
        Operation::SplitCell(predecessor) => Some(Operation::AddCells(AddCellsOperation {
            cells: with_adjusted_indices_for_split_cell(state, &successor.cells, predecessor)?,
            referencing_cells: match successor.referencing_cells.as_ref() {
                Some(cells) => {
                    let mut cells = with_added_source_ids(
                        cells,
                        &[predecessor.new_cell.id()],
                        &predecessor.referencing_cells,
                    );
                    adjust_indices_for_split_cell(state, &mut cells, predecessor)?;
                    Some(cells)
                }
                None => None,
            },
        })),
        Operation::UpdateCell(predecessor) => Some(Operation::AddCells(AddCellsOperation {
            cells: successor.cells.clone(),
            referencing_cells: match successor.referencing_cells.as_ref() {
                Some(cells) => Some(with_merged_source_ids(state, cells, predecessor)?),
                None => None,
            },
        })),
        Operation::UpdateNotebookTimeRange(_)
        | Operation::UpdateNotebookTitle(_)
        | Operation::AddDataSource(_)
        | Operation::UpdateDataSource(_)
        | Operation::RemoveDataSource(_) => Some(Operation::AddCells(successor.clone())),
    };

    Ok(operation)
}

pub fn transform_merge_cells_operation(
    state: &dyn TransformOperationState,
    successor: &MergeCellsOperation,
    predecessor: &Operation,
) -> Result<Option<Operation>, Error> {
    let operation = match predecessor {
        Operation::AddCells(predecessor) => {
            let mut transformed = successor.clone();
            if let Some(referencing_cells) = &mut transformed.referencing_cells {
                adjust_indices_for_added_cells(referencing_cells, predecessor);
            }
            Some(Operation::MergeCells(transformed))
        }
        Operation::MergeCells(predecessor) => {
            if successor.target_cell_id == predecessor.target_cell_id {
                if successor.source_cell.id() == predecessor.source_cell.id() {
                    // Both tried to move the same source into the same target. Discard successor:
                    None
                } else {
                    // Both are trying to merge different cells into the same target. Append
                    // successor to the result of predecessor:
                    Some(Operation::MergeCells(MergeCellsOperation {
                        glue_text: successor.glue_text.clone(),
                        source_cell: successor.source_cell.clone(),
                        target_cell_id: successor.target_cell_id.clone(),
                        target_content_length: predecessor.target_content_length
                            + predecessor
                                .glue_text
                                .as_ref()
                                .map(|text| text.len())
                                .unwrap_or_default() as u32
                            + predecessor.source_cell.content().unwrap_or_default().len() as u32,
                        referencing_cells: match successor.referencing_cells.as_ref() {
                            Some(cells) => Some(with_adjusted_indices_for_merged_cells(
                                state,
                                cells,
                                predecessor,
                            )?),
                            None => None,
                        },
                    }))
                }
            } else if successor.source_cell.id() == predecessor.source_cell.id() {
                // Both tried to move the same source into different targets. Discard successor:
                None
            } else if successor.source_cell.id() == &predecessor.target_cell_id {
                // Successor tried to merge the target of predecessor. Update its source:
                let target_cell = state
                    .cell(&predecessor.target_cell_id)
                    .ok_or_else(|| Error::CellNotFound(predecessor.target_cell_id.clone()))?;
                Some(Operation::MergeCells(MergeCellsOperation {
                    glue_text: successor.glue_text.clone(),
                    source_cell: target_cell.with_appended_content(&format!(
                        "{}{}",
                        predecessor.glue_text.clone().unwrap_or_default(),
                        predecessor.source_cell.content().ok_or_else(|| {
                            Error::NoContentCell(predecessor.source_cell.id().clone())
                        })?
                    )),
                    target_cell_id: successor.target_cell_id.clone(),
                    target_content_length: successor.target_content_length,
                    referencing_cells: match successor.referencing_cells.as_ref() {
                        Some(cells) => Some(with_adjusted_indices_for_merged_cells(
                            state,
                            cells,
                            predecessor,
                        )?),
                        None => None,
                    },
                }))
            } else if &successor.target_cell_id == predecessor.source_cell.id() {
                // Successor tried to merge into the source of predecessor. Update its target:
                Some(Operation::MergeCells(MergeCellsOperation {
                    glue_text: successor.glue_text.clone(),
                    source_cell: successor.source_cell.clone(),
                    target_cell_id: predecessor.target_cell_id.clone(),
                    target_content_length: predecessor.target_content_length
                        + predecessor
                            .glue_text
                            .as_ref()
                            .map(|text| text.len())
                            .unwrap_or_default() as u32
                        + predecessor.source_cell.content().unwrap_or_default().len() as u32,
                    referencing_cells: match successor.referencing_cells.as_ref() {
                        Some(cells) => Some(with_adjusted_indices_for_merged_cells(
                            state,
                            cells,
                            predecessor,
                        )?),
                        None => None,
                    },
                }))
            } else {
                // Target and source differ, no conflict:
                Some(Operation::MergeCells(MergeCellsOperation {
                    glue_text: successor.glue_text.clone(),
                    source_cell: successor.source_cell.clone(),
                    target_cell_id: successor.target_cell_id.clone(),
                    target_content_length: successor.target_content_length,
                    referencing_cells: match successor.referencing_cells.as_ref() {
                        Some(cells) => Some(with_adjusted_indices_for_merged_cells(
                            state,
                            cells,
                            predecessor,
                        )?),
                        None => None,
                    },
                }))
            }
        }
        Operation::MoveCells(predecessor) => Some(Operation::MergeCells(MergeCellsOperation {
            glue_text: successor.glue_text.clone(),
            source_cell: successor.source_cell.clone(),
            target_cell_id: successor.target_cell_id.clone(),
            target_content_length: successor.target_content_length,
            referencing_cells: successor
                .referencing_cells
                .as_ref()
                .map(|cells| with_adjusted_indices_for_moved_cells(cells, predecessor)),
        })),
        Operation::RemoveCells(predecessor) => {
            let removed_cell_ids = get_all_removed_cell_ids(predecessor);
            if removed_cell_ids.contains(&successor.source_cell.id().as_ref()) {
                // Source was removed. No merge necessary (or possible) anymore:
                None
            } else if removed_cell_ids.contains(&successor.target_cell_id.as_ref()) {
                // Target was removed. Merging source into it now means that we simply re-assign
                // the ID of the source to become the target:
                Some(Operation::UpdateCell(UpdateCellOperation {
                    old_cell: Box::new(successor.source_cell.clone()),
                    updated_cell: Box::new(
                        successor.source_cell.with_id(&successor.target_cell_id),
                    ),
                }))
            } else {
                // No conflict:
                Some(Operation::MergeCells(MergeCellsOperation {
                    glue_text: successor.glue_text.clone(),
                    source_cell: successor.source_cell.clone(),
                    target_cell_id: successor.target_cell_id.clone(),
                    target_content_length: successor.target_content_length,
                    referencing_cells: successor
                        .referencing_cells
                        .as_ref()
                        .map(|cells| with_adjusted_indices_for_removed_cells(cells, predecessor)),
                }))
            }
        }
        Operation::SplitCell(predecessor) => {
            let referencing_cells = match successor.referencing_cells.as_ref() {
                Some(cells) => {
                    let mut cells = with_added_source_ids(
                        cells,
                        &[predecessor.new_cell.id()],
                        &predecessor.referencing_cells,
                    );
                    adjust_indices_for_split_cell(state, &mut cells, predecessor)?;
                    Some(cells)
                }
                None => None,
            };

            if successor.source_cell.id() == &predecessor.cell_id {
                // The source we're trying to merge has been split, update it:
                let source_cell = &successor.source_cell;
                Some(Operation::MergeCells(MergeCellsOperation {
                    glue_text: successor.glue_text.clone(),
                    source_cell: source_cell.with_content(
                        source_cell
                            .content()
                            .ok_or_else(|| Error::NoContentCell(source_cell.id().clone()))?
                            .get(..predecessor.split_index as usize)
                            .ok_or_else(|| {
                                Error::InvalidSplitIndex(
                                    predecessor.split_index,
                                    source_cell.id().clone(),
                                )
                            })?,
                    ),
                    target_cell_id: successor.target_cell_id.clone(),
                    target_content_length: successor.target_content_length,
                    referencing_cells,
                }))
            } else if successor.target_cell_id == predecessor.cell_id {
                // The target we're trying to merge into has been split, update it:
                Some(Operation::MergeCells(MergeCellsOperation {
                    glue_text: successor.glue_text.clone(),
                    source_cell: successor.source_cell.clone(),
                    target_cell_id: predecessor.new_cell.id().clone(),
                    target_content_length: predecessor
                        .new_cell
                        .content()
                        .ok_or_else(|| Error::NoContentCell(predecessor.new_cell.id().clone()))?
                        .len() as u32,
                    referencing_cells,
                }))
            } else {
                // No conflict:
                Some(Operation::MergeCells(MergeCellsOperation {
                    glue_text: successor.glue_text.clone(),
                    source_cell: successor.source_cell.clone(),
                    target_cell_id: successor.target_cell_id.clone(),
                    target_content_length: successor.target_content_length,
                    referencing_cells,
                }))
            }
        }
        Operation::UpdateCell(predecessor) => {
            if merge_and_update_converge(successor, predecessor) {
                Some(Operation::MergeCells(MergeCellsOperation {
                    glue_text: successor.glue_text.clone(),
                    source_cell: successor.source_cell.clone(),
                    target_cell_id: successor.target_cell_id.clone(),
                    target_content_length: successor.target_content_length,
                    referencing_cells: match successor.referencing_cells.as_ref() {
                        Some(cells) => Some(with_merged_source_ids(state, cells, predecessor)?),
                        None => None,
                    },
                }))
            } else {
                None
            }
        }
        Operation::UpdateNotebookTimeRange(_)
        | Operation::UpdateNotebookTitle(_)
        | Operation::AddDataSource(_)
        | Operation::UpdateDataSource(_)
        | Operation::RemoveDataSource(_) => Some(Operation::MergeCells(successor.clone())),
    };

    Ok(operation)
}

pub fn transform_move_cells_operation(
    state: &dyn TransformOperationState,
    successor: &MoveCellsOperation,
    predecessor: &Operation,
) -> Result<Option<Operation>, Error> {
    let operation = match predecessor {
        Operation::AddCells(predecessor) => {
            let mut transformed = MoveCellsOperation {
                cell_ids: successor.cell_ids.clone(),
                from_index: get_index_adjusted_for_added_cells(successor.from_index, predecessor),
                to_index: get_index_adjusted_for_added_cells(successor.to_index, predecessor),
            };

            // If cells were added inside the range we are moving, then move the newly added
            // cell(s) along:
            for predecessor_cell in &predecessor.cells {
                if predecessor_cell.index > transformed.from_index {
                    let index_in_range = (predecessor_cell.index - transformed.from_index) as usize;
                    if index_in_range < transformed.cell_ids.len() {
                        transformed.cell_ids.splice(
                            index_in_range..index_in_range,
                            vec![predecessor_cell.cell.id().clone()],
                        );
                    }
                }
            }

            Some(Operation::MoveCells(transformed))
        }
        Operation::MergeCells(predecessor) => {
            let mut cell_ids = successor.cell_ids.clone();

            // If (one of) the cells we are trying to move was merged into another, remove it:
            cell_ids.retain(|id| id != predecessor.source_cell.id());

            // Adjust indices:
            let source_cell_index = get_source_cell_index(state, predecessor)?;
            let from_index = get_index_adjusted_for_merged_cells(
                successor.from_index,
                predecessor,
                source_cell_index,
            );
            let to_index = get_index_adjusted_for_merged_cells(
                successor.to_index,
                predecessor,
                source_cell_index,
            );

            if cell_ids.is_empty() || from_index == to_index {
                None
            } else {
                Some(Operation::MoveCells(MoveCellsOperation {
                    cell_ids,
                    from_index,
                    to_index,
                }))
            }
        }
        Operation::MoveCells(predecessor) => {
            if moves_converge(successor, predecessor) {
                Some(Operation::MoveCells(MoveCellsOperation {
                    cell_ids: successor.cell_ids.clone(),
                    from_index: get_index_adjusted_for_moved_cells(
                        successor.from_index,
                        predecessor,
                    ),
                    to_index: get_index_adjusted_for_moved_cells(successor.to_index, predecessor),
                }))
            } else {
                // Discard successor in case of non-convergence:
                None
            }
        }
        Operation::RemoveCells(predecessor) => {
            let mut cell_ids = successor.cell_ids.clone();

            // Don't move cells that have already been removed:
            let removed_cell_ids = get_all_removed_cell_ids(predecessor);
            cell_ids.retain(|id| !removed_cell_ids.contains(&id.as_ref()));

            // Adjust the indices to compensate for the removed cells:
            let from_index =
                get_index_adjusted_for_removed_cells(successor.from_index, predecessor);
            let to_index = if from_index < successor.to_index {
                let num_cells_to_move = cell_ids.len() as u32;
                get_index_adjusted_for_removed_cells(
                    successor.to_index + num_cells_to_move,
                    predecessor,
                ) - num_cells_to_move
            } else {
                get_index_adjusted_for_removed_cells(successor.to_index, predecessor)
            };

            if cell_ids.is_empty() || from_index == to_index {
                None
            } else {
                Some(Operation::MoveCells(MoveCellsOperation {
                    cell_ids,
                    from_index,
                    to_index,
                }))
            }
        }
        Operation::SplitCell(predecessor) => {
            let split_cell_index = state
                .cell_index(&predecessor.cell_id)
                .ok_or_else(|| Error::CellNotFound(predecessor.cell_id.clone()))?;
            let newly_added_referencing_cells =
                &get_newly_added_referencing_cells(state, &predecessor.referencing_cells);

            let mut transformed = MoveCellsOperation {
                cell_ids: successor.cell_ids.clone(),
                from_index: get_index_adjusted_for_split_cell(
                    successor.from_index,
                    split_cell_index,
                    newly_added_referencing_cells,
                ),
                to_index: get_index_adjusted_for_split_cell(
                    successor.to_index,
                    split_cell_index,
                    newly_added_referencing_cells,
                ),
            };

            if split_cell_index > transformed.from_index {
                let index_in_range = (split_cell_index - transformed.from_index) as usize;
                if index_in_range < transformed.cell_ids.len() {
                    // Split cell was inside the range we are moving, so move it along:
                    transformed.cell_ids.splice(
                        index_in_range + 1..index_in_range + 1,
                        vec![predecessor.new_cell.id().clone()],
                    );
                } else if split_cell_index <= transformed.to_index {
                    // Split cell was inside the cells we moved across, so adjust the destination:
                    transformed.to_index += 1;
                }
            }
            // Same for newly added referencing cells that were added inside the range:
            for cell in newly_added_referencing_cells {
                if cell.index > transformed.from_index {
                    let index_in_range = (cell.index - transformed.from_index) as usize;
                    if index_in_range < transformed.cell_ids.len() {
                        transformed
                            .cell_ids
                            .splice(index_in_range..index_in_range, vec![cell.cell.id().clone()]);
                    } else if cell.index <= transformed.to_index {
                        // New cell was inside the cells we moved across, so adjust the destination:
                        transformed.to_index += 1;
                    }
                }
            }

            Some(Operation::MoveCells(transformed))
        }
        Operation::UpdateCell(predecessor) => {
            if predecessor.old_cell.id() == predecessor.updated_cell.id()
                || !successor.cell_ids.contains(predecessor.old_cell.id())
            {
                Some(Operation::MoveCells(successor.clone()))
            } else {
                // Replace the swapped cell ID in the array of moved cells:
                let mut cell_ids = successor.cell_ids.clone();
                let index = cell_ids
                    .iter()
                    .position(|id| id == predecessor.old_cell.id())
                    .unwrap();
                cell_ids[index] = predecessor.updated_cell.id().clone();
                Some(Operation::MoveCells(MoveCellsOperation {
                    cell_ids,
                    from_index: successor.from_index,
                    to_index: successor.to_index,
                }))
            }
        }
        Operation::UpdateNotebookTimeRange(_)
        | Operation::UpdateNotebookTitle(_)
        | Operation::AddDataSource(_)
        | Operation::UpdateDataSource(_)
        | Operation::RemoveDataSource(_) => Some(Operation::MoveCells(successor.clone())),
    };

    Ok(operation)
}

pub fn transform_remove_cells_operation(
    state: &dyn TransformOperationState,
    successor: &RemoveCellsOperation,
    predecessor: &Operation,
) -> Result<Option<Operation>, Error> {
    let operation = match predecessor {
        Operation::AddCells(predecessor) => {
            // Adjust the indices for the added cells:
            let mut transformed = successor.clone();
            adjust_indices_for_added_cells(&mut transformed.removed_cells, predecessor);
            if let Some(referencing_cells) = &mut transformed.referencing_cells {
                adjust_indices_for_added_cells(referencing_cells, predecessor);
            }
            Some(Operation::RemoveCells(transformed))
        }
        Operation::MergeCells(predecessor) => {
            if successor
                .removed_cells
                .iter()
                .any(|c| c.cell.id() == predecessor.source_cell.id())
            {
                // One of the cells we want to remove was merged into another:
                if successor.removed_cells.len() == 1 {
                    // It was the only cell we wanted to remove, so we remove the content from the
                    // cell it was merged into:
                    let target_cell = state
                        .cell(&predecessor.target_cell_id)
                        .ok_or_else(|| Error::CellNotFound(predecessor.target_cell_id.clone()))?;
                    Some(Operation::UpdateCell(UpdateCellOperation {
                        old_cell: Box::new(target_cell.with_appended_content(
                            predecessor.source_cell.content().ok_or_else(|| {
                                Error::NoContentCell(predecessor.source_cell.id().clone())
                            })?,
                        )),
                        updated_cell: Box::new(target_cell.clone()),
                    }))
                } else if successor
                    .removed_cells
                    .iter()
                    .any(|c| c.cell.id() == &predecessor.target_cell_id)
                {
                    // The target was removed too, so we simply remove the source from the list of
                    // cells to remove:
                    Some(Operation::RemoveCells(RemoveCellsOperation {
                        removed_cells: without_removed_cell(
                            &successor.removed_cells,
                            predecessor.source_cell.id(),
                        ),
                        referencing_cells: match successor.referencing_cells.as_ref() {
                            Some(cells) => Some(with_adjusted_indices_for_merged_cells(
                                state,
                                cells,
                                predecessor,
                            )?),
                            None => None,
                        },
                    }))
                } else {
                    // FIXME: For proper convergence, we should split this into two operations:
                    //        one for the removal of the remaining cells, and one for updating the
                    //        target. For now, we just drop the remove operation...
                    None
                }
            } else if successor
                .removed_cells
                .iter()
                .any(|c| c.cell.id() == &predecessor.target_cell_id)
            {
                // We're trying to remove the target of a merge:
                if successor.removed_cells.len() == 1 {
                    // It was the only cell we wanted to remove, so we remove the content from the
                    // cell it was merged into:
                    let target_cell = state
                        .cell(&predecessor.target_cell_id)
                        .ok_or_else(|| Error::CellNotFound(predecessor.target_cell_id.clone()))?;
                    let source_cell_content =
                        predecessor.source_cell.content().ok_or_else(|| {
                            Error::NoContentCell(predecessor.source_cell.id().clone())
                        })?;
                    Some(Operation::UpdateCell(UpdateCellOperation {
                        old_cell: Box::new(target_cell.with_appended_content(source_cell_content)),
                        updated_cell: Box::new(
                            predecessor.source_cell.with_id(&predecessor.target_cell_id),
                        ),
                    }))
                } else {
                    // FIXME: For proper convergence, we should split this into two operations:
                    //        one for the removal of the remaining cells, and one for updating the
                    //        target. For now, we just drop the remove operation...
                    None
                }
            } else {
                // No conflict:
                Some(Operation::RemoveCells(RemoveCellsOperation {
                    removed_cells: successor.removed_cells.clone(),
                    referencing_cells: match successor.referencing_cells.as_ref() {
                        Some(cells) => Some(with_adjusted_indices_for_merged_cells(
                            state,
                            cells,
                            predecessor,
                        )?),
                        None => None,
                    },
                }))
            }
        }
        Operation::MoveCells(predecessor) => {
            // Adjust the indices for the moved cells:
            let mut transformed = successor.clone();
            adjust_indices_for_moved_cells(&mut transformed.removed_cells, predecessor);
            if let Some(referencing_cells) = &mut transformed.referencing_cells {
                adjust_indices_for_moved_cells(referencing_cells, predecessor);
            }
            Some(Operation::RemoveCells(transformed))
        }
        Operation::RemoveCells(predecessor) => {
            let previously_removed_cell_ids = get_all_removed_cell_ids(predecessor);
            let mut removed_cells = successor.removed_cells.clone();
            removed_cells.retain(|c| !previously_removed_cell_ids.contains(&c.cell.id().as_ref()));

            adjust_indices_for_removed_cells(&mut removed_cells, predecessor);

            if removed_cells.is_empty() {
                // All cells already removed:
                None
            } else if let Some(mut referencing_cells) = successor.referencing_cells.clone() {
                // Update referencing cells as well:
                referencing_cells
                    .retain(|c| !previously_removed_cell_ids.contains(&c.cell.id().as_ref()));

                // Remove the removed source IDs:
                referencing_cells = referencing_cells
                    .iter()
                    .map(|c| {
                        let source_ids = c
                            .cell
                            .source_ids()
                            .into_iter()
                            .filter_map(|id| {
                                if previously_removed_cell_ids.contains(&id) {
                                    None
                                } else {
                                    Some(id.to_owned())
                                }
                            })
                            .collect();
                        CellWithIndex {
                            cell: c.cell.with_source_ids(source_ids),
                            index: c.index,
                        }
                    })
                    .collect();

                adjust_indices_for_removed_cells(&mut referencing_cells, predecessor);

                Some(Operation::RemoveCells(RemoveCellsOperation {
                    removed_cells,
                    referencing_cells: Some(referencing_cells),
                }))
            } else {
                // Just keep the remaining cells:
                Some(Operation::RemoveCells(RemoveCellsOperation {
                    removed_cells,
                    referencing_cells: None,
                }))
            }
        }
        Operation::SplitCell(predecessor) => {
            let removed_cell_ids = get_all_removed_cell_ids(successor);
            let referencing_cells = match successor.referencing_cells.as_ref() {
                Some(cells) => {
                    let mut cells = with_added_source_ids(
                        cells,
                        &[predecessor.new_cell.id()],
                        &predecessor.referencing_cells,
                    );
                    adjust_indices_for_split_cell(state, &mut cells, predecessor)?;
                    Some(cells)
                }
                None => None,
            };

            let mut removed_cells =
                with_adjusted_indices_for_split_cell(state, &successor.removed_cells, predecessor)?;
            if removed_cell_ids.contains(&predecessor.cell_id.as_ref()) {
                // One of the cells that we're trying to remove has been split.
                // Remove the split-off cell too:
                let split_cell_index = removed_cells
                    .iter()
                    .find(|c| c.cell.id() == &predecessor.cell_id)
                    .unwrap()
                    .index;
                removed_cells.push(CellWithIndex {
                    cell: predecessor.new_cell.clone(),
                    index: split_cell_index + 1,
                });
            }

            Some(Operation::RemoveCells(RemoveCellsOperation {
                removed_cells,
                referencing_cells,
            }))
        }
        Operation::UpdateCell(predecessor) => {
            let mut removed_cells = successor.removed_cells.clone();

            if predecessor.old_cell.id() != predecessor.updated_cell.id() {
                if let Some(index) = removed_cells
                    .iter()
                    .position(|c| c.cell.id() == predecessor.old_cell.id())
                {
                    removed_cells[index].cell = predecessor.updated_cell.as_ref().clone();
                }
            }

            Some(Operation::RemoveCells(RemoveCellsOperation {
                removed_cells,
                referencing_cells: match successor.referencing_cells.as_ref() {
                    Some(cells) => Some(with_merged_source_ids(state, cells, predecessor)?),
                    None => None,
                },
            }))
        }
        Operation::UpdateNotebookTimeRange(_)
        | Operation::UpdateNotebookTitle(_)
        | Operation::AddDataSource(_)
        | Operation::UpdateDataSource(_)
        | Operation::RemoveDataSource(_) => Some(Operation::RemoveCells(successor.clone())),
    };

    Ok(operation)
}

pub fn transform_split_cell_operation(
    state: &dyn TransformOperationState,
    successor: &SplitCellOperation,
    predecessor: &Operation,
) -> Result<Option<Operation>, Error> {
    let operation = match predecessor {
        Operation::AddCells(predecessor) => {
            // Adjust the indices for the added cells:
            let mut transformed = successor.clone();
            if let Some(referencing_cells) = &mut transformed.referencing_cells {
                adjust_indices_for_added_cells(referencing_cells, predecessor);
            }
            Some(Operation::SplitCell(transformed))
        }
        Operation::MergeCells(predecessor) => {
            if &successor.cell_id == predecessor.source_cell.id() {
                // The cell we're trying to split has been merged into another. Update it:
                Some(Operation::SplitCell(SplitCellOperation {
                    cell_id: predecessor.target_cell_id.clone(),
                    new_cell: successor.new_cell.clone(),
                    removed_text: successor.removed_text.clone(),
                    split_index: successor.split_index
                        + predecessor
                            .glue_text
                            .as_ref()
                            .map(|text| text.len())
                            .unwrap_or_default() as u32
                        + predecessor.target_content_length,
                    referencing_cells: match successor.referencing_cells.as_ref() {
                        Some(cells) => {
                            let mut cells =
                                without_source_ids(cells, &[successor.cell_id.as_ref()]);
                            adjust_indices_for_merged_cells(state, &mut cells, predecessor)?;
                            Some(cells)
                        }
                        None => None,
                    },
                }))
            } else if successor.cell_id == predecessor.target_cell_id {
                // The cell we're trying to split has had another cell merged into it. Update it:
                Some(Operation::SplitCell(SplitCellOperation {
                    cell_id: successor.cell_id.clone(),
                    new_cell: successor.new_cell.with_appended_content(&format!(
                        "{}{}",
                        predecessor
                            .glue_text
                            .as_ref()
                            .map(|text| text.as_ref())
                            .unwrap_or(""),
                        predecessor.source_cell.content().ok_or_else(|| {
                            Error::NoContentCell(predecessor.source_cell.id().clone())
                        })?,
                    )),
                    removed_text: successor.removed_text.clone(),
                    split_index: successor.split_index,
                    referencing_cells: match successor.referencing_cells.as_ref() {
                        Some(cells) => {
                            let mut cells =
                                without_source_ids(cells, &[successor.cell_id.as_ref()]);
                            adjust_indices_for_merged_cells(state, &mut cells, predecessor)?;
                            Some(cells)
                        }
                        None => None,
                    },
                }))
            } else {
                // No conflicts:
                let mut transformed = successor.clone();
                if let Some(referencing_cells) = &mut transformed.referencing_cells {
                    adjust_indices_for_merged_cells(state, referencing_cells, predecessor)?;
                }
                Some(Operation::SplitCell(transformed))
            }
        }
        Operation::MoveCells(predecessor) => {
            // Adjust the indices for the moved cells:
            let mut transformed = successor.clone();
            if let Some(referencing_cells) = &mut transformed.referencing_cells {
                adjust_indices_for_moved_cells(referencing_cells, predecessor);
            }
            Some(Operation::SplitCell(transformed))
        }
        Operation::RemoveCells(predecessor) => {
            let removed_cell_ids = get_all_removed_cell_ids(predecessor);
            if removed_cell_ids.contains(&successor.cell_id.as_ref()) {
                // The cell we wanted to split has been removed. Drop the split:
                None
            } else {
                // No conflict:
                let mut transformed = successor.clone();
                if let Some(referencing_cells) = &mut transformed.referencing_cells {
                    adjust_indices_for_removed_cells(referencing_cells, predecessor);
                }
                Some(Operation::SplitCell(transformed))
            }
        }
        Operation::SplitCell(predecessor) => {
            if !splits_converge(successor, predecessor) {
                // Splits cannot converge. Drop successor:
                None
            } else if successor.cell_id == predecessor.cell_id {
                let referencing_cells = match successor.referencing_cells.as_ref() {
                    Some(cells) => {
                        let mut cells = with_added_source_ids(
                            cells,
                            &[predecessor.new_cell.id()],
                            &predecessor.referencing_cells,
                        );
                        adjust_indices_for_split_cell(state, &mut cells, predecessor)?;
                        Some(cells)
                    }
                    None => None,
                };

                // The cell we're trying to split has already been split.
                // Update it depending on where the split was:
                match successor.split_index.cmp(&predecessor.split_index) {
                    Ordering::Less => Some(Operation::SplitCell(SplitCellOperation {
                        cell_id: successor.cell_id.clone(),
                        new_cell: get_new_cell_split_by_predecessor(successor, predecessor)?,
                        removed_text: successor.removed_text.clone(),
                        split_index: successor.split_index,
                        referencing_cells,
                    })),
                    Ordering::Equal => None,
                    Ordering::Greater => Some(Operation::SplitCell(SplitCellOperation {
                        cell_id: predecessor.new_cell.id().clone(),
                        new_cell: successor.new_cell.clone(),
                        removed_text: successor.removed_text.clone(),
                        split_index: successor.split_index
                            - predecessor.split_index
                            - predecessor
                                .removed_text
                                .as_ref()
                                .map(|text| text.len() as u32)
                                .unwrap_or_default(),
                        referencing_cells,
                    })),
                }
            } else {
                // No conflict:
                let mut transformed = successor.clone();
                if let Some(referencing_cells) = &mut transformed.referencing_cells {
                    adjust_indices_for_split_cell(state, referencing_cells, predecessor)?;
                }
                Some(Operation::SplitCell(transformed))
            }
        }
        Operation::UpdateCell(predecessor) => {
            if predecessor.old_cell.id() == &successor.cell_id {
                // Cell we tried to split has already been updated. Cannot converge:
                None
            } else {
                Some(Operation::SplitCell(SplitCellOperation {
                    cell_id: successor.cell_id.clone(),
                    new_cell: successor.new_cell.clone(),
                    removed_text: successor.removed_text.clone(),
                    split_index: successor.split_index,
                    referencing_cells: match successor.referencing_cells.as_ref() {
                        Some(cells) => Some(with_merged_source_ids(state, cells, predecessor)?),
                        None => None,
                    },
                }))
            }
        }
        Operation::UpdateNotebookTimeRange(_)
        | Operation::UpdateNotebookTitle(_)
        | Operation::AddDataSource(_)
        | Operation::UpdateDataSource(_)
        | Operation::RemoveDataSource(_) => Some(Operation::SplitCell(successor.clone())),
    };

    Ok(operation)
}

pub fn transform_update_cell_operation(
    _: &dyn TransformOperationState,
    successor: &UpdateCellOperation,
    predecessor: &Operation,
) -> Result<Option<Operation>, Error> {
    let operation = match predecessor {
        Operation::AddCells(predecessor) => Some(Operation::UpdateCell(UpdateCellOperation {
            old_cell: get_updated_old_cell(successor, &predecessor.referencing_cells),
            updated_cell: successor.updated_cell.clone(),
        })),
        Operation::MergeCells(predecessor) => {
            if merge_and_update_converge(predecessor, successor) {
                Some(Operation::UpdateCell(UpdateCellOperation {
                    old_cell: get_updated_old_cell(successor, &predecessor.referencing_cells),
                    updated_cell: successor.updated_cell.clone(),
                }))
            } else {
                None
            }
        }
        Operation::MoveCells(_) => Some(Operation::UpdateCell(successor.clone())),
        Operation::RemoveCells(predecessor) => {
            let removed_cell_ids = get_all_removed_cell_ids(predecessor);
            if removed_cell_ids.contains(&successor.old_cell.id().as_ref()) {
                // The cell we wanted to update has been removed. Drop successor:
                None
            } else {
                // No conflict:
                Some(Operation::UpdateCell(UpdateCellOperation {
                    old_cell: get_updated_old_cell(successor, &predecessor.referencing_cells),
                    updated_cell: successor.updated_cell.clone(),
                }))
            }
        }
        Operation::SplitCell(predecessor) => {
            if &predecessor.cell_id == successor.old_cell.id() {
                // The cell we wanted to update has been split. Drop successor:
                None
            } else {
                // No conflict:
                Some(Operation::UpdateCell(UpdateCellOperation {
                    old_cell: get_updated_old_cell(successor, &predecessor.referencing_cells),
                    updated_cell: successor.updated_cell.clone(),
                }))
            }
        }
        Operation::UpdateCell(predecessor) => {
            if predecessor.old_cell.id() == successor.old_cell.id() {
                // Cell we tried to update has already been updated. Drop successor:
                None
            } else {
                Some(Operation::UpdateCell(successor.clone()))
            }
        }
        Operation::UpdateNotebookTimeRange(_)
        | Operation::UpdateNotebookTitle(_)
        | Operation::AddDataSource(_)
        | Operation::UpdateDataSource(_)
        | Operation::RemoveDataSource(_) => Some(Operation::UpdateCell(successor.clone())),
    };

    Ok(operation)
}

pub fn transform_update_notebook_time_range_operation(
    _: &dyn TransformOperationState,
    successor: &UpdateNotebookTimeRangeOperation,
    predecessor: &Operation,
) -> Option<Operation> {
    if matches!(predecessor, Operation::UpdateNotebookTimeRange(_)) {
        // Cannot converge. Drop successor:
        None
    } else {
        Some(Operation::UpdateNotebookTimeRange(successor.clone()))
    }
}

pub fn transform_update_notebook_title_operation(
    _: &dyn TransformOperationState,
    successor: &UpdateNotebookTitleOperation,
    predecessor: &Operation,
) -> Option<Operation> {
    if matches!(predecessor, Operation::UpdateNotebookTitle(_)) {
        // Cannot converge. Drop successor:
        None
    } else {
        Some(Operation::UpdateNotebookTitle(successor.clone()))
    }
}

pub fn transform_add_data_source_operation(
    _: &dyn TransformOperationState,
    successor: &AddDataSourceOperation,
    predecessor: &Operation,
) -> Option<Operation> {
    // Only allow it if the predecessor was not adding a data-source with the
    // same name (updating or removing data-sources with the same name should
    // not be possible).
    match predecessor {
        Operation::AddDataSource(predecessor) => {
            if predecessor.name == successor.name {
                None
            } else {
                Some(Operation::AddDataSource(successor.clone()))
            }
        }
        Operation::AddCells(_)
        | Operation::MergeCells(_)
        | Operation::MoveCells(_)
        | Operation::RemoveCells(_)
        | Operation::SplitCell(_)
        | Operation::UpdateCell(_)
        | Operation::UpdateNotebookTimeRange(_)
        | Operation::UpdateNotebookTitle(_)
        | Operation::UpdateDataSource(_)
        | Operation::RemoveDataSource(_) => Some(Operation::AddDataSource(successor.clone())),
    }
}

pub fn transform_update_data_source_operation(
    _: &dyn TransformOperationState,
    successor: &UpdateDataSourceOperation,
    predecessor: &Operation,
) -> Option<Operation> {
    // Only allow it if the predecessor was not updating or removing the same data-source
    match predecessor {
        Operation::UpdateDataSource(predecessor) => {
            if predecessor.name == successor.name {
                None
            } else {
                Some(Operation::UpdateDataSource(successor.clone()))
            }
        }
        Operation::RemoveDataSource(predecessor) => {
            if predecessor.name == successor.name {
                None
            } else {
                Some(Operation::UpdateDataSource(successor.clone()))
            }
        }
        Operation::AddCells(_)
        | Operation::MergeCells(_)
        | Operation::MoveCells(_)
        | Operation::RemoveCells(_)
        | Operation::SplitCell(_)
        | Operation::UpdateCell(_)
        | Operation::UpdateNotebookTimeRange(_)
        | Operation::UpdateNotebookTitle(_)
        | Operation::AddDataSource(_) => Some(Operation::UpdateDataSource(successor.clone())),
    }
}

pub fn transform_remove_data_source_operation(
    _: &dyn TransformOperationState,
    successor: &RemoveDataSourceOperation,
    predecessor: &Operation,
) -> Option<Operation> {
    // Only allow it if the predecessor has not removed the same data-source
    match predecessor {
        Operation::RemoveDataSource(predecessor) => {
            if predecessor.name == successor.name {
                None
            } else {
                Some(Operation::RemoveDataSource(successor.clone()))
            }
        }
        Operation::UpdateDataSource(_)
        | Operation::AddCells(_)
        | Operation::MergeCells(_)
        | Operation::MoveCells(_)
        | Operation::RemoveCells(_)
        | Operation::SplitCell(_)
        | Operation::UpdateCell(_)
        | Operation::UpdateNotebookTimeRange(_)
        | Operation::UpdateNotebookTitle(_)
        | Operation::AddDataSource(_) => Some(Operation::RemoveDataSource(successor.clone())),
    }
}

fn adjust_indices_for_added_cells(cells: &mut Vec<CellWithIndex>, predecessor: &AddCellsOperation) {
    for cell in cells {
        cell.index = get_index_adjusted_for_added_cells(cell.index, predecessor);
    }
}

fn adjust_indices_for_merged_cells(
    state: &dyn TransformOperationState,
    cells: &mut Vec<CellWithIndex>,
    predecessor: &MergeCellsOperation,
) -> Result<(), Error> {
    let source_cell_index = get_source_cell_index(state, predecessor)?;
    for cell in cells {
        cell.index =
            get_index_adjusted_for_merged_cells(cell.index, predecessor, source_cell_index);
    }
    Ok(())
}

fn adjust_indices_for_moved_cells(
    cells: &mut Vec<CellWithIndex>,
    predecessor: &MoveCellsOperation,
) {
    for cell in cells {
        cell.index = get_index_adjusted_for_moved_cells(cell.index, predecessor);
    }
}

fn adjust_indices_for_removed_cells(
    cells: &mut Vec<CellWithIndex>,
    predecessor: &RemoveCellsOperation,
) {
    for cell in cells {
        cell.index = get_index_adjusted_for_removed_cells(cell.index, predecessor);
    }
}

fn adjust_indices_for_split_cell(
    state: &dyn TransformOperationState,
    cells: &mut Vec<CellWithIndex>,
    predecessor: &SplitCellOperation,
) -> Result<(), Error> {
    let split_cell_index = state
        .cell_index(predecessor.cell_id.as_ref())
        .ok_or_else(|| Error::CellNotFound(predecessor.cell_id.clone()))?;
    let newly_added_referencing_cells =
        get_newly_added_referencing_cells(state, &predecessor.referencing_cells);

    for cell in cells {
        cell.index = get_index_adjusted_for_split_cell(
            cell.index,
            split_cell_index,
            &newly_added_referencing_cells,
        );
    }
    Ok(())
}

fn get_all_removed_cell_ids(operation: &RemoveCellsOperation) -> Vec<&str> {
    let mut removed_cell_ids: Vec<&str> = get_removed_cell_ids(operation);
    if let Some(referencing_cells) = &operation.referencing_cells {
        for referencing_cell in referencing_cells {
            if is_subset(&referencing_cell.cell.source_ids(), &removed_cell_ids) {
                removed_cell_ids.push(referencing_cell.cell.id().as_ref());
            }
        }
    }
    removed_cell_ids
}

fn get_index_adjusted_for_added_cells(index: u32, predecessor: &AddCellsOperation) -> u32 {
    let mut add = 0;
    for added_cell in &predecessor.cells {
        if added_cell.index <= index {
            add += 1;
        }
    }
    index + add
}

fn get_index_adjusted_for_merged_cells(
    index: u32,
    predecessor: &MergeCellsOperation,
    source_cell_index: u32,
) -> u32 {
    let mut subtract = 0;
    if index >= source_cell_index {
        subtract += 1;
    }
    if let Some(referencing_cells) = &predecessor.referencing_cells {
        for referencing_cell in referencing_cells {
            let source_ids = referencing_cell.cell.source_ids();
            if source_ids.len() == 1
                && source_ids[0] == predecessor.source_cell.id()
                && referencing_cell.index < index
            {
                subtract += 1;
            }
        }
    }
    index - subtract
}

fn get_index_adjusted_for_moved_cells(index: u32, predecessor: &MoveCellsOperation) -> u32 {
    if predecessor.from_index < index {
        if predecessor.from_index + predecessor.cell_ids.len() as u32 > index {
            // Our index was moved as part of a range, so we move it along:
            (index as i32 + predecessor.to_index as i32 - predecessor.from_index as i32) as u32
        } else if predecessor.to_index >= index {
            // Cells were moved from somewhere above our index to somewhere below,
            // so we move up to compensate:
            index - predecessor.cell_ids.len() as u32
        } else {
            index
        }
    } else if predecessor.to_index < index {
        // Cells were moved from somewhere below our index to somewhere above,
        // so we move down to compensate:
        index + predecessor.cell_ids.len() as u32
    } else {
        index
    }
}

fn get_index_adjusted_for_removed_cells(index: u32, predecessor: &RemoveCellsOperation) -> u32 {
    let mut subtract = 0;
    for previously_removed_cell in &predecessor.removed_cells {
        if previously_removed_cell.index < index {
            subtract += 1;
        }
    }
    if let Some(referencing_cells) = &predecessor.referencing_cells {
        let removed_cell_ids: Vec<&str> = predecessor
            .removed_cells
            .iter()
            .map(|c| c.cell.id().as_ref())
            .collect();
        for referencing_cell in referencing_cells {
            if is_subset(&referencing_cell.cell.source_ids(), &removed_cell_ids)
                && referencing_cell.index < index
            {
                subtract += 1;
            }
        }
    }
    index - subtract
}

fn get_index_adjusted_for_split_cell(
    index: u32,
    split_cell_index: u32,
    newly_added_referencing_cells: &[&CellWithIndex],
) -> u32 {
    let mut add = 0;
    if index > split_cell_index {
        add += 1;
    }
    for cell in newly_added_referencing_cells {
        if index >= cell.index {
            add += 1;
        }
    }
    index + add
}

fn get_move_min_index(operation: &MoveCellsOperation) -> u32 {
    std::cmp::min(operation.from_index, operation.to_index)
}

fn get_move_max_index(operation: &MoveCellsOperation) -> u32 {
    std::cmp::max(operation.from_index, operation.to_index)
}

fn get_new_cell_split_by_predecessor(
    successor: &SplitCellOperation,
    predecessor: &SplitCellOperation,
) -> Result<Cell, Error> {
    let removed_text_len = successor
        .removed_text
        .as_ref()
        .map(|text| text.len())
        .unwrap_or_default() as u32;
    let split_index = predecessor.split_index - successor.split_index - removed_text_len;

    Ok(successor.new_cell.with_content(
        successor
            .new_cell
            .content()
            .ok_or_else(|| Error::NoContentCell(successor.new_cell.id().clone()))?
            .get(..split_index as usize)
            .ok_or_else(|| {
                Error::InvalidSplitIndex(split_index, successor.new_cell.id().clone())
            })?,
    ))
}

fn get_newly_added_referencing_cells<'a>(
    state: &'_ dyn TransformOperationState,
    referencing_cells: &'a Option<Vec<CellWithIndex>>,
) -> Vec<&'a CellWithIndex> {
    match referencing_cells {
        Some(referencing_cells) => referencing_cells
            .iter()
            .filter(|c| state.cell_index(c.cell.id()).is_none())
            .collect(),
        None => vec![],
    }
}

fn get_removed_cell_ids(operation: &RemoveCellsOperation) -> Vec<&str> {
    operation
        .removed_cells
        .iter()
        .map(|c| c.cell.id().as_ref())
        .collect()
}

fn get_source_cell_index(
    state: &dyn TransformOperationState,
    predecessor: &MergeCellsOperation,
) -> Result<u32, Error> {
    state
        .cell_index(predecessor.source_cell.id())
        .ok_or_else(|| Error::CellNotFound(predecessor.source_cell.id().clone()))
}

/// Determines the changes to the source IDs in predecessor cell respective to the original cell
/// that is taken from the given notebook, then applies those changes to the source IDs in the
/// successor and returns them.
fn get_merged_source_ids(
    state: &dyn TransformOperationState,
    predecessor_cell: &Cell,
    successor_cell: &Cell,
) -> Result<Vec<String>, Error> {
    let original_cell = state
        .cell(predecessor_cell.id())
        .ok_or_else(|| Error::CellNotFound(predecessor_cell.id().clone()))?;

    let original_source_ids = original_cell.source_ids();
    let predecessor_source_ids = predecessor_cell.source_ids();

    let mut added_source_ids = predecessor_source_ids.clone();
    added_source_ids.retain(|id| !original_source_ids.contains(id));

    let mut removed_source_ids = original_source_ids;
    removed_source_ids.retain(|id| !predecessor_source_ids.contains(id));

    let mut resulting_source_ids = successor_cell.source_ids();
    resulting_source_ids.retain(|id| !removed_source_ids.contains(id));
    resulting_source_ids.append(&mut added_source_ids);
    Ok(resulting_source_ids.into_iter().map(String::from).collect())
}

fn get_updated_old_cell(
    successor: &UpdateCellOperation,
    referencing_cells: &Option<Vec<CellWithIndex>>,
) -> Box<Cell> {
    Box::new(
        referencing_cells
            .as_ref()
            .and_then(|cells| {
                cells
                    .iter()
                    .find(|c| c.cell.id() == successor.old_cell.id())
            })
            .map(|c| &c.cell)
            .unwrap_or(&successor.old_cell)
            .clone(),
    )
}

fn is_subset(subset: &[&str], superset: &[&str]) -> bool {
    subset.iter().all(|item| superset.contains(item))
}

pub fn merge_and_update_converge(
    merge: &MergeCellsOperation,
    update: &UpdateCellOperation,
) -> bool {
    update.old_cell.id() != &merge.target_cell_id
        && update.old_cell.id() != merge.source_cell.id()
        && update.updated_cell.id() != &merge.target_cell_id
        && update.updated_cell.id() != merge.source_cell.id()
}

pub fn moves_converge(move1: &MoveCellsOperation, move2: &MoveCellsOperation) -> bool {
    // Moves (currently) don't converge when they try to move the same cells:
    if move1.cell_ids.iter().any(|id| move2.cell_ids.contains(id)) {
        return false;
    }

    // Moves (currently) only converge when their ranges don't overlap:
    get_move_max_index(move1) < get_move_min_index(move2)
        || get_move_min_index(move1) > get_move_max_index(move2)
}

pub fn splits_converge(split1: &SplitCellOperation, split2: &SplitCellOperation) -> bool {
    // Splits on different cells have no trouble converging:
    if split1.cell_id != split2.cell_id {
        return true;
    }

    // Splits on the same cell only converge if the removed text of one
    // doesn't cross the split index of the other:
    let index1 = split1.split_index;
    let index2 = split2.split_index;
    match index1.cmp(&index2) {
        Ordering::Less => {
            split1
                .removed_text
                .as_ref()
                .map(|text| text.len() as u32)
                .unwrap_or_default()
                < index2 - index1
        }
        Ordering::Equal => split1.removed_text == split2.removed_text,
        Ordering::Greater => {
            split2
                .removed_text
                .as_ref()
                .map(|text| text.len() as u32)
                .unwrap_or_default()
                < index1 - index2
        }
    }
}

/// Returns new cells with their source IDs expanded to include the added source IDs, but only
/// the source IDs of the cells that are listed as affected are expanded.
fn with_added_source_ids(
    cells: &[CellWithIndex],
    added_source_ids: &[&str],
    affected_cells: &Option<Vec<CellWithIndex>>,
) -> Vec<CellWithIndex> {
    cells
        .iter()
        .map(|c| {
            let is_affected = affected_cells
                .as_ref()
                .map(|affected_cells| affected_cells.iter().any(|ac| ac.cell.id() == c.cell.id()))
                .unwrap_or(false);

            if is_affected {
                let mut source_ids: Vec<String> =
                    c.cell.source_ids().into_iter().map(String::from).collect();
                for &added_id in added_source_ids {
                    if !source_ids.iter().any(|id| id == added_id) {
                        source_ids.push(added_id.to_owned());
                    }
                }
                CellWithIndex {
                    cell: c.cell.with_source_ids(source_ids),
                    index: c.index,
                }
            } else {
                c.clone()
            }
        })
        .collect()
}

fn with_adjusted_indices_for_merged_cells(
    state: &dyn TransformOperationState,
    cells: &[CellWithIndex],
    predecessor: &MergeCellsOperation,
) -> Result<Vec<CellWithIndex>, Error> {
    let mut cells = cells.to_vec();
    adjust_indices_for_merged_cells(state, &mut cells, predecessor)?;
    Ok(cells)
}

fn with_adjusted_indices_for_moved_cells(
    cells: &[CellWithIndex],
    predecessor: &MoveCellsOperation,
) -> Vec<CellWithIndex> {
    let mut cells = cells.to_vec();
    adjust_indices_for_moved_cells(&mut cells, predecessor);
    cells
}

fn with_adjusted_indices_for_removed_cells(
    cells: &[CellWithIndex],
    predecessor: &RemoveCellsOperation,
) -> Vec<CellWithIndex> {
    let mut cells = cells.to_vec();
    adjust_indices_for_removed_cells(&mut cells, predecessor);
    cells
}

fn with_adjusted_indices_for_split_cell(
    state: &dyn TransformOperationState,
    cells: &[CellWithIndex],
    predecessor: &SplitCellOperation,
) -> Result<Vec<CellWithIndex>, Error> {
    let mut cells = cells.to_vec();
    adjust_indices_for_split_cell(state, &mut cells, predecessor)?;
    Ok(cells)
}

fn with_merged_source_ids(
    state: &dyn TransformOperationState,
    cells: &[CellWithIndex],
    predecessor: &UpdateCellOperation,
) -> Result<Vec<CellWithIndex>, Error> {
    cells
        .iter()
        .map(|c| {
            if c.cell.id() == predecessor.old_cell.id() {
                let cell = predecessor
                    .updated_cell
                    .with_source_ids(get_merged_source_ids(
                        state,
                        &predecessor.updated_cell,
                        &c.cell,
                    )?);
                Ok(CellWithIndex {
                    cell,
                    index: c.index,
                })
            } else {
                Ok(c.clone())
            }
        })
        .collect()
}

fn without_removed_cell(cells: &[CellWithIndex], removed_cell_id: &str) -> Vec<CellWithIndex> {
    cells
        .iter()
        .filter(|c| c.cell.id() != removed_cell_id)
        .cloned()
        .collect()
}

fn without_source_ids(cells: &[CellWithIndex], removed_source_ids: &[&str]) -> Vec<CellWithIndex> {
    cells
        .iter()
        .map(|c| {
            let source_ids = c
                .cell
                .source_ids()
                .into_iter()
                .filter_map(|id| {
                    if removed_source_ids.contains(&id) {
                        None
                    } else {
                        Some(id.to_owned())
                    }
                })
                .collect();
            CellWithIndex {
                cell: c.cell.with_source_ids(source_ids),
                index: c.index,
            }
        })
        .collect()
}
