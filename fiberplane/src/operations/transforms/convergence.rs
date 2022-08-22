use crate::{
    protocols::{core::Cell, operations::*},
    query_data::get_query_field,
    text_util::char_count,
};

pub(crate) fn moves_converge(move1: &MoveCellsOperation, move2: &MoveCellsOperation) -> bool {
    let move1_len = move1.cell_ids.len();
    let move2_len = move2.cell_ids.len();

    !ranges_overlap(move1.from_index, move1_len, move2.from_index, move2_len)
        && !ranges_overlap(move1.to_index, move1_len, move2.from_index, move2_len)
        && !ranges_overlap(move1.from_index, move1_len, move2.to_index, move2_len)
        && !ranges_overlap(move1.to_index, move1_len, move2.to_index, move2_len)
}

pub(crate) fn move_and_replace_cells_converge(
    move_op: &MoveCellsOperation,
    replace: &ReplaceCellsOperation,
) -> bool {
    if let Some(first_removed_cell) = replace.old_cells.first() {
        let move_len = move_op.cell_ids.len();
        let remove_len = replace.old_cells.len();

        !ranges_overlap(
            move_op.from_index,
            move_len,
            first_removed_cell.index,
            remove_len,
        ) && !ranges_overlap(
            move_op.to_index,
            move_len,
            first_removed_cell.index,
            remove_len,
        )
    } else {
        // If nothing was removed, we converge in any case.
        true
    }
}

pub(crate) fn ranges_overlap(
    range1_index: u32,
    range1_len: usize,
    range2_index: u32,
    range2_len: usize,
) -> bool {
    (range1_index + range1_len as u32 > range2_index)
        && (range1_index < range2_index + range2_len as u32)
}

#[cfg(test)]
pub(crate) fn replace_cells_converge(
    op1: &ReplaceCellsOperation,
    op2: &ReplaceCellsOperation,
) -> bool {
    // Two identical replacements make each other obsolete, which converges by definition:
    if op1 == op2 {
        return true;
    }

    let has_duplicate_in_new_ids = || {
        let new_cell_ids: Vec<&str> = op1
            .all_newly_inserted_cells()
            .map(CellWithIndex::id)
            .collect();
        op2.all_newly_inserted_cells()
            .any(|cell| new_cell_ids.contains(&cell.id()))
    };

    let has_overlap_in_old_cells = || {
        let old_cell_ids: Vec<&str> = op1.all_old_cells().map(CellWithIndex::id).collect();
        op2.all_old_cells()
            .any(|cell| old_cell_ids.contains(&cell.id()))
    };

    let op1_start_index = op1
        .old_cells
        .first()
        .or_else(|| op1.new_cells.first())
        .map(|cell| cell.index)
        .unwrap_or_default();
    let op2_start_index = op2
        .old_cells
        .first()
        .or_else(|| op2.new_cells.first())
        .map(|cell| cell.index)
        .unwrap_or_default();
    if ranges_overlap(
        op1_start_index,
        op1.old_cells.len(),
        op2_start_index,
        op2.old_cells.len(),
    ) {
        if op1.old_referencing_cells.iter().any(|old_cell| {
            op2.old_referencing_cells
                .iter()
                .any(|cell| cell.id() == old_cell.id())
        }) {
            return false;
        }

        let merge_cell_id = op1.old_cells.first().map(CellWithIndex::id);
        let op1_split_offset =
            op1.split_offset
                .filter(|_| match op2.old_cells.last().map(CellWithIndex::id) {
                    Some(last_old_cell_id) => merge_cell_id
                        .map(|id| id == last_old_cell_id)
                        .unwrap_or_default(),
                    None => false,
                });

        let split_cell_id = op1.old_cells.last().map(CellWithIndex::id);
        let op1_merge_offset =
            op1.merge_offset
                .filter(|_| match op2.old_cells.first().map(CellWithIndex::id) {
                    Some(first_old_cell_id) => split_cell_id
                        .map(|id| id == first_old_cell_id)
                        .unwrap_or_default(),
                    None => false,
                });

        if let (Some(op2_merge_offset), Some(op1_split_offset)) =
            (op2.merge_offset, op1_split_offset)
        {
            op2_merge_offset <= op1_split_offset
        } else if let (Some(op2_split_offset), Some(op1_merge_offset)) =
            (op2.split_offset, op1_merge_offset)
        {
            op2_split_offset >= op1_merge_offset
        } else {
            false
        }
    } else {
        !has_duplicate_in_new_ids() && !has_overlap_in_old_cells()
    }
}

pub(crate) fn replace_cells_and_replace_text_converge(
    cells_op: &ReplaceCellsOperation,
    text_op: &ReplaceTextOperation,
) -> bool {
    if let Some(field) = text_op.field.as_ref() {
        return replace_cells_and_replace_text_field_converge(cells_op, text_op, field);
    }

    if let Some(index) = cells_op
        .old_cells
        .iter()
        .position(|cell| cell.id() == text_op.cell_id)
    {
        // The cell in which we want to change text is part of the range of
        // cells that was replaced. We can only hope our text range lies outside
        // the split or merge offset:
        if index == 0 {
            if let Some(split_offset) = cells_op.split_offset {
                return text_op.offset + char_count(&text_op.old_text) <= split_offset;
            }
        } else if index == cells_op.old_cells.len() - 1 {
            if let Some(merge_offset) = cells_op.merge_offset {
                return text_op.offset >= merge_offset;
            }
        }

        false
    } else if let Some(cell) = cells_op
        .old_referencing_cells
        .iter()
        .find(|cell| cell.id() == text_op.cell_id)
    {
        // If the cell that we want to change text in is found in the
        // referencing cells, the only thing that can save us is if the text
        // did not change in the new referencing cell:
        cells_op
            .new_referencing_cells
            .iter()
            .find(|new_cell| new_cell.id() == cell.id())
            .map(|new_cell| new_cell.text() == cell.text())
            .unwrap_or_default()
    } else {
        true // No overlap in cells.
    }
}

fn replace_cells_and_replace_text_field_converge(
    cells_op: &ReplaceCellsOperation,
    text_op: &ReplaceTextOperation,
    field_name: &str,
) -> bool {
    // When a field is being updated, the only way for the operations to
    // converge is if the cells don't overlap, or the field was not touched.
    if let Some(old_cell) = cells_op
        .old_cells
        .iter()
        .chain(cells_op.old_referencing_cells.iter())
        .find(|cell| cell.id() == text_op.cell_id)
    {
        // If the cell was replaced, we can only verify the field was not
        // touched if it was a field inside a ProviderCell:
        cells_op
            .new_cells
            .iter()
            .chain(cells_op.new_referencing_cells.iter())
            .find(|new_cell| new_cell.id() == old_cell.id())
            .map(|new_cell| match (&new_cell.cell, &old_cell.cell) {
                (Cell::Provider(new_cell), Cell::Provider(old_cell)) => {
                    match (new_cell.query_data.as_ref(), old_cell.query_data.as_ref()) {
                        (Some(new_query_data), Some(old_query_data)) => {
                            get_query_field(new_query_data, field_name)
                                == get_query_field(old_query_data, field_name)
                        }
                        (None, None) => true,
                        _ => false,
                    }
                }
                _ => false,
            })
            .unwrap_or_default()
    } else {
        true // No overlap in cells.
    }
}

#[cfg(test)]
pub(crate) fn replace_texts_converge(
    op1: &ReplaceTextOperation,
    op2: &ReplaceTextOperation,
) -> bool {
    // Replacements in different cells always converge:
    if op1.cell_id != op2.cell_id || op1.field != op2.field {
        return true;
    }

    // Convergence works as long as there's no overlap in the regions being replaced:
    op1.offset + char_count(&op1.old_text) <= op2.offset
        || op1.offset >= op2.offset + char_count(&op2.old_text)
}
