use crate::{protocols::operations::*, text_util::char_count};

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
        MoveCells(operation) => invert_move_cells_operation(operation),
        ReplaceCells(operation) => invert_replace_cells_operation(operation),
        ReplaceText(operation) => invert_replace_text_operation(operation),
        UpdateNotebookTimeRange(operation) => invert_update_notebook_time_range(operation),
        UpdateNotebookTitle(operation) => invert_update_notebook_title(operation),
        AddDataSource(operation) => invert_add_data_source_operation(operation),
        UpdateDataSource(operation) => invert_update_data_source_operation(operation),
        RemoveDataSource(operation) => invert_remove_data_source_operation(operation),
        AddLabel(operation) => invert_add_label_operation(operation),
        ReplaceLabel(operation) => invert_replace_label_operation(operation),
        RemoveLabel(operation) => invert_remove_label_operation(operation),
    }
}

fn invert_move_cells_operation(operation: &MoveCellsOperation) -> Operation {
    Operation::MoveCells(MoveCellsOperation {
        cell_ids: operation.cell_ids.clone(),
        from_index: operation.to_index,
        to_index: operation.from_index,
    })
}

fn invert_replace_cells_operation(operation: &ReplaceCellsOperation) -> Operation {
    Operation::ReplaceCells(ReplaceCellsOperation {
        new_cells: operation.old_cells.clone(),
        old_cells: operation.new_cells.clone(),
        new_referencing_cells: operation.old_referencing_cells.clone(),
        old_referencing_cells: operation.new_referencing_cells.clone(),
        split_offset: operation.split_offset,
        merge_offset: operation.merge_offset.map(|_| {
            match (operation.new_cells.first(), operation.new_cells.last()) {
                (Some(first_cell), Some(last_cell)) if first_cell.id() == last_cell.id() => {
                    operation.split_offset.unwrap_or_default()
                        + last_cell.cell.text().map(char_count).unwrap_or_default()
                }
                (_, Some(last_cell)) => last_cell.cell.text().map(char_count).unwrap_or_default(),
                _ => 0,
            }
        }),
    })
}

fn invert_replace_text_operation(operation: &ReplaceTextOperation) -> Operation {
    Operation::ReplaceText(ReplaceTextOperation {
        cell_id: operation.cell_id.clone(),
        field: operation.field.clone(),
        offset: operation.offset,
        old_text: operation.new_text.clone(),
        old_formatting: operation.new_formatting.clone(),
        new_text: operation.old_text.clone(),
        new_formatting: operation.old_formatting.clone(),
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

fn invert_add_label_operation(operation: &AddLabelOperation) -> Operation {
    Operation::RemoveLabel(RemoveLabelOperation {
        label: operation.label.clone(),
    })
}

fn invert_replace_label_operation(operation: &ReplaceLabelOperation) -> Operation {
    Operation::ReplaceLabel(ReplaceLabelOperation {
        old_label: operation.new_label.clone(),
        new_label: operation.old_label.clone(),
    })
}

fn invert_remove_label_operation(operation: &RemoveLabelOperation) -> Operation {
    Operation::AddLabel(AddLabelOperation {
        label: operation.label.clone(),
    })
}
