use crate::protocols::operations::CellWithIndex;
use serde::{Deserialize, Serialize};

/// State of a notebook to apply an operation to. This differs from a full `Notebook` in that it
/// only needs to include the minimal necessary state for an operation to be converted into changes
/// by the Mill.
///
/// Users of this struct are responsible for making sure all cells that are relevant to a given
/// operation are included. A naive implementation may simply include all of a notebook's cells.
#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct NotebookState {
    pub cells: Vec<CellWithIndex>,
}
