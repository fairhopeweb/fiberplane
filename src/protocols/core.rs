use serde::{Deserialize, Serialize};
use std::{collections::HashMap, usize};

/// Representation of a single notebook cell.
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
    /// Returns the cell's content, if any.
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

    /// Returns the cell's ID.
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

    /// Returns whether the cell is an output cell.
    pub fn is_output_cell(&self) -> bool {
        matches!(self, Cell::Graph(_) | Cell::Table(_))
    }

    /// Returns a copy of the cell with the given content appended.
    pub fn with_appended_content(&self, content: &str) -> Self {
        self.with_content(&format!("{}{}", self.content().unwrap_or(""), content))
    }

    /// Returns a copy of the cell with its content replaced by the given content.
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

#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GraphCell {
    pub id: String,
    pub graph_type: GraphType,
    pub source_ids: Vec<String>,
    pub time_range: Option<TimeRange>,
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

/// A special role that can be assigned to certain cells, giving it unique capabilities.
#[derive(Clone, Copy, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CellRole {
    /// A cell with the Title role will cause the notebook title to be updated when its content is
    /// updated.
    Title,
}

#[derive(Clone, Copy, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GraphType {
    Bar,
    Line,
}

#[derive(Clone, Copy, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum HeadingType {
    H1,
    H2,
    H3,
}

// A range in time from a given timestamp (inclusive) up to another timestamp (exclusive).
#[derive(Clone, Deserialize, Serialize)]
pub struct TimeRange {
    pub from: Timestamp,
    pub to: Timestamp,
}

/// Timestamp specified in seconds since the UNIX epoch, with subsecond precision.
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

#[derive(Clone, Copy, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PointType {
    F64,
    String,
}

/// A single data-point in time, with meta-data about the metric it was taken from.
#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Instant<T> {
    pub metric: Metric,
    pub point: Point<T>,
    point_type: PointType,
}

impl<T> Instant<T> {
    pub fn point_type(&self) -> PointType {
        self.point_type
    }
}

impl Instant<f64> {
    pub fn new_f64(metric: Metric, point: Point<f64>) -> Self {
        Self {
            metric,
            point,
            point_type: PointType::F64,
        }
    }
}

impl Instant<String> {
    pub fn new_string(metric: Metric, point: Point<String>) -> Self {
        Self {
            metric,
            point,
            point_type: PointType::String,
        }
    }
}

pub type InstantsBySourceId<T> = HashMap<String, Vec<Instant<T>>>;

/// A series of data-points in time, with meta-data about the metric it was taken from.
#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Series<T> {
    pub metric: Metric,
    pub points: Vec<Point<T>>,
    point_type: PointType,
}

impl<T> Series<T> {
    pub fn point_type(&self) -> PointType {
        self.point_type
    }
}

impl Series<f64> {
    pub fn new_f64(metric: Metric, points: Vec<Point<f64>>) -> Self {
        Self {
            metric,
            points,
            point_type: PointType::F64,
        }
    }
}

impl Series<String> {
    pub fn new_string(metric: Metric, points: Vec<Point<String>>) -> Self {
        Self {
            metric,
            points,
            point_type: PointType::String,
        }
    }
}

pub type SeriesBySourceId<T> = HashMap<String, Vec<Series<T>>>;
