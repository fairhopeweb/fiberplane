use std::cmp::Ordering;

use super::{
    convergence::*,
    get_cell_index_changes::{get_cell_index_changes, CellIndexChange, CellIndexPriority},
    TransformOperationState,
};
use crate::{operations::error::*, protocols::operations::*, text_util::char_count};

pub(crate) fn transform_move_cells_operation(
    _: &dyn TransformOperationState,
    successor: &MoveCellsOperation,
    predecessor: &Operation,
) -> Result<Option<Operation>, Error> {
    let operation = match predecessor {
        Operation::MoveCells(predecessor) => {
            if moves_converge(successor, predecessor) {
                Some(Operation::MoveCells(MoveCellsOperation {
                    cell_ids: successor.cell_ids.clone(),
                    from_index: get_index_adjusted_for_moved_cells(
                        successor.from_index,
                        None,
                        predecessor,
                    ),
                    to_index: get_index_adjusted_for_moved_cells(
                        successor.to_index,
                        successor.cell_ids.first().map(String::as_str),
                        predecessor,
                    ),
                }))
            } else {
                // Discard successor in case of non-convergence:
                None
            }
        }
        Operation::ReplaceCells(predecessor) => {
            if move_and_replace_cells_converge(successor, predecessor) {
                let removed_cell_ids: Vec<&str> = predecessor
                    .old_removed_cells()
                    .map(CellWithIndex::id)
                    .collect();
                let cell_ids: Vec<String> = successor
                    .cell_ids
                    .iter()
                    .filter(|cell_id| !removed_cell_ids.contains(&cell_id.as_str()))
                    .cloned()
                    .collect();

                if cell_ids.is_empty() {
                    None // Everything we tried to move was already removed.
                } else {
                    let cell_index_changes = get_cell_index_changes(predecessor);
                    let first_cell_id = cell_ids.first().map(String::as_str).unwrap_or_default();

                    let from_index = get_index_adjusted_for_old_replaced_cell(
                        successor.from_index,
                        first_cell_id,
                        &cell_index_changes,
                    );
                    let to_index = get_index_adjusted_for_new_replaced_cell(
                        successor.to_index,
                        first_cell_id,
                        CellIndexPriority::Normal,
                        &cell_index_changes,
                    );

                    Some(Operation::MoveCells(MoveCellsOperation {
                        cell_ids,
                        from_index,
                        to_index,
                    }))
                }
            } else {
                None
            }
        }
        Operation::ReplaceText(_)
        | Operation::UpdateNotebookTimeRange(_)
        | Operation::UpdateNotebookTitle(_)
        | Operation::AddDataSource(_)
        | Operation::UpdateDataSource(_)
        | Operation::RemoveDataSource(_)
        | Operation::AddLabel(_)
        | Operation::ReplaceLabel(_)
        | Operation::RemoveLabel(_) => Some(Operation::MoveCells(successor.clone())),
    };

    Ok(operation)
}

pub(crate) fn transform_replace_cells_operation(
    _: &dyn TransformOperationState,
    successor: &ReplaceCellsOperation,
    predecessor: &Operation,
) -> Result<Option<Operation>, Error> {
    let operation = match predecessor {
        Operation::MoveCells(predecessor) => {
            transform_replace_cells_after_move_cells(successor, predecessor)
        }
        Operation::ReplaceCells(predecessor) => {
            transform_replace_cells_after_replace_cells(successor, predecessor)?
        }
        Operation::ReplaceText(predecessor) => {
            transform_replace_cells_after_replace_text(successor, predecessor)
        }
        Operation::UpdateNotebookTimeRange(_)
        | Operation::UpdateNotebookTitle(_)
        | Operation::AddDataSource(_)
        | Operation::UpdateDataSource(_)
        | Operation::RemoveDataSource(_)
        | Operation::AddLabel(_)
        | Operation::ReplaceLabel(_)
        | Operation::RemoveLabel(_) => Some(Operation::ReplaceCells(successor.clone())),
    };

    Ok(operation)
}

fn transform_replace_cells_after_move_cells(
    successor: &ReplaceCellsOperation,
    predecessor: &MoveCellsOperation,
) -> Option<Operation> {
    if !move_and_replace_cells_converge(predecessor, successor) {
        return None;
    }

    Some(Operation::ReplaceCells(ReplaceCellsOperation {
        new_cells: with_adjusted_adjacent_indices_for_moved_cells(
            &successor.new_cells,
            predecessor,
        ),
        old_cells: with_adjusted_adjacent_indices_for_moved_cells(
            &successor.old_cells,
            predecessor,
        ),
        new_referencing_cells: with_adjusted_disjoint_indices_for_moved_cells(
            &successor.new_referencing_cells,
            predecessor,
        ),
        old_referencing_cells: with_adjusted_disjoint_indices_for_moved_cells(
            &successor.old_referencing_cells,
            predecessor,
        ),
        split_offset: successor.split_offset,
        merge_offset: successor.merge_offset,
    }))
}

fn transform_replace_cells_after_replace_cells(
    successor: &ReplaceCellsOperation,
    predecessor: &ReplaceCellsOperation,
) -> Result<Option<Operation>, Error> {
    let has_duplicate_in_new_ids = || {
        let new_cell_ids: Vec<&str> = successor
            .all_newly_inserted_cells()
            .map(CellWithIndex::id)
            .collect();
        predecessor
            .all_newly_inserted_cells()
            .any(|cell| new_cell_ids.contains(&cell.id()))
    };

    let has_overlap_in_old_cells = || {
        let old_cell_ids: Vec<&str> = successor.all_old_cells().map(CellWithIndex::id).collect();
        predecessor
            .all_old_cells()
            .any(|cell| old_cell_ids.contains(&cell.id()))
    };

    let transform_old_referencing_cells = |predecessor_index_changes: &[CellIndexChange]| {
        with_adjusted_indices_for_old_replaced_cells(
            &successor
                .old_referencing_cells
                .iter()
                .map(|old_referencing_cell| {
                    if let Some(new_referencing_cell) = predecessor
                        .new_referencing_cells
                        .iter()
                        .find(|new_referencing_cell| {
                            new_referencing_cell.id() == old_referencing_cell.id()
                        })
                    {
                        CellWithIndex {
                            cell: new_referencing_cell.cell.clone(),
                            index: old_referencing_cell.index,
                        }
                    } else {
                        old_referencing_cell.clone()
                    }
                })
                .collect::<Vec<_>>(),
            predecessor_index_changes,
        )
    };

    let transform_without_conflicts = || {
        let predecessor_index_changes = get_cell_index_changes(predecessor);
        let successor_index_changes = get_cell_index_changes(successor);
        Operation::ReplaceCells(ReplaceCellsOperation {
            new_cells: with_adjusted_indices_for_new_replaced_cells(
                &successor.new_cells,
                CellIndexPriority::Normal,
                &predecessor_index_changes,
                &successor_index_changes,
            ),
            old_cells: with_adjusted_indices_for_old_replaced_cells(
                &successor.old_cells,
                &predecessor_index_changes,
            ),
            new_referencing_cells: with_adjusted_indices_for_new_replaced_cells(
                &successor.new_referencing_cells,
                CellIndexPriority::Low,
                &predecessor_index_changes,
                &successor_index_changes,
            ),
            old_referencing_cells: transform_old_referencing_cells(&predecessor_index_changes),
            split_offset: successor.split_offset,
            merge_offset: successor.merge_offset,
        })
    };

    // First we determine the indices to see if the ranges overlap:
    let successor_start_index = successor
        .old_cells
        .first()
        .or_else(|| successor.new_cells.first())
        .map(|cell| cell.index)
        .unwrap_or_default();
    let predecessor_start_index = predecessor
        .old_cells
        .first()
        .or_else(|| predecessor.new_cells.first())
        .map(|cell| cell.index)
        .unwrap_or_default();
    if !ranges_overlap(
        successor_start_index,
        successor.old_cells.len(),
        predecessor_start_index,
        predecessor.old_cells.len(),
    ) {
        return if has_duplicate_in_new_ids() || has_overlap_in_old_cells() {
            // Overlap in the referencing cells is easy, we drop the operation:
            Ok(None)
        } else {
            // Saul Goodman!
            Ok(Some(transform_without_conflicts()))
        };
    }

    // Now it gets real fun :)
    // If there is overlap in the cell ranges, the only way to transform the
    // operation is if there is no overlap in the text, due to the split or
    // merge offset...

    // ... and we still should double-check there is no overlap in referencing
    // cells.
    if successor.old_referencing_cells.iter().any(|old_cell| {
        predecessor
            .old_referencing_cells
            .iter()
            .any(|cell| cell.id() == old_cell.id())
    }) {
        // No cigar.
        return Ok(None);
    }

    // We make sure `predecessor_split_offset` is only `Some` if the cell at
    // which the predecessor split is the same as the one where the successor
    // merges:
    let successor_merge_cell_id = predecessor.old_cells.first().map(CellWithIndex::id);
    let predecessor_split_offset = predecessor.split_offset.filter(|_| {
        match successor.old_cells.last().map(CellWithIndex::id) {
            Some(last_old_cell_id) => successor_merge_cell_id
                .map(|id| id == last_old_cell_id)
                .unwrap_or_default(),
            None => false,
        }
    });

    // We make sure `predecessor_merge_offset` is only `Some` if the cell at
    // which the predecessor merges is the same as the one where the successor
    // splits:
    let successor_split_cell_id = predecessor.old_cells.last().map(CellWithIndex::id);
    let predecessor_merge_offset = predecessor.merge_offset.filter(|_| {
        match successor.old_cells.first().map(CellWithIndex::id) {
            Some(first_old_cell_id) => successor_split_cell_id
                .map(|id| id == first_old_cell_id)
                .unwrap_or_default(),
            None => false,
        }
    });

    if let (Some(successor_merge_offset), Some(predecessor_split_offset)) =
        (successor.merge_offset, predecessor_split_offset)
    {
        if successor_merge_offset <= predecessor_split_offset {
            // The successor can still be cleanly applied before the
            // predecessor.
            return Ok(Some(transform_without_conflicts()));
        }
    }

    if let (Some(successor_split_offset), Some(predecessor_merge_offset)) =
        (successor.split_offset, predecessor_merge_offset)
    {
        let transform_split_cell = |cell: &CellWithIndex| -> Result<CellWithIndex, Error> {
            if Some(cell.id()) == successor_split_cell_id {
                match predecessor.new_cells.last() {
                    Some(split_cell) => Ok(CellWithIndex {
                        cell: match (cell.text(), cell.formatting()) {
                            (Some(text), Some(formatting)) => {
                                split_cell.cell.with_rich_text(text, formatting.clone())
                            }
                            (Some(text), None) => split_cell.cell.with_text(text),
                            _ => {
                                return Err(Error::NoTextCell(cell.id().to_owned()));
                            }
                        },
                        index: cell.index,
                    }),
                    None => Err(Error::CellNotFound(
                        successor_split_cell_id.unwrap_or_default().to_owned(),
                    )),
                }
            } else {
                Ok(cell.clone())
            }
        };

        if successor_split_offset >= predecessor_merge_offset {
            // Now we need a thorough transformation:
            let new_cells = successor
                .new_cells
                .iter()
                .map(transform_split_cell)
                .collect::<Result<Vec<CellWithIndex>, Error>>()?;

            let old_cells: Vec<CellWithIndex> = successor
                .old_cells
                .iter()
                .map(transform_split_cell)
                .collect::<Result<Vec<CellWithIndex>, Error>>()?;

            let predecessor_index_changes = get_cell_index_changes(predecessor);
            let successor_index_changes = get_cell_index_changes(successor);
            return Ok(Some(Operation::ReplaceCells(ReplaceCellsOperation {
                new_cells: with_adjusted_indices_for_new_replaced_cells(
                    &new_cells,
                    CellIndexPriority::Normal,
                    &predecessor_index_changes,
                    &successor_index_changes,
                ),
                old_cells: with_adjusted_indices_for_old_replaced_cells(
                    &old_cells,
                    &predecessor_index_changes,
                ),
                new_referencing_cells: with_adjusted_indices_for_new_replaced_cells(
                    &successor.new_referencing_cells,
                    CellIndexPriority::Low,
                    &predecessor_index_changes,
                    &successor_index_changes,
                ),
                old_referencing_cells: transform_old_referencing_cells(&predecessor_index_changes),
                split_offset: get_optional_offset_adjusted_for_replaced_cells(
                    old_cells.first().map(CellWithIndex::id).unwrap_or_default(),
                    successor.split_offset,
                    predecessor,
                )?,
                merge_offset: successor.merge_offset,
            })));
        }
    }

    Ok(None)
}

fn transform_replace_cells_after_replace_text(
    successor: &ReplaceCellsOperation,
    predecessor: &ReplaceTextOperation,
) -> Option<Operation> {
    if !replace_cells_and_replace_text_converge(successor, predecessor) {
        return None;
    }

    let operation = match (successor.split_offset, successor.old_cells.first()) {
        (Some(split_offset), Some(first_old_cell))
            if first_old_cell.id() == predecessor.cell_id && predecessor.offset < split_offset =>
        {
            ReplaceCellsOperation {
                split_offset: Some(
                    split_offset + char_count(&predecessor.new_text)
                        - char_count(&predecessor.old_text),
                ),
                ..successor.clone()
            }
        }
        _ => successor.clone(),
    };

    Some(Operation::ReplaceCells(operation))
}

pub(crate) fn transform_replace_text_operation(
    _: &dyn TransformOperationState,
    successor: &ReplaceTextOperation,
    predecessor: &Operation,
) -> Result<Option<Operation>, Error> {
    let operation = match predecessor {
        Operation::ReplaceCells(predecessor) => {
            if replace_cells_and_replace_text_converge(predecessor, successor) {
                match (predecessor.merge_offset, predecessor.old_cells.last()) {
                    (Some(merge_offset), Some(last_old_cell))
                        if last_old_cell.id() == successor.cell_id
                            && successor.offset >= merge_offset =>
                    {
                        if let (Some(split_offset), Some(last_new_cell)) =
                            (predecessor.split_offset, predecessor.new_cells.last())
                        {
                            Some(Operation::ReplaceText(ReplaceTextOperation {
                                cell_id: last_new_cell.id().to_owned(),
                                offset: split_offset
                                    + successor.offset
                                    + last_new_cell.text().map(char_count).unwrap_or_default()
                                    - merge_offset,
                                ..successor.clone()
                            }))
                        } else {
                            None
                        }
                    }
                    _ => Some(Operation::ReplaceText(successor.clone())),
                }
            } else {
                None
            }
        }
        Operation::ReplaceText(predecessor) => {
            let ordering = if predecessor.cell_id != successor.cell_id {
                // We are slightly abused the `Ordering` enum here, but the
                // result for mismatching cell IDs is the same as when the
                // successor comes *before* the predecessor.
                Ordering::Less
            } else if successor.offset == predecessor.offset
                && predecessor.old_text.is_empty()
                && successor.old_text.is_empty()
            {
                if successor.new_text < predecessor.new_text {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            } else if predecessor.offset + char_count(&predecessor.old_text) <= successor.offset {
                Ordering::Greater
            } else if successor.offset + char_count(&successor.old_text) <= predecessor.offset {
                Ordering::Less
            } else {
                Ordering::Equal
            };
            match ordering {
                Ordering::Greater => {
                    // Adjust the offset to account for the previous replacement:
                    Some(Operation::ReplaceText(ReplaceTextOperation {
                        cell_id: successor.cell_id.clone(),
                        field: successor.field.clone(),
                        offset: successor.offset + char_count(&predecessor.new_text)
                            - char_count(&predecessor.old_text),
                        new_text: successor.new_text.clone(),
                        new_formatting: successor.new_formatting.clone(),
                        old_text: successor.old_text.clone(),
                        old_formatting: successor.old_formatting.clone(),
                    }))
                }
                Ordering::Equal => {
                    // No convergence due to overlapping regions.
                    None
                }
                Ordering::Less => {
                    // Previous replacement didn't affect ours.
                    Some(Operation::ReplaceText(successor.clone()))
                }
            }
        }
        Operation::MoveCells(_)
        | Operation::UpdateNotebookTimeRange(_)
        | Operation::UpdateNotebookTitle(_)
        | Operation::AddDataSource(_)
        | Operation::UpdateDataSource(_)
        | Operation::RemoveDataSource(_)
        | Operation::AddLabel(_)
        | Operation::ReplaceLabel(_)
        | Operation::RemoveLabel(_) => Some(Operation::ReplaceText(successor.clone())),
    };

    Ok(operation)
}

/// Returns a delta that should be applied to the cell index to transform after
/// the given predecessor.
fn get_delta_for_moved_cells(
    index: u32,
    new_cell_id: Option<&str>,
    predecessor: &MoveCellsOperation,
) -> i32 {
    let is_predecessor_before = || {
        predecessor.to_index < index
            || new_cell_id
                .map(|new_cell_id| {
                    predecessor.to_index == index
                        && predecessor
                            .cell_ids
                            .first()
                            .map(|cell_id| cell_id.as_str() < new_cell_id)
                            .unwrap_or_default()
                })
                .unwrap_or_default()
    };

    if predecessor.from_index < index {
        if predecessor.from_index + predecessor.cell_ids.len() as u32 > index {
            // Our index was moved as part of a range, so we move it along:
            predecessor.to_index as i32 - predecessor.from_index as i32
        } else if predecessor.to_index >= index {
            // Cells were moved from somewhere above our index to somewhere below,
            // so we move up to compensate:
            -(predecessor.cell_ids.len() as i32)
        } else {
            0
        }
    } else if is_predecessor_before() {
        // Cells were moved from somewhere below our index to somewhere above,
        // so we move down to compensate:
        predecessor.cell_ids.len() as i32
    } else {
        0
    }
}

fn get_delta_for_new_replaced_cell(
    new_index: u32,
    new_cell_id: &str,
    changes: &[CellIndexChange],
) -> i32 {
    let mut delta = 0;
    for change in changes {
        match change {
            CellIndexChange::Insertion {
                new_index: insertion_index,
                cell_id,
                ..
            } => {
                if cell_id == &new_cell_id {
                    return delta;
                } else if insertion_index < &new_index {
                    delta += 1;
                }
            }
            CellIndexChange::Removal {
                new_index: removal_index,
                ..
            } => {
                if removal_index < &new_index {
                    delta -= 1;
                }
            }
            CellIndexChange::Replacement { cell_id, .. } => {
                if cell_id == &new_cell_id {
                    return delta;
                }
            }
        }
    }

    delta
}

fn get_index_adjusted_for_moved_cells(
    index: u32,
    new_cell_id: Option<&str>,
    predecessor: &MoveCellsOperation,
) -> u32 {
    let delta = get_delta_for_moved_cells(index, new_cell_id, predecessor);
    (index as i32 + delta) as u32
}

fn get_index_adjusted_for_new_replaced_cell(
    mut new_index: u32,
    new_cell_id: &str,
    new_cell_priority: CellIndexPriority,
    changes: &[CellIndexChange],
) -> u32 {
    for change in changes {
        match change {
            CellIndexChange::Insertion {
                new_index: insertion_index,
                cell_id,
                priority,
                ..
            } => {
                if cell_id == &new_cell_id {
                    return *insertion_index;
                } else if CellIndexPriority::successor_should_move(
                    *insertion_index,
                    cell_id,
                    *priority,
                    new_index,
                    new_cell_id,
                    new_cell_priority,
                ) {
                    new_index += 1;
                }
            }
            CellIndexChange::Removal {
                new_index: removal_index,
                ..
            } => {
                if removal_index < &new_index {
                    new_index -= 1;
                }
            }
            CellIndexChange::Replacement {
                new_index: replacement_index,
                cell_id,
                ..
            } => {
                if cell_id == &new_cell_id {
                    return *replacement_index;
                }
            }
        }
    }

    new_index
}

fn get_index_adjusted_for_old_replaced_cell(
    mut old_index: u32,
    old_cell_id: &str,
    changes: &[CellIndexChange],
) -> u32 {
    let mut num_removed_cells = 0;
    for change in changes {
        match change {
            CellIndexChange::Insertion {
                old_index: insertion_index,
                ..
            } => {
                if insertion_index <= &old_index {
                    old_index += 1;
                }
            }
            CellIndexChange::Removal {
                old_index: removal_index,
                cell_id,
                ..
            } => {
                if cell_id == &old_cell_id {
                    return *removal_index;
                } else if removal_index < &old_index {
                    num_removed_cells += 1;
                }
            }
            CellIndexChange::Replacement {
                old_index: replacement_index,
                cell_id,
                ..
            } => {
                if cell_id == &old_cell_id {
                    return *replacement_index;
                }
            }
        }
    }

    old_index - num_removed_cells
}

/// Returns a text offset in a cell, adjusted for the predecessor.
fn get_offset_adjusted_for_replaced_cells(
    cell_id: &str,
    offset: u32,
    predecessor: &ReplaceCellsOperation,
) -> Result<u32, Error> {
    let merge_cell = predecessor.new_cells.last();

    let merge_offset = predecessor.merge_offset.filter(|_| {
        merge_cell
            .map(CellWithIndex::id)
            .map(|merge_cell_id| merge_cell_id == cell_id)
            .unwrap_or_default()
    });

    let split_offset = predecessor.split_offset.filter(|_| {
        predecessor
            .new_cells
            .first()
            .map(CellWithIndex::id)
            .map(|split_cell_id| split_cell_id == cell_id)
            .unwrap_or_default()
    });

    match (merge_offset, split_offset) {
        (Some(merge_offset), Some(split_offset)) => {
            if offset >= merge_offset {
                match merge_cell.and_then(CellWithIndex::text).map(char_count) {
                    Some(new_text_len) => Ok(offset - merge_offset + new_text_len + split_offset),
                    None => Err(Error::NoTextCell(cell_id.to_owned())),
                }
            } else if offset <= split_offset {
                Ok(offset)
            } else {
                Err(Error::InvalidTextOffset(cell_id.to_owned(), offset))
            }
        }
        (Some(merge_offset), None) => {
            if offset >= merge_offset {
                match merge_cell.and_then(CellWithIndex::text).map(char_count) {
                    Some(new_text_len) => Ok(offset - merge_offset + new_text_len),
                    None => Err(Error::NoTextCell(cell_id.to_owned())),
                }
            } else {
                Err(Error::InvalidTextOffset(cell_id.to_owned(), offset))
            }
        }
        (None, Some(split_offset)) => {
            if offset <= split_offset {
                Ok(offset)
            } else {
                Err(Error::InvalidTextOffset(cell_id.to_owned(), offset))
            }
        }
        (None, None) => Ok(offset),
    }
}

fn get_optional_offset_adjusted_for_replaced_cells(
    cell_id: &str,
    offset: Option<u32>,
    predecessor: &ReplaceCellsOperation,
) -> Result<Option<u32>, Error> {
    match offset {
        Some(offset) => Ok(Some(get_offset_adjusted_for_replaced_cells(
            cell_id,
            offset,
            predecessor,
        )?)),
        None => Ok(None),
    }
}

fn with_adjusted_adjacent_indices_for_moved_cells(
    cells: &[CellWithIndex],
    predecessor: &MoveCellsOperation,
) -> Vec<CellWithIndex> {
    let mut cells = cells.to_vec();
    if let Some(first_cell) = cells.first() {
        let delta = get_delta_for_moved_cells(first_cell.index, Some(first_cell.id()), predecessor);
        if delta != 0 {
            for cell in cells.iter_mut() {
                cell.index = (cell.index as i32 + delta) as u32;
            }
        }
    }
    cells
}

fn with_adjusted_disjoint_indices_for_moved_cells(
    cells: &[CellWithIndex],
    predecessor: &MoveCellsOperation,
) -> Vec<CellWithIndex> {
    cells
        .iter()
        .map(|cell| CellWithIndex {
            cell: cell.cell.clone(),
            index: get_index_adjusted_for_moved_cells(cell.index, Some(cell.id()), predecessor),
        })
        .collect()
}

fn with_adjusted_indices_for_new_replaced_cells(
    new_cells: &[CellWithIndex],
    new_cell_priority: CellIndexPriority,
    predecessor_index_changes: &[CellIndexChange],
    successor_index_changes: &[CellIndexChange],
) -> Vec<CellWithIndex> {
    new_cells
        .iter()
        .enumerate()
        .map(|(i, cell)| {
            let delta =
                get_delta_for_new_replaced_cell(cell.index, cell.id(), successor_index_changes);
            CellWithIndex {
                cell: cell.cell.clone(),
                index: (get_index_adjusted_for_new_replaced_cell(
                    (cell.index as i32 - delta) as u32,
                    cell.id(),
                    if i > 0 && new_cell_priority == CellIndexPriority::Normal {
                        // If the cell priority is normal, we escalate to
                        // high after the first cell, to avoid breaking up
                        // continuous ranges.
                        CellIndexPriority::High
                    } else {
                        new_cell_priority
                    },
                    predecessor_index_changes,
                ) as i32
                    + delta) as u32,
            }
        })
        .collect()
}

fn with_adjusted_indices_for_old_replaced_cells(
    old_cells: &[CellWithIndex],
    predecessor_index_changes: &[CellIndexChange],
) -> Vec<CellWithIndex> {
    old_cells
        .iter()
        .map(|cell| CellWithIndex {
            cell: cell.cell.clone(),
            index: get_index_adjusted_for_old_replaced_cell(
                cell.index,
                cell.id(),
                predecessor_index_changes,
            ),
        })
        .collect()
}
