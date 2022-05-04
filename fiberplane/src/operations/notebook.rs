use crate::operations::{ApplyOperationState, CellRefWithIndex};
pub use crate::protocols::core::{Notebook, NotebookVisibility};

impl ApplyOperationState for Notebook {
    fn all_cell_ids(&self) -> Vec<&str> {
        self.cells.iter().map(|cell| cell.id().as_str()).collect()
    }

    fn all_relevant_cells(&self) -> Vec<CellRefWithIndex> {
        self.cells
            .iter()
            .enumerate()
            .map(|(index, cell)| CellRefWithIndex {
                cell,
                index: index as u32,
            })
            .collect()
    }
}
