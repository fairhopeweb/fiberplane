use super::utils::is_annotation_included_in_formatting;
use crate::{
    operations::{changes::*, error::*},
    protocols::{core::*, formatting::*, operations::*},
    query_data::get_query_field,
    text_util::{char_count, char_slice, char_slice_from},
};
use std::{borrow::Cow, cmp::Ordering};

#[derive(Clone, Debug, PartialEq)]
pub struct CellRefWithIndex<'a> {
    pub cell: &'a Cell,
    pub index: u32,
}

impl<'a> CellRefWithIndex<'a> {
    pub fn formatting(&self) -> Option<&Formatting> {
        self.cell.formatting()
    }

    pub fn id(&self) -> &str {
        self.cell.id()
    }

    pub fn text(&self) -> Option<&str> {
        self.cell.text()
    }
}

/// Allows `apply_operation()` to query for the cells that may be affected by a single operation.
///
/// See `relevant_cell_ids_for_operation()` if you want to know how to determine which cells may be
/// affected by an operation.
pub trait ApplyOperationState {
    /// Returns the IDs of all the cells in the notebook.
    ///
    /// This includes IDs for cells that are not relevant to the operation itself.
    fn all_cell_ids(&self) -> Vec<&str>;

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

    /// Returns the text for the cell with the given ID with an optional field.
    fn cell_text_and_formatting(
        &self,
        id: &str,
        field: Option<&str>,
    ) -> Option<(Cow<str>, Option<&Formatting>)> {
        self.cell(id)
            .and_then(|cell| text_and_formatting_for_cell_and_field(cell, field))
    }

    /// Returns a cell by ID, plus the index of that cell in the notebook.
    ///
    /// May return `None` if the cell exists, but was considered not relevant for the operation.
    fn cell_with_index(&self, id: &str) -> Option<CellRefWithIndex> {
        self.all_relevant_cells()
            .into_iter()
            .find(|cell| cell.id() == id)
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
        MoveCells(operation) => Ok(apply_move_cells_operation(state, operation)),
        ReplaceCells(operation) => apply_replace_cells_operation(state, operation),
        ReplaceText(operation) => apply_replace_text_operation(state, operation),
        UpdateNotebookTimeRange(operation) => {
            Ok(apply_update_notebook_time_range(state, operation))
        }
        UpdateNotebookTitle(operation) => Ok(apply_update_notebook_title(state, operation)),
        SetSelectedDataSource(operation) => Ok(apply_set_selected_data_source(state, operation)),
        AddLabel(operation) => Ok(apply_add_label_operation(state, operation)),
        ReplaceLabel(operation) => Ok(apply_replace_label_operation(state, operation)),
        RemoveLabel(operation) => Ok(apply_remove_label_operation(state, operation)),
    }
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

fn apply_replace_cells_operation(
    state: &dyn ApplyOperationState,
    operation: &ReplaceCellsOperation,
) -> Result<Vec<Change>, Error> {
    let mut new_cells = operation
        .new_cells
        .iter()
        .enumerate()
        .map(|(index, cell)| {
            // The split offset is only used for the first cell,
            // so use `None` otherwise.
            let split_offset = if index == 0 {
                operation.split_offset
            } else {
                None
            };
            // The merge offset is only used for the last cell,
            // so use `None` otherwise.
            let merge_offset = if index == operation.new_cells.len() - 1 {
                operation.merge_offset
            } else {
                None
            };

            let cell = if let Some(split_offset) = split_offset {
                let first_old_cell = operation.old_cells.first().ok_or_else(|| {
                    Error::InternalError("old cells should not be empty".to_owned())
                })?;
                let first_original_cell = state
                    .cell(first_old_cell.id())
                    .ok_or_else(|| Error::CellNotFound(first_old_cell.id().to_owned()))?;

                // If there is a split offset, the text of the actual new cell
                // (`new_text`) will be a concatenation of the text of the
                // original cell (up to the split offset) and the text provided
                // in the operation (assigned to `new_cell_text`).
                let new_cell_text = cell.text().unwrap_or_default();
                let new_text = format!(
                    "{}{}{}",
                    first_original_cell
                        .text()
                        .map(|text| char_slice(text, 0, split_offset))
                        .unwrap_or_default(),
                    new_cell_text,
                    // Be careful, if there is only one new cell, we also need
                    // to take the merge offset into account.
                    if let Some(merge_offset) = merge_offset {
                        operation
                            .old_cells
                            .last()
                            .map(CellWithIndex::id)
                            .and_then(|cell_id| state.cell(cell_id))
                            .and_then(Cell::text)
                            .map(|text| char_slice_from(text, merge_offset))
                            .unwrap_or_default()
                    } else {
                        ""
                    }
                );
                if cell.cell.supports_formatting() {
                    // For formatting, things are even a little bit trickier.
                    // New formatting is a concatenation of original formatting
                    // and newly provided formatting, similar to how `new_text`
                    // was created. However, for formatting that is exactly at
                    // the split offset, we need to look at which annotations
                    // are provided in the operation's matching old cell
                    // formatting. Annotations that are provided in the
                    // operation are stripped, while those that are not are
                    // kept.
                    let old_formatting = first_old_cell.formatting();
                    let mut new_formatting: Formatting = first_original_cell
                        .formatting()
                        .map(|formatting| {
                            formatting
                                .iter()
                                .filter(|annotation| match annotation.offset.cmp(&split_offset) {
                                    Ordering::Less => true,
                                    Ordering::Equal => !old_formatting
                                        .map(|old_formatting| {
                                            is_annotation_included_in_formatting(
                                                &annotation.annotation,
                                                annotation.offset - split_offset,
                                                old_formatting,
                                            )
                                        })
                                        .unwrap_or_default(),
                                    Ordering::Greater => false,
                                })
                                .cloned()
                                .collect()
                        })
                        .unwrap_or_default();
                    if let Some(formatting) = cell.formatting() {
                        for annotation in formatting.iter() {
                            new_formatting.push(annotation.translate(split_offset as i64));
                        }
                    }
                    // And again, if there is only one new cell, merge offset
                    // should be considered too.
                    if let (Some(merge_offset), Some(formatting)) = (
                        merge_offset,
                        operation
                            .old_cells
                            .last()
                            .map(CellWithIndex::id)
                            .and_then(|cell_id| state.cell(cell_id))
                            .and_then(Cell::formatting),
                    ) {
                        let old_formatting = operation
                            .old_cells
                            .last()
                            .and_then(CellWithIndex::formatting);
                        let delta =
                            (split_offset + char_count(new_cell_text)) as i64 - merge_offset as i64;
                        for annotation in formatting.iter().filter(|annotation| {
                            match annotation.offset.cmp(&merge_offset) {
                                Ordering::Greater => true,
                                Ordering::Equal => !old_formatting
                                    .map(|old_formatting| {
                                        if split_offset <= merge_offset {
                                            is_annotation_included_in_formatting(
                                                &annotation.annotation,
                                                merge_offset - split_offset,
                                                old_formatting,
                                            )
                                        } else {
                                            false
                                        }
                                    })
                                    .unwrap_or_default(),
                                Ordering::Less => false,
                            }
                        }) {
                            new_formatting.push(annotation.translate(delta));
                        }
                    }
                    CellWithIndex {
                        cell: cell.cell.with_rich_text(&new_text, new_formatting),
                        index: cell.index,
                    }
                } else {
                    CellWithIndex {
                        cell: cell.cell.with_text(&new_text),
                        index: cell.index,
                    }
                }
            } else if let Some(merge_offset) = merge_offset {
                let last_old_cell = operation.old_cells.last().ok_or_else(|| {
                    Error::InternalError("old cells should not be empty".to_owned())
                })?;
                let last_original_cell = state
                    .cell(last_old_cell.id())
                    .ok_or_else(|| Error::CellNotFound(last_old_cell.id().to_owned()))?;

                // If there is a merge offset, the text of the actual new cell
                // (`new_text`) will be a concatenation of the text provided in
                // the operation (`new_cell_text`) and the text of the original
                // cell (from the merge offset onwards).
                let new_cell_text = cell.text().unwrap_or_default();
                let new_text = format!(
                    "{}{}",
                    new_cell_text,
                    last_original_cell
                        .text()
                        .map(|text| char_slice_from(text, merge_offset))
                        .unwrap_or_default()
                );
                if cell.cell.supports_formatting() {
                    // For formatting, we also need to consider whether
                    // annotations are included in the operation's old
                    // formatting again (see above).
                    let mut new_formatting: Formatting = Vec::new();
                    if let Some(formatting) = cell.formatting() {
                        for annotation in formatting.iter() {
                            new_formatting.push(annotation.clone());
                        }
                    }
                    if let Some(formatting) = last_original_cell.formatting() {
                        let old_formatting = last_old_cell.formatting();
                        let delta = char_count(new_cell_text) as i64 - merge_offset as i64;
                        for annotation in formatting.iter().filter(|annotation| {
                            match annotation.offset.cmp(&merge_offset) {
                                Ordering::Greater => true,
                                Ordering::Equal => !old_formatting
                                    .map(|old_formatting| {
                                        is_annotation_included_in_formatting(
                                            &annotation.annotation,
                                            merge_offset
                                                - if operation.old_cells.len() == 1 {
                                                    operation.split_offset.unwrap_or_default()
                                                } else {
                                                    0
                                                },
                                            old_formatting,
                                        )
                                    })
                                    .unwrap_or_default(),
                                Ordering::Less => false,
                            }
                        }) {
                            new_formatting.push(annotation.translate(delta));
                        }
                    }
                    CellWithIndex {
                        cell: cell.cell.with_rich_text(&new_text, new_formatting),
                        index: cell.index,
                    }
                } else {
                    CellWithIndex {
                        cell: cell.cell.with_text(&new_text),
                        index: cell.index,
                    }
                }
            } else {
                cell.clone()
            };

            Ok(cell)
        })
        .collect::<Result<Vec<CellWithIndex>, Error>>()?;
    for new_referencing_cell in operation.new_referencing_cells.iter() {
        new_cells.push(new_referencing_cell.clone());
    }

    let mut changes: Vec<Change> = Vec::new();

    for cell in operation.old_cells.iter() {
        if let Some((index, new_cell)) = new_cells
            .iter()
            .enumerate()
            .find(|(_, new_cell)| new_cell.id() == cell.id())
        {
            changes.push(Change::UpdateCell(UpdateCellChange {
                cell: new_cell.cell.clone(),
            }));
            new_cells.remove(index);
        } else {
            changes.push(Change::DeleteCell(DeleteCellChange {
                cell_id: cell.id().to_owned(),
            }));
        }
    }
    for cell in operation.old_referencing_cells.iter() {
        if let Some((index, new_cell)) = new_cells
            .iter()
            .enumerate()
            .find(|(_, new_cell)| new_cell.id() == cell.id())
        {
            changes.push(Change::UpdateCell(UpdateCellChange {
                cell: new_cell.cell.clone(),
            }));
            new_cells.remove(index);
        } else {
            changes.push(Change::DeleteCell(DeleteCellChange {
                cell_id: cell.id().to_owned(),
            }));
        }
    }

    for cell in new_cells {
        changes.push(Change::InsertCell(InsertCellChange {
            cell: cell.cell.clone(),
            index: cell.index,
        }));
    }

    Ok(changes)
}

fn apply_replace_text_operation(
    state: &dyn ApplyOperationState,
    operation: &ReplaceTextOperation,
) -> Result<Vec<Change>, Error> {
    let (text, formatting) = state
        .cell_text_and_formatting(&operation.cell_id, operation.field.as_deref())
        .ok_or_else(|| Error::NoTextCell(operation.cell_id.clone()))?;

    let old_text_len = char_count(&operation.old_text);
    Ok(vec![Change::UpdateCellText(UpdateCellTextChange {
        cell_id: operation.cell_id.clone(),
        field: operation.field.clone(),
        text: replace_text(&text, &operation.new_text, operation.offset, old_text_len),
        formatting: Some(replace_formatting(
            formatting,
            operation.old_formatting.as_ref(),
            operation.new_formatting.as_ref(),
            operation.offset,
            old_text_len,
            char_count(&operation.new_text),
        )),
    })])
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

fn apply_set_selected_data_source(
    _: &dyn ApplyOperationState,
    operation: &SetSelectedDataSourceOperation,
) -> Vec<Change> {
    let SetSelectedDataSourceOperation {
        provider_type,
        new_selected_data_source,
        ..
    } = operation;
    vec![Change::SetSelectedDataSource(SetSelectedDataSourceChange {
        provider_type: provider_type.clone(),
        selected_data_source: new_selected_data_source.clone(),
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

/// Returns the text and formatting for the given cell and the optional field.
///
/// This handles cell-specific logic for retrieving content out of fields.
pub fn text_and_formatting_for_cell_and_field<'a, 'b>(
    cell: &'a Cell,
    field: Option<&'b str>,
) -> Option<(Cow<'a, str>, Option<&'a Formatting>)> {
    match (cell, field) {
        (Cell::Provider(cell), Some(field)) => Some(cell.query_data.as_ref().map_or_else(
            || (Cow::Borrowed(""), None),
            |query_data| (get_query_field(query_data, field), None),
        )),
        (cell, _) => cell
            .text()
            .map(|text| (Cow::Borrowed(text), cell.formatting())),
    }
}

/// Performs a formatting replacement within the given `formatting`. The
/// replacement will start at `offset`, remove the given `old_formatting`
/// (which may contain offsets from `0` through `old_text_len`), and replace
/// it with `new_formatting` (which may contain offsets from `0` through
/// `new_text_len`).
pub fn replace_formatting(
    formatting: Option<&Formatting>,
    old_formatting: Option<&Formatting>,
    new_formatting: Option<&Formatting>,
    offset: u32,
    old_text_len: u32,
    new_text_len: u32,
) -> Formatting {
    let formatting = if let Some(old_formatting) = old_formatting {
        formatting
            .map(|formatting| {
                formatting
                    .iter()
                    .filter(|annotation| {
                        annotation.offset < offset
                            || annotation.offset > offset + old_text_len
                            || !is_annotation_included_in_formatting(
                                &annotation.annotation,
                                annotation.offset - offset,
                                old_formatting,
                            )
                    })
                    .cloned()
                    .collect()
            })
            .unwrap_or_default()
    } else {
        formatting.cloned().unwrap_or_default()
    };

    // We split the formatting at the index *beyond* the offset, so that no
    // formatting is lost unless explicitly included in the `old_formatting`.
    let split_index = first_annotation_index_beyond_offset(&formatting, offset);
    let merge_index = if old_text_len == 0 {
        // We continue from the split index, or annotations that are *at* the
        // index would get duplicated.
        split_index
    } else {
        // If we removed text, we continue *before* the offset from where the
        // text continues, to avoid losing formatting again.
        first_annotation_index_for_offset(&formatting, offset + old_text_len)
    };

    let delta = new_text_len as i64 - old_text_len as i64;
    [
        &formatting[0..split_index],
        &new_formatting
            .map(|formatting| translate(formatting, offset as i64))
            .unwrap_or_default(),
        &formatting
            .iter()
            .skip(merge_index)
            .map(|annotation| annotation.translate(delta))
            .collect::<Vec<_>>(),
    ]
    .concat()
}

/// Performs a string replacement within the given `text`. The replacement will
/// start at `offset`, remove the given `old_text_len` characters, and replace
/// them with `new_text`.
pub fn replace_text(text: &str, new_text: &str, offset: u32, old_text_len: u32) -> String {
    text.chars()
        .take(offset as usize)
        .chain(new_text.chars())
        .chain(text.chars().skip((offset + old_text_len) as usize))
        .collect()
}
