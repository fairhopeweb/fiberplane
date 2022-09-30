use crate::{
    operations::{
        apply_operation, changes::*, error::*, notebook::Notebook, relevant_cell_ids_for_operation,
        ApplyOperationState, CellRefWithIndex, TransformOperationState,
    },
    protocols::{core::*, data_sources::SelectedDataSource, operations::*},
};
use std::collections::BTreeMap;

impl Notebook {
    /// Applies the given operation to this notebook.
    pub fn apply_operation(&self, operation: &Operation) -> Result<Self, Error> {
        Ok(self.clone().apply_changes(apply_operation(
            &self.state_for_operation(operation),
            operation,
        )?))
    }

    fn apply_changes(self, changes: Vec<Change>) -> Self {
        let mut notebook = self;
        for change in changes.into_iter() {
            notebook = notebook.apply_change(change);
        }
        notebook
    }

    fn apply_change(mut self, change: Change) -> Self {
        use Change::*;
        match change {
            DeleteCell(DeleteCellChange { cell_id }) => self.with_updated_cells(|cells| {
                cells.retain(|cell| *cell.id() != cell_id);
            }),
            InsertCell(InsertCellChange { cell, index }) => self.with_updated_cells(|cells| {
                cells.insert(index as usize, cell);
            }),
            MoveCells(MoveCellsChange { cell_ids, index }) => self.with_updated_cells(|cells| {
                cell_ids.iter().enumerate().for_each(|(i, cell_id)| {
                    if let Some(old_index) = cells.iter().position(|c| c.id() == cell_id) {
                        let cell = cells.remove(old_index);
                        let new_index = index as usize + i;
                        cells.insert(new_index, cell);
                    }
                });
            }),
            UpdateCell(UpdateCellChange { cell }) => self.with_updated_cells(|cells| {
                if let Some(index) = cells.iter().position(|c| c.id() == cell.id()) {
                    cells[index] = cell;
                }
            }),
            UpdateCellText(UpdateCellTextChange {
                cell_id,
                field,
                formatting,
                text,
            }) => self.with_updated_cells(|cells| {
                if let Some(index) = cells.iter().position(|cell| cell.id() == cell_id) {
                    cells[index] =
                        cells[index].with_text_for_field(&text, formatting, field.as_deref());
                }
            }),
            UpdateNotebookTimeRange(UpdateNotebookTimeRangeChange { time_range }) => {
                Self { time_range, ..self }
            }
            UpdateNotebookTitle(UpdateNotebookTitleChange { title }) => Self { title, ..self },
            SetSelectedDataSource(SetSelectedDataSourceChange {
                provider_type,
                selected_data_source,
            }) => {
                if let Some(selected_data_source) = selected_data_source {
                    self.selected_data_sources
                        .insert(provider_type, selected_data_source);
                } else {
                    self.selected_data_sources.remove(&provider_type);
                }
                self
            }
            AddLabel(change) => {
                self.labels.push(change.label);
                self
            }
            ReplaceLabel(change) => {
                if let Some(label) = self.labels.iter_mut().find(|label| label.key == change.key) {
                    *label = change.label
                };
                self
            }
            RemoveLabel(change) => {
                self.labels.retain(|label| *label.key != change.label.key);
                self
            }
        }
    }

    pub fn clone_cell_with_index_by_id(&self, id: &str) -> CellWithIndex {
        self.cells
            .iter()
            .enumerate()
            .find(|(_, cell)| cell.id() == id)
            .map(|(index, cell)| CellWithIndex {
                cell: cell.clone(),
                index: index as u32,
            })
            .expect("No cell found with that ID")
    }

    /// Returns the notebook state with all the cells necessary for applying the given operation
    /// to it.
    fn state_for_operation(&self, operation: &Operation) -> NotebookState {
        let cell_ids = self.cells.iter().map(Cell::id).collect();
        let relevant_cell_ids = relevant_cell_ids_for_operation(operation);
        NotebookState {
            cell_ids,
            relevant_cells: self
                .cells
                .iter()
                .enumerate()
                .filter(|(_, cell)| relevant_cell_ids.contains(&cell.id()))
                .map(|(index, cell)| CellRefWithIndex {
                    cell,
                    index: index as u32,
                })
                .collect(),
        }
    }

    /// Returns a copy of the notebook with updated cells.
    pub fn with_updated_cells<F>(&self, updater: F) -> Self
    where
        F: FnOnce(&mut Vec<Cell>),
    {
        let mut clone = self.clone();
        updater(&mut clone.cells);
        clone
    }

    /// Returns a copy of the notebook with updated cells.
    pub fn with_updated_selected_data_sources<F>(&self, updater: F) -> Self
    where
        F: FnOnce(&mut BTreeMap<String, SelectedDataSource>),
    {
        let mut clone = self.clone();
        updater(&mut clone.selected_data_sources);
        clone
    }

    /// Returns a copy of the notebook with updated labels.
    pub fn with_updated_labels<F>(&self, updater: F) -> Self
    where
        F: FnOnce(&mut Vec<Label>),
    {
        let mut clone = self.clone();
        updater(&mut clone.labels);
        clone
    }
}

impl TransformOperationState for Notebook {
    fn cell(&self, id: &str) -> Option<&Cell> {
        self.cells.iter().find(|cell| cell.id() == id)
    }

    fn cell_index(&self, id: &str) -> Option<u32> {
        self.cells
            .iter()
            .position(|cell| cell.id() == id)
            .map(|index| index as u32)
    }
}

struct NotebookState<'a> {
    cell_ids: Vec<&'a str>,
    relevant_cells: Vec<CellRefWithIndex<'a>>,
}

// Note: It would be easier to just use the trait implementation of `Notebook`, but the reason I'm
// still sticking with a separate struct is so that we test `relevant_cell_ids_for_operation()` in
// the process.
impl<'a> ApplyOperationState for NotebookState<'a> {
    fn all_cell_ids(&self) -> Vec<&str> {
        self.cell_ids.clone()
    }

    fn all_relevant_cells(&self) -> Vec<CellRefWithIndex> {
        self.relevant_cells.clone()
    }
}
