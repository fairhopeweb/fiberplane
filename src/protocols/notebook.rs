use crate::protocols::core::{Cell, TimeRange};

/// A notebook with all (meta)data included.
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Notebook {
    pub id: String,
    pub cells: Vec<Cell>,
    pub time_range: TimeRange,
    pub title: String,
}
