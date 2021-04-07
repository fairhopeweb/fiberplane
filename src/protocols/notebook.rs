use crate::protocols::core::{Cell, TimeRange};
use serde::{Deserialize, Serialize};

/// A notebook with all (meta)data included.
#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Notebook {
    pub id: String,
    pub cells: Vec<Cell>,
    pub revision: usize,
    pub time_range: TimeRange,
    pub title: String,
}
