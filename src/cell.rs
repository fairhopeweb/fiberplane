use std::collections::HashMap;
use std::usize;

use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CellRole {
    Title,
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GraphType {
    Bar,
    Line,
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum HeadingType {
    H1,
    H2,
    H3,
}

#[derive(Clone, Deserialize, Serialize)]
// A range in time from a given timestamp (inclusive) up to another timestamp (exclusive).
pub struct TimeRange {
    pub from: Timestamp,
    pub to: Timestamp,
}

/// Timestamp specified in seconds, with subsecond precision.
pub type Timestamp = f64;

#[derive(Clone, Deserialize, Serialize)]
pub struct Metric {
    pub name: String,
    pub labels: HashMap<String, String>,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Point<T> {
    pub timestamp: Timestamp,
    pub value: T,
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PointType {
    F64,
    String,
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Instant<T> {
    pub metric: Metric,
    pub point: Point<T>,
    pub point_type: PointType,
}

pub type InstantsBySourceId<T> = HashMap<String, Vec<Instant<T>>>;

#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Series<T> {
    pub metric: Metric,
    pub point_type: PointType,
    pub points: Vec<Point<T>>,
}

pub type SeriesBySourceId<T> = HashMap<String, Vec<Series<T>>>;

#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GraphCell {
    pub id: String,
    pub graph_type: GraphType,
    pub source_ids: Vec<String>,
    pub time_range: TimeRange,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<SeriesBySourceId<f64>>,
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HeadingCell {
    pub id: String,
    pub heading_type: HeadingType,
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<CellRole>,
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MarkdownCell {
    pub id: String,
    pub content: String,
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PrometheusCell {
    pub id: String,
    pub content: String,
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TableCell {
    pub id: String,
    pub source_ids: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<InstantsBySourceId<f64>>,
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TextCell {
    pub id: String,
    pub content: String,
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Cell {
    Graph(GraphCell),
    Heading(HeadingCell),
    Markdown(MarkdownCell),
    Prometheus(PrometheusCell),
    Table(TableCell),
    Text(TextCell),
}

impl Cell {
    pub fn content(&self) -> Option<&str> {
        match self {
            Cell::Graph(_) => None,
            Cell::Heading(cell) => Some(&cell.content),
            Cell::Markdown(cell) => Some(&cell.content),
            Cell::Prometheus(cell) => Some(&cell.content),
            Cell::Table(_) => None,
            Cell::Text(cell) => Some(&cell.content),
        }
    }

    pub fn id(&self) -> &String {
        match self {
            Cell::Graph(cell) => &cell.id,
            Cell::Heading(cell) => &cell.id,
            Cell::Markdown(cell) => &cell.id,
            Cell::Prometheus(cell) => &cell.id,
            Cell::Table(cell) => &cell.id,
            Cell::Text(cell) => &cell.id,
        }
    }

    pub fn is_output_cell(&self) -> bool {
        match self {
            Cell::Graph { .. } => true,
            Cell::Table { .. } => true,
            _ => false,
        }
    }

    pub fn with_appended_content(&self, content: &str) -> Self {
        self.with_content(&format!("{}{}", self.content().unwrap_or(""), content))
    }

    pub fn with_content(&self, content: &str) -> Self {
        match self {
            Cell::Graph(cell) => Cell::Graph(cell.clone()),
            Cell::Heading(cell) => Cell::Heading(HeadingCell {
                content: content.to_owned(),
                ..cell.clone()
            }),
            Cell::Markdown(cell) => Cell::Markdown(MarkdownCell {
                content: content.to_owned(),
                ..cell.clone()
            }),
            Cell::Prometheus(cell) => Cell::Prometheus(PrometheusCell {
                content: content.to_owned(),
                ..cell.clone()
            }),
            Cell::Table(cell) => Cell::Table(cell.clone()),
            Cell::Text(cell) => Cell::Text(TextCell {
                content: content.to_owned(),
                ..cell.clone()
            }),
        }
    }
}
