use serde::{Deserialize, Serialize};
use std::{collections::HashMap, usize};

/// Representation of a single notebook cell.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
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

    /// Returns all the source IDs referenced by the cell.
    pub fn source_ids(&self) -> Vec<&str> {
        match self {
            Cell::Graph(cell) => cell.source_ids.iter().map(String::as_str).collect(),
            Cell::Table(cell) => cell.source_ids.iter().map(String::as_str).collect(),
            Cell::Heading(_) | Cell::Markdown(_) | Cell::Prometheus(_) | Cell::Text(_) => vec![],
        }
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
                id: cell.id.clone(),
                content: content.to_owned(),
                ..*cell
            }),
            Cell::Markdown(cell) => Cell::Markdown(MarkdownCell {
                id: cell.id.clone(),
                content: content.to_owned(),
                ..*cell
            }),
            Cell::Prometheus(cell) => Cell::Prometheus(PrometheusCell {
                id: cell.id.clone(),
                content: content.to_owned(),
                ..*cell
            }),
            Cell::Table(cell) => Cell::Table(cell.clone()),
            Cell::Text(cell) => Cell::Text(TextCell {
                id: cell.id.clone(),
                content: content.to_owned(),
                ..*cell
            }),
        }
    }

    /// Returns a copy of the cell with its source IDs replaced by the given IDs.
    ///
    /// If the cell contains any data, only data that belongs to any of the new
    /// source IDs is retained.
    pub fn with_source_ids(&self, source_ids: Vec<String>) -> Self {
        match self {
            Cell::Graph(cell) => Cell::Graph(GraphCell {
                id: cell.id.clone(),
                data: cell.data.as_ref().map(|data| {
                    data.iter()
                        .filter(|(k, _)| source_ids.contains(k))
                        .map(|(k, v)| (k.clone(), v.clone()))
                        .collect()
                }),
                source_ids,
                time_range: cell.time_range.clone(),
                title: cell.title.clone(),
                ..*cell
            }),
            Cell::Heading(cell) => Cell::Heading(cell.clone()),
            Cell::Markdown(cell) => Cell::Markdown(cell.clone()),
            Cell::Prometheus(cell) => Cell::Prometheus(cell.clone()),
            Cell::Table(cell) => Cell::Table(TableCell {
                id: cell.id.clone(),
                data: cell.data.as_ref().map(|data| {
                    data.iter()
                        .filter(|(k, _)| source_ids.contains(k))
                        .map(|(k, v)| (k.clone(), v.clone()))
                        .collect()
                }),
                source_ids,
                ..*cell
            }),
            Cell::Text(cell) => Cell::Text(cell.clone()),
        }
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GraphCell {
    pub id: String,
    pub graph_type: GraphType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    pub source_ids: Vec<String>,
    pub time_range: Option<TimeRange>,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<SeriesBySourceId<f64>>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HeadingCell {
    pub id: String,
    pub heading_type: HeadingType,
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MarkdownCell {
    pub id: String,
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PrometheusCell {
    pub id: String,
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TableCell {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    pub source_ids: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<InstantsBySourceId<f64>>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TextCell {
    pub id: String,
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GraphType {
    Bar,
    Line,
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum HeadingType {
    H1,
    H2,
    H3,
}

// A range in time from a given timestamp (inclusive) up to another timestamp (exclusive).
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct TimeRange {
    pub from: Timestamp,
    pub to: Timestamp,
}

/// Timestamp specified in seconds since the UNIX epoch, with subsecond precision.
pub type Timestamp = f64;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Metric {
    pub name: String,
    pub labels: HashMap<String, String>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Point<T> {
    pub timestamp: Timestamp,
    pub value: T,
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PointType {
    F64,
    String,
}

/// A single data-point in time, with meta-data about the metric it was taken from.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
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
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
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
