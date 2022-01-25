use std::borrow::Borrow;

use crate::operations::{change::*, error::*};
use crate::protocols::{core::*, operations::*};

#[derive(Clone, Debug, PartialEq)]
pub struct CellRefWithIndex<'a> {
    pub cell: &'a Cell,
    pub index: u32,
}

/// Allows `apply_operation()` to query for the cells that may be affected by a single operation.
///
/// See `relevant_cell_ids_for_operation()` if you want to know how to determine which cells may be
/// affected by an operation.
pub trait ApplyOperationState {
    /// Returns all cells that may be relevant for the operation, including their indices.
    ///
    /// This may always return *more* cells than are relevant for the operation, but it must
    /// *at least* return those that will be affected.
    fn all_relevant_cells(&self) -> Vec<CellRefWithIndex>;

    /// Returns a cell by ID.
    ///
    /// May return `None` if the cell exists, but was considered not relevant for the operation.
    fn cell(&self, id: &str) -> Option<&Cell> {
        self.cell_with_index(id).map(|c| c.cell)
    }

    /// Returns a cell by ID, plus the index of that cell in the notebook.
    ///
    /// May return `None` if the cell exists, but was considered not relevant for the operation.
    fn cell_with_index(&self, id: &str) -> Option<CellRefWithIndex> {
        self.all_relevant_cells()
            .into_iter()
            .find(|c| c.cell.id() == id)
    }
}

/// Applies an operation to the given notebook state.
///
/// Clients are responsible for making sure all cells that are relevant to a given operation are
/// included in the state. A naive client may simply include all cells.
///
/// Note: The name of this function is a bit of a misnomer as it doesn't actually apply the
///       necessary changes itself. It merely indicates which changes must be executed to apply
///       the operation.
pub fn apply_operation(
    state: &dyn ApplyOperationState,
    operation: &Operation,
) -> Result<Vec<Change>, Error> {
    use Operation::*;
    match operation {
        AddCells(operation) => Ok(apply_add_cells_operation(state, operation)),
        MergeCells(operation) => apply_merge_cells_operation(state, operation),
        MoveCells(operation) => Ok(apply_move_cells_operation(state, operation)),
        RemoveCells(operation) => Ok(apply_remove_cells_operation(state, operation)),
        ReplaceText(operation) => apply_replace_text_operation(state, operation),
        SplitCell(operation) => apply_split_cells_operation(state, operation),
        UpdateCell(operation) => apply_update_cell_operation(state, operation),
        UpdateNotebookTimeRange(operation) => {
            Ok(apply_update_notebook_time_range(state, operation))
        }
        UpdateNotebookTitle(operation) => Ok(apply_update_notebook_title(state, operation)),
        AddDataSource(operation) => Ok(apply_add_data_source_operation(state, operation)),
        UpdateDataSource(operation) => Ok(apply_update_data_source_operation(state, operation)),
        RemoveDataSource(operation) => Ok(apply_remove_data_source_operation(state, operation)),
        AddLabel(operation) => Ok(apply_add_label_operation(state, operation)),
        ReplaceLabel(operation) => Ok(apply_replace_label_operation(state, operation)),
        RemoveLabel(operation) => Ok(apply_remove_label_operation(state, operation)),
    }
}

fn apply_add_cells_operation(
    _: &dyn ApplyOperationState,
    operation: &AddCellsOperation,
) -> Vec<Change> {
    let mut changes: Vec<Change> = operation
        .cells
        .iter()
        .map(|CellWithIndex { cell, index }| {
            Change::InsertCell(InsertCellChange {
                cell: cell.clone(),
                index: *index,
            })
        })
        .collect();

    if let Some(referencing_cells) = &operation.referencing_cells {
        for referencing_cell in referencing_cells {
            changes.push(Change::UpdateCell(UpdateCellChange {
                cell: referencing_cell.cell.clone(),
            }))
        }
    }

    changes
}

fn apply_merge_cells_operation(
    state: &dyn ApplyOperationState,
    operation: &MergeCellsOperation,
) -> Result<Vec<Change>, Error> {
    let MergeCellsOperation {
        source_cell,
        target_cell_id,
        target_content_length,
        glue_text,
        referencing_cells,
    } = operation;

    let target_cell = state
        .cell(target_cell_id)
        .ok_or_else(|| Error::CellNotFound(target_cell_id.clone()))?;
    if target_cell.content().unwrap_or_default().chars().count() != *target_content_length as usize
    {
        return Err(Error::ContentLengthMismatch(*target_content_length));
    }

    let mut changes = vec![
        Change::UpdateCellText(UpdateCellTextChange {
            cell_id: target_cell_id.clone(),
            text: format!(
                "{}{}{}",
                target_cell.content().unwrap_or(""),
                glue_text.clone().unwrap_or_default(),
                source_cell.content().unwrap_or("")
            ),
        }),
        Change::DeleteCell(DeleteCellChange {
            cell_id: source_cell.id().clone(),
        }),
    ];

    if let Some(referencing_cells) = referencing_cells.as_ref() {
        for referencing_cell in referencing_cells {
            let mut source_ids = referencing_cell.cell.source_ids();
            source_ids.retain(|id| id != source_cell.id());
            changes.push(get_change_for_dropped_reference(
                referencing_cell,
                source_ids,
            ));
        }
    }

    Ok(changes)
}

fn apply_move_cells_operation(
    _: &dyn ApplyOperationState,
    operation: &MoveCellsOperation,
) -> Vec<Change> {
    vec![Change::MoveCells(MoveCellsChange {
        cell_ids: operation.cell_ids.to_vec(),
        index: operation.to_index,
    })]
}

fn apply_remove_cells_operation(
    _: &dyn ApplyOperationState,
    operation: &RemoveCellsOperation,
) -> Vec<Change> {
    let mut changes: Vec<Change> = operation
        .removed_cells
        .iter()
        .map(|removed_cell| {
            Change::DeleteCell(DeleteCellChange {
                cell_id: removed_cell.cell.id().clone(),
            })
        })
        .collect();

    if let Some(referencing_cells) = &operation.referencing_cells {
        for referencing_cell in referencing_cells {
            let mut source_ids = referencing_cell.cell.source_ids();
            source_ids.retain(|id| !operation.removed_cells.iter().any(|c| c.cell.id() == id));
            changes.push(get_change_for_dropped_reference(
                referencing_cell,
                source_ids,
            ));
        }
    }

    changes
}

fn apply_replace_text_operation(
    state: &dyn ApplyOperationState,
    operation: &ReplaceTextOperation,
) -> Result<Vec<Change>, Error> {
    match state.cell(&operation.cell_id) {
        Some(cell) => {
            if let Some(text) = cell.text() {
                Ok(vec![Change::UpdateCellText(UpdateCellTextChange {
                    cell_id: operation.cell_id.clone(),
                    text: replace_text(text, operation),
                })])
            } else {
                Err(Error::NoContentCell(operation.cell_id.clone()))
            }
        }
        None => Err(Error::CellNotFound(operation.cell_id.clone())),
    }
}

fn apply_split_cells_operation(
    state: &dyn ApplyOperationState,
    operation: &SplitCellOperation,
) -> Result<Vec<Change>, Error> {
    let cell_with_index = state
        .cell_with_index(&operation.cell_id)
        .ok_or_else(|| Error::CellNotFound(operation.cell_id.clone()))?;
    let cell = &cell_with_index.cell;

    let mut changes = vec![
        Change::UpdateCellText(UpdateCellTextChange {
            cell_id: cell.id().clone(),
            text: cell
                .content()
                .map(|c| c.chars().take(operation.split_index as usize).collect())
                .unwrap_or_default(),
        }),
        Change::InsertCell(InsertCellChange {
            cell: operation.new_cell.clone(),
            index: cell_with_index.index + 1,
        }),
    ];

    if let Some(referencing_cells) = &operation.referencing_cells {
        for referencing_cell in referencing_cells {
            if state
                .all_relevant_cells()
                .iter()
                .any(|c| c.cell.id() == referencing_cell.cell.id())
            {
                changes.push(Change::UpdateCell(UpdateCellChange {
                    cell: referencing_cell.cell.clone(),
                }))
            } else {
                changes.push(Change::InsertCell(InsertCellChange {
                    cell: referencing_cell.cell.clone(),
                    index: referencing_cell.index,
                }))
            }
        }
    }

    Ok(changes)
}

fn apply_update_cell_operation(
    state: &dyn ApplyOperationState,
    operation: &UpdateCellOperation,
) -> Result<Vec<Change>, Error> {
    if operation.updated_cell.id() == operation.old_cell.id() {
        Ok(vec![Change::UpdateCell(UpdateCellChange {
            cell: operation.updated_cell.as_ref().clone(),
        })])
    } else if state
        .all_relevant_cells()
        .iter()
        .any(|c| c.cell.id() == operation.updated_cell.id())
    {
        Err(Error::DuplicateId(operation.updated_cell.id().clone()))
    } else {
        // If the ID changed, we remove the old cell and insert the new one at the old index.
        // This is necessary to let remove and merge cell operations converge:
        let index = state
            .cell_with_index(operation.old_cell.id())
            .ok_or_else(|| Error::CellNotFound(operation.old_cell.id().clone()))?
            .index;

        Ok(vec![
            Change::DeleteCell(DeleteCellChange {
                cell_id: operation.old_cell.id().clone(),
            }),
            Change::InsertCell(InsertCellChange {
                cell: operation.updated_cell.as_ref().clone(),
                index,
            }),
        ])
    }
}

fn apply_update_notebook_time_range(
    _: &dyn ApplyOperationState,
    operation: &UpdateNotebookTimeRangeOperation,
) -> Vec<Change> {
    let UpdateNotebookTimeRangeOperation { time_range, .. } = operation;
    vec![Change::UpdateNotebookTimeRange(
        UpdateNotebookTimeRangeChange {
            time_range: time_range.clone(),
        },
    )]
}

fn apply_update_notebook_title(
    _: &dyn ApplyOperationState,
    operation: &UpdateNotebookTitleOperation,
) -> Vec<Change> {
    let UpdateNotebookTitleOperation { title, .. } = operation;
    vec![Change::UpdateNotebookTitle(UpdateNotebookTitleChange {
        title: title.clone(),
    })]
}

fn apply_add_data_source_operation(
    _: &dyn ApplyOperationState,
    operation: &AddDataSourceOperation,
) -> Vec<Change> {
    vec![Change::AddDataSource(AddDataSourceChange {
        name: operation.name.clone(),
        data_source: operation.data_source.clone(),
    })]
}

fn apply_update_data_source_operation(
    _: &dyn ApplyOperationState,
    operation: &UpdateDataSourceOperation,
) -> Vec<Change> {
    vec![Change::UpdateDataSource(UpdateDataSourceChange {
        name: operation.name.clone(),
        data_source: operation.data_source.clone(),
    })]
}

fn apply_remove_data_source_operation(
    _: &dyn ApplyOperationState,
    operation: &RemoveDataSourceOperation,
) -> Vec<Change> {
    vec![Change::DeleteDataSource(DeleteDataSourceChange {
        name: operation.name.clone(),
    })]
}

fn apply_add_label_operation(
    _: &dyn ApplyOperationState,
    operation: &AddLabelOperation,
) -> Vec<Change> {
    vec![Change::AddLabel(AddLabelChange {
        label: operation.label.clone(),
    })]
}

fn apply_replace_label_operation(
    _: &dyn ApplyOperationState,
    operation: &ReplaceLabelOperation,
) -> Vec<Change> {
    vec![Change::ReplaceLabel(ReplaceLabelChange {
        key: operation.old_label.key.clone(),
        label: operation.new_label.clone(),
    })]
}

fn apply_remove_label_operation(
    _: &dyn ApplyOperationState,
    operation: &RemoveLabelOperation,
) -> Vec<Change> {
    vec![Change::RemoveLabel(RemoveLabelChange {
        label: operation.label.clone(),
    })]
}

pub fn char_count<T>(text: &T) -> u32
where
    T: Borrow<str> + ?Sized,
{
    text.borrow().chars().count() as u32
}

fn get_change_for_dropped_reference(
    referencing_cell: &CellWithIndex,
    source_ids: Vec<&str>,
) -> Change {
    if source_ids.is_empty() {
        Change::DeleteCell(DeleteCellChange {
            cell_id: referencing_cell.cell.id().clone(),
        })
    } else {
        Change::UpdateCell(UpdateCellChange {
            cell: referencing_cell
                .cell
                .with_source_ids(source_ids.into_iter().map(String::from).collect()),
        })
    }
}

pub fn replace_text(text: &str, operation: &ReplaceTextOperation) -> String {
    text.chars()
        .take(operation.offset as usize)
        .chain(operation.new_text.chars())
        .chain(
            text.chars()
                .skip((operation.offset + char_count(&operation.old_text)) as usize),
        )
        .collect()
}
