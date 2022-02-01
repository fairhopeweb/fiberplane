use fp_bindgen::prelude::Serializable;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::collections::HashMap;
use time::OffsetDateTime;

/// Validator for the prefix portion of a Label.
pub static LABEL_PREFIX_RE: Lazy<regex::Regex> = Lazy::new(|| {
    regex::Regex::new(r#"^[a-z0-9]([-a-z0-9]*[a-z0-9])?(\.[a-z0-9]([-a-z0-9]*[a-z0-9])?)*$"#)
        .unwrap()
});

/// Validator for the name and value portion of a Label.
pub static LABEL_NAME_VALUE_RE: Lazy<regex::Regex> =
    Lazy::new(|| regex::Regex::new(r#"^[a-z\dA-Z]([\w\-\.]*[a-z\dA-Z])?$"#).unwrap());

const MAX_LABEL_VALUE_LENGTH: usize = 63;
const MAX_LABEL_NAME_LENGTH: usize = 63;
const MAX_LABEL_PREFIX_LENGTH: usize = 253;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Serializable)]
#[fp(rust_plugin_module = "fiberplane::protocols::core")]
#[serde(rename_all = "camelCase")]
pub struct NewNotebook {
    pub title: String,
    pub cells: Vec<Cell>,
    pub time_range: TimeRange,

    #[serde(default)]
    pub data_sources: BTreeMap<String, NotebookDataSource>,

    #[serde(default)]
    pub labels: Vec<Label>,
}

impl From<Notebook> for NewNotebook {
    fn from(notebook: Notebook) -> Self {
        NewNotebook {
            title: notebook.title,
            cells: notebook.cells,
            time_range: notebook.time_range,
            data_sources: notebook.data_sources,
            labels: notebook.labels,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Serializable)]
#[fp(rust_plugin_module = "fiberplane::protocols::core")]
#[serde(rename_all = "snake_case")]
pub enum UserType {
    Anonymous,
    Individual,
    Organization,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Serializable)]
#[fp(rust_plugin_module = "fiberplane::protocols::core")]
#[serde(rename_all = "camelCase")]
pub struct CreatedBy {
    #[serde(rename = "type")]
    pub user_type: UserType,
    pub name: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Deserialize, Serialize, Serializable)]
#[fp(rust_plugin_module = "fiberplane::protocols::core")]
#[serde(rename_all = "snake_case")]
pub enum NotebookVisibility {
    Private,
    Public,
}

impl Default for NotebookVisibility {
    fn default() -> Self {
        Self::Private
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Serializable)]
#[fp(rust_plugin_module = "fiberplane::protocols::core")]
#[serde(rename_all = "camelCase")]
pub struct Notebook {
    pub id: String,
    #[serde(with = "time_util::serde_rfc3339")]
    pub created_at: OffsetDateTime,
    #[serde(with = "time_util::serde_rfc3339")]
    pub updated_at: OffsetDateTime,
    pub time_range: TimeRange,
    pub title: String,
    pub cells: Vec<Cell>,
    pub revision: u32,
    pub visibility: NotebookVisibility,
    pub read_only: bool,
    pub created_by: CreatedBy,

    #[serde(default)]
    pub data_sources: BTreeMap<String, NotebookDataSource>,

    #[serde(default)]
    pub labels: Vec<Label>,
}

/// Representation of a single notebook cell.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Serializable)]
#[fp(rust_plugin_module = "fiberplane::protocols::core")]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Cell {
    Checkbox(CheckboxCell),
    Code(CodeCell),
    Graph(GraphCell),
    Heading(HeadingCell),
    ListItem(ListItemCell),
    Prometheus(PrometheusCell),
    Elasticsearch(ElasticsearchCell),
    Table(TableCell),
    Log(LogCell),
    Text(TextCell),
    Image(ImageCell),
    Divider(DividerCell),
}

impl Cell {
    /// Returns the cell's content, if any.
    pub fn content(&self) -> Option<&str> {
        match self {
            Cell::Checkbox(cell) => Some(&cell.content),
            Cell::Code(cell) => Some(&cell.content),
            Cell::Graph(_) => None,
            Cell::Heading(cell) => Some(&cell.content),
            Cell::ListItem(cell) => Some(&cell.content),
            Cell::Log(_) => None,
            Cell::Prometheus(cell) => Some(&cell.content),
            Cell::Elasticsearch(cell) => Some(&cell.content),
            Cell::Table(_) => None,
            Cell::Text(cell) => Some(&cell.content),
            Cell::Image(_) => None,
            Cell::Divider(_) => None,
        }
    }

    /// Returns the cell's ID.
    pub fn id(&self) -> &String {
        match self {
            Cell::Checkbox(cell) => &cell.id,
            Cell::Code(cell) => &cell.id,
            Cell::Graph(cell) => &cell.id,
            Cell::Heading(cell) => &cell.id,
            Cell::ListItem(cell) => &cell.id,
            Cell::Log(cell) => &cell.id,
            Cell::Prometheus(cell) => &cell.id,
            Cell::Elasticsearch(cell) => &cell.id,
            Cell::Table(cell) => &cell.id,
            Cell::Text(cell) => &cell.id,
            Cell::Image(cell) => &cell.id,
            Cell::Divider(cell) => &cell.id,
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
            Cell::Log(cell) => cell.source_ids.iter().map(String::as_str).collect(),
            Cell::Table(cell) => cell.source_ids.iter().map(String::as_str).collect(),
            Cell::Checkbox(_)
            | Cell::Code(_)
            | Cell::Heading(_)
            | Cell::ListItem(_)
            | Cell::Prometheus(_)
            | Cell::Elasticsearch(_)
            | Cell::Text(_)
            | Cell::Image(_)
            | Cell::Divider(_) => vec![],
        }
    }

    /// Returns the cell's text, if any.
    pub fn text(&self) -> Option<&str> {
        match self {
            Cell::Graph(cell) => Some(&cell.title),
            cell => cell.content(),
        }
    }

    /// Returns a copy of the cell with the given content appended.
    #[must_use]
    pub fn with_appended_content(&self, content: &str) -> Self {
        self.with_content(&format!("{}{}", self.content().unwrap_or(""), content))
    }

    /// Returns a copy of the cell with its content replaced by the given content.
    #[must_use]
    pub fn with_content(&self, content: &str) -> Self {
        match self {
            Cell::Checkbox(cell) => Cell::Checkbox(CheckboxCell {
                id: cell.id.clone(),
                content: content.to_owned(),
                ..*cell
            }),
            Cell::Code(cell) => Cell::Code(CodeCell {
                id: cell.id.clone(),
                content: content.to_owned(),
                syntax: cell.syntax.clone(),
                ..*cell
            }),
            Cell::Graph(cell) => Cell::Graph(cell.clone()),
            Cell::Heading(cell) => Cell::Heading(HeadingCell {
                id: cell.id.clone(),
                content: content.to_owned(),
                ..*cell
            }),
            Cell::ListItem(cell) => Cell::ListItem(ListItemCell {
                id: cell.id.clone(),
                content: content.to_owned(),
                ..*cell
            }),
            Cell::Log(cell) => Cell::Log(cell.clone()),
            Cell::Prometheus(cell) => Cell::Prometheus(PrometheusCell {
                id: cell.id.clone(),
                content: content.to_owned(),
                ..*cell
            }),
            Cell::Elasticsearch(cell) => Cell::Elasticsearch(ElasticsearchCell {
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
            Cell::Image(cell) => Cell::Image(cell.clone()),
            Cell::Divider(cell) => Cell::Divider(cell.clone()),
        }
    }

    /// Returns a copy of the cell with a new ID.
    #[must_use]
    pub fn with_id(&self, id: &str) -> Self {
        match self {
            Cell::Checkbox(cell) => Cell::Checkbox(CheckboxCell {
                id: id.to_owned(),
                content: cell.content.clone(),
                ..*cell
            }),
            Cell::Code(cell) => Cell::Code(CodeCell {
                id: id.to_owned(),
                content: cell.content.clone(),
                syntax: cell.syntax.clone(),
                ..*cell
            }),
            Cell::Graph(cell) => Cell::Graph(GraphCell {
                id: id.to_owned(),
                stacking_type: cell.stacking_type,
                data: cell.data.clone(),
                source_ids: cell.source_ids.clone(),
                time_range: cell.time_range.clone(),
                title: cell.title.clone(),
                ..*cell
            }),
            Cell::Heading(cell) => Cell::Heading(HeadingCell {
                id: id.to_owned(),
                content: cell.content.clone(),
                ..*cell
            }),
            Cell::ListItem(cell) => Cell::ListItem(ListItemCell {
                id: id.to_owned(),
                content: cell.content.clone(),
                ..*cell
            }),
            Cell::Log(cell) => Cell::Log(LogCell {
                id: id.to_owned(),
                data: cell.data.clone(),
                source_ids: cell.source_ids.clone(),
                time_range: cell.time_range.clone(),
                ..*cell
            }),
            Cell::Prometheus(cell) => Cell::Prometheus(PrometheusCell {
                id: id.to_owned(),
                content: cell.content.clone(),
                ..*cell
            }),
            Cell::Elasticsearch(cell) => Cell::Elasticsearch(ElasticsearchCell {
                id: id.to_owned(),
                content: cell.content.clone(),
                ..*cell
            }),
            Cell::Table(cell) => Cell::Table(TableCell {
                id: id.to_owned(),
                data: cell.data.clone(),
                source_ids: cell.source_ids.clone(),
                ..*cell
            }),
            Cell::Text(cell) => Cell::Text(TextCell {
                id: id.to_owned(),
                content: cell.content.clone(),
                ..*cell
            }),
            Cell::Image(cell) => Cell::Image(ImageCell {
                id: id.to_owned(),
                file_id: cell.file_id.clone(),
                preview: cell.preview.clone(),
                url: cell.url.clone(),
                ..*cell
            }),
            Cell::Divider(cell) => Cell::Divider(DividerCell {
                id: id.to_owned(),
                ..*cell
            }),
        }
    }

    /// Returns a copy of the cell with its source IDs replaced by the given IDs.
    ///
    /// If the cell contains any data, only data that belongs to any of the new
    /// source IDs is retained.
    #[must_use]
    pub fn with_source_ids(&self, source_ids: Vec<String>) -> Self {
        match self {
            Cell::Checkbox(cell) => Cell::Checkbox(cell.clone()),
            Cell::Code(cell) => Cell::Code(cell.clone()),
            Cell::Graph(cell) => Cell::Graph(GraphCell {
                id: cell.id.clone(),
                data: cell.data.as_ref().map(|data| {
                    data.iter()
                        .filter(|&(k, _)| source_ids.contains(k))
                        .map(|(k, v)| (k.clone(), v.clone()))
                        .collect()
                }),
                stacking_type: cell.stacking_type,
                source_ids,
                time_range: cell.time_range.clone(),
                title: cell.title.clone(),
                ..*cell
            }),
            Cell::Heading(cell) => Cell::Heading(cell.clone()),
            Cell::ListItem(cell) => Cell::ListItem(cell.clone()),
            Cell::Log(cell) => Cell::Log(LogCell {
                id: cell.id.clone(),
                data: cell.data.as_ref().map(|data| {
                    data.iter()
                        .filter(|(k, _)| source_ids.contains(k))
                        .map(|(k, v)| (k.clone(), v.clone()))
                        .collect()
                }),
                time_range: cell.time_range.clone(),
                source_ids,
                ..*cell
            }),
            Cell::Prometheus(cell) => Cell::Prometheus(cell.clone()),
            Cell::Elasticsearch(cell) => Cell::Elasticsearch(cell.clone()),
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
            Cell::Image(cell) => Cell::Image(cell.clone()),
            Cell::Divider(cell) => Cell::Divider(cell.clone()),
        }
    }

    /// Returns a copy of the cell with its text replaced by the given text.
    pub fn with_text(&self, text: &str) -> Self {
        match self {
            Cell::Graph(cell) => Cell::Graph(GraphCell {
                id: cell.id.clone(),
                data: cell.data.clone(),
                source_ids: cell.source_ids.clone(),
                time_range: cell.time_range.clone(),
                title: text.to_owned(),
                ..*cell
            }),
            cell => cell.with_content(text),
        }
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Serializable)]
#[fp(rust_plugin_module = "fiberplane::protocols::core")]
#[serde(rename_all = "camelCase")]
pub struct CheckboxCell {
    pub id: String,
    pub checked: bool,
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Serializable)]
#[fp(rust_plugin_module = "fiberplane::protocols::core")]
#[serde(rename_all = "camelCase")]
pub struct CodeCell {
    pub id: String,
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    /// Optional MIME type to use for syntax highlighting.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub syntax: Option<String>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Serializable)]
#[fp(rust_plugin_module = "fiberplane::protocols::core")]
#[serde(rename_all = "camelCase")]
pub struct DividerCell {
    pub id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Serializable)]
#[fp(rust_plugin_module = "fiberplane::protocols::core")]
#[serde(rename_all = "camelCase")]
pub struct GraphCell {
    pub id: String,
    pub graph_type: GraphType,
    pub stacking_type: StackingType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    pub source_ids: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_range: Option<TimeRange>,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<BTreeMap<String, Vec<Series<f64>>>>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Serializable)]
#[fp(rust_plugin_module = "fiberplane::protocols::core")]
#[serde(rename_all = "camelCase")]
pub struct HeadingCell {
    pub id: String,
    pub heading_type: HeadingType,
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Serializable)]
#[fp(rust_plugin_module = "fiberplane::protocols::core")]
#[serde(rename_all = "camelCase")]
pub struct LogCell {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    pub source_ids: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<BTreeMap<String, Vec<LogRecord>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_range: Option<TimeRange>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Serializable)]
#[fp(rust_plugin_module = "fiberplane::protocols::core")]
#[serde(rename_all = "camelCase")]
pub struct ListItemCell {
    pub id: String,
    pub content: String,
    pub list_type: ListType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_number: Option<u16>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Serializable)]
#[fp(rust_plugin_module = "fiberplane::protocols::core")]
#[serde(rename_all = "camelCase")]
pub struct PrometheusCell {
    pub id: String,
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Serializable)]
#[fp(rust_plugin_module = "fiberplane::protocols::core")]
#[serde(rename_all = "camelCase")]
pub struct ElasticsearchCell {
    pub id: String,
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Serializable)]
#[fp(rust_plugin_module = "fiberplane::protocols::core")]
#[serde(rename_all = "camelCase")]
pub struct TableCell {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    pub source_ids: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<BTreeMap<String, Vec<Instant<f64>>>>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Serializable)]
#[fp(rust_plugin_module = "fiberplane::protocols::core")]
#[serde(rename_all = "camelCase")]
pub struct TextCell {
    pub id: String,
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Serializable)]
#[fp(rust_plugin_module = "fiberplane::protocols::core")]
#[serde(rename_all = "camelCase")]
pub struct ImageCell {
    pub id: String,

    // Refers to the id for a file (used to retrieve the file)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_id: Option<String>,

    /// Used to indicates the upload progress.
    /// If file_id is set this shouldn't be set
    /// Also: if no progress is set and no file_id exists
    /// it means the cell is in the initial state (ready for upload)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i32>,

    /// Will contain a hash to show as a preview for the image
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preview: Option<String>,

    /// URL of the image if it was originally hosted on a remote server.
    /// This will not be set if the image was uploaded through the
    /// Fiberplane Studio.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize, Serializable)]
#[fp(rust_plugin_module = "fiberplane::protocols::core")]
#[serde(rename_all = "snake_case")]
pub enum GraphType {
    Bar,
    Line,
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize, Serializable)]
#[fp(rust_plugin_module = "fiberplane::protocols::core")]
#[serde(rename_all = "snake_case")]
pub enum StackingType {
    None,
    Stacked,
    Percentage,
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize, Serializable)]
#[fp(rust_plugin_module = "fiberplane::protocols::core")]
#[serde(rename_all = "snake_case")]
pub enum HeadingType {
    H1,
    H2,
    H3,
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize, Serializable)]
#[fp(rust_plugin_module = "fiberplane::protocols::core")]
#[serde(rename_all = "snake_case")]
pub enum ListType {
    Ordered,
    Unordered,
}

/// A range in time from a given timestamp (inclusive) up to another timestamp (exclusive).
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize, Serializable)]
#[fp(rust_plugin_module = "fiberplane::protocols::core")]
pub struct TimeRange {
    pub from: Timestamp,
    pub to: Timestamp,
}

/// Timestamp specified in seconds since the UNIX epoch, with subsecond precision.
pub type Timestamp = f64;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Serializable)]
#[fp(rust_plugin_module = "fiberplane::protocols::core")]
pub struct Metric {
    pub name: String,
    pub labels: HashMap<String, String>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Serializable)]
#[fp(rust_plugin_module = "fiberplane::protocols::core")]
pub struct Point<T> {
    pub timestamp: Timestamp,
    pub value: T,
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize, Serializable)]
#[deprecated(note = "see FP-676: https://linear.app/fiberplane/issue/FP-676/deprecate-point-type")]
#[fp(rust_plugin_module = "fiberplane::protocols::core")]
#[serde(rename_all = "snake_case")]
pub enum PointType {
    F64,
    String,
}

/// A single data-point in time, with meta-data about the metric it was taken from.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Serializable)]
#[fp(rust_plugin_module = "fiberplane::protocols::core")]
#[serde(rename_all = "camelCase")]
pub struct Instant<T>
where
    T: Serializable,
{
    pub metric: Metric,
    pub point: Point<T>,
    point_type: PointType,
}

impl<T: Serializable> Instant<T> {
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

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Serializable)]
#[fp(rust_plugin_module = "fiberplane::protocols::core")]
#[serde(rename_all = "camelCase")]
pub struct LogRecord {
    pub timestamp: Timestamp,
    pub body: String,
    pub attributes: HashMap<String, String>,
    pub resource: HashMap<String, String>,
    pub trace_id: Option<String>,
    pub span_id: Option<String>,
}

/// A series of data-points in time, with meta-data about the metric it was taken from.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Serializable)]
#[fp(rust_plugin_module = "fiberplane::protocols::core")]
#[serde(rename_all = "camelCase")]
pub struct Series<T>
where
    T: Serializable,
{
    pub metric: Metric,
    pub points: Vec<Point<T>>,
    point_type: PointType,
    pub visible: bool,
}

impl<T: Serializable> Series<T> {
    pub fn point_type(&self) -> PointType {
        self.point_type
    }
}

impl Series<f64> {
    pub fn new_f64(metric: Metric, points: Vec<Point<f64>>, visible: bool) -> Self {
        Self {
            metric,
            points,
            point_type: PointType::F64,
            visible,
        }
    }
}

/// NotebookDataSource represents the way a data-source can be embedded in a
/// Notebook.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Serializable)]
#[fp(rust_plugin_module = "fiberplane::protocols::core")]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum NotebookDataSource {
    /// Inline is a data-source which only exists in this notebook.
    Inline(InlineDataSource),

    /// Organization is a data-source which is stored on the API server,
    /// allowing for data-source reuse.
    Organization(OrganizationDataSource),
}

/// OrganizationDataSource represents a data-source as stored for a organization
/// on the API.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Serializable)]
#[fp(rust_plugin_module = "fiberplane::protocols::core")]
#[serde(rename_all = "camelCase")]
pub struct InlineDataSource {
    /// The actual data-source.
    pub data_source: DataSource,
}

/// OrganizationDataSource represents a data-source as stored for a organization
/// on the API.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Serializable)]
#[fp(rust_plugin_module = "fiberplane::protocols::core")]
#[serde(rename_all = "camelCase")]
pub struct OrganizationDataSource {
    /// identifier used to manipulate this data-source.
    pub id: String,

    /// Name to identify this organization data-source. This does not have to be
    /// the same as the name in the data-source.
    pub name: String,

    /// If default_data_source is true, then this data-source will be added to
    /// any newly created notebooks.
    pub default_data_source: bool,

    /// The actual data-source.
    pub data_source: DataSource,
}

/// A data-source represents all the configuration for a specific component or
/// service. It will be used by provider.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Serializable)]
#[fp(rust_plugin_module = "fiberplane::protocols::core")]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum DataSource {
    Prometheus(PrometheusDataSource),
    Proxy(ProxyDataSource),
    Elasticsearch(ElasticsearchDataSource),
    // Kubernetes
}

/// A data-source for Prometheus. Currently only requires a url. This should be
/// a full URL starting with http:// or https:// the domain, and optionally a
/// port and a path.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Serializable)]
#[fp(rust_plugin_module = "fiberplane::protocols::core")]
#[serde(rename_all = "camelCase")]
pub struct PrometheusDataSource {
    pub url: String,
}

/// A data-source for Elasticsearch. Currently only requires a url. This should be
/// a full URL starting with http:// or https:// the domain, and optionally a
/// port and a path.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Serializable)]
#[fp(rust_plugin_module = "fiberplane::protocols::core")]
#[serde(rename_all = "camelCase")]
pub struct ElasticsearchDataSource {
    pub url: String,
}

/// Relays requests for a data-source to a proxy server registered with the API.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Serializable)]
#[fp(rust_plugin_module = "fiberplane::protocols::core")]
#[serde(rename_all = "camelCase")]
pub struct ProxyDataSource {
    /// ID of the proxy as known by the API.
    pub proxy_id: String,

    /// Name of the data source exposed by the proxy.
    pub data_source_name: String,
}

/// Labels that are associated with a Notebook.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Serializable)]
#[fp(rust_plugin_module = "fiberplane::protocols::core")]
#[serde(rename_all = "camelCase")]
pub struct Label {
    /// The key of the label. Should be unique for a single Notebook.
    pub key: String,

    /// The value of the label. Can be left empty.
    pub value: String,
}

impl Label {
    pub fn new(key: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            value: value.into(),
        }
    }

    /// Validates the key and value.
    pub fn validate(&self) -> Result<(), LabelValidationError> {
        Label::validate_key(&self.key)?;
        Label::validate_value(&self.value)?;

        Ok(())
    }

    /// A key is considered valid if it adheres to the following criteria:
    /// It can contain two segments, a prefix and a name, the name segment has
    /// the following criteria:
    /// - must be 63 characters or less (cannot be empty)
    /// - must begin and end with an alphanumeric character ([a-z0-9A-Z])
    /// - could contain dashes (-), underscores (_), dots (.), and alphanumerics between
    /// The prefix is optional, if specified must follow the following criteria:
    /// - must be 253 characters or less
    /// - must be a valid DNS subdomain
    pub fn validate_key(key: &str) -> Result<(), LabelValidationError> {
        if key.is_empty() {
            return Err(LabelValidationError::EmptyKey);
        }

        let (prefix, name) = match key.split_once('/') {
            Some((prefix, name)) => (Some(prefix), name),
            None => (None, key),
        };

        // Validation of the name portion
        if name.is_empty() {
            return Err(LabelValidationError::EmptyName);
        }

        if name.len() > MAX_LABEL_NAME_LENGTH {
            return Err(LabelValidationError::NameTooLong);
        }

        if !LABEL_NAME_VALUE_RE.is_match(name) {
            return Err(LabelValidationError::NameInvalidCharacters);
        }

        // Validation of the prefix portion
        if let Some(prefix) = prefix {
            if prefix.is_empty() {
                return Err(LabelValidationError::EmptyPrefix);
            }

            if prefix.len() > MAX_LABEL_PREFIX_LENGTH {
                return Err(LabelValidationError::PrefixTooLong);
            }

            if !LABEL_PREFIX_RE.is_match(prefix) {
                return Err(LabelValidationError::PrefixInvalidCharacters);
            }
        }

        Ok(())
    }

    /// A value is considered valid if it adheres to the following criteria:
    /// - must be 63 characters or less (can be empty)
    /// - unless empty, must begin and end with an alphanumeric character ([a-z0-9A-Z])
    /// - could contain dashes (-), underscores (_), dots (.), and alphanumerics between
    pub fn validate_value(value: &str) -> Result<(), LabelValidationError> {
        // Validation of the value (only if it contains something)
        if !value.is_empty() {
            if value.len() > MAX_LABEL_VALUE_LENGTH {
                return Err(LabelValidationError::ValueTooLong);
            }

            if !LABEL_NAME_VALUE_RE.is_match(value) {
                return Err(LabelValidationError::ValueInvalidCharacters);
            }
        }
        Ok(())
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Serializable)]
#[fp(rust_plugin_module = "fiberplane::protocols::core")]
#[serde(rename_all = "snake_case")]
pub enum LabelValidationError {
    /// The key in the label was empty
    EmptyKey,

    /// The name portion of the key was empty
    EmptyName,

    /// The name portion of the key was too long
    NameTooLong,

    /// The name portion of the key contains invalid characters
    NameInvalidCharacters,

    /// The prefix portion of the key was empty
    EmptyPrefix,

    /// The prefix portion of the key was too long
    PrefixTooLong,

    /// The prefix portion of the key contains invalid characters
    PrefixInvalidCharacters,

    /// The value is too long
    ValueTooLong,

    /// The value contains invalid characters
    ValueInvalidCharacters,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn label_key_valid() {
        let keys = vec![
            "key",
            "key.with.dot",
            "key_with_underscore",
            "key-with-dash",
            "key..with..double..dot",
            "fiberplane.io/key",
            "fiberplane.io/key.with.dot",
            "fiberplane.io/key_with_underscore",
            "fiberplane.io/key-with-dash",
        ];
        for key in keys.into_iter() {
            assert!(
                Label::validate_key(key).is_ok(),
                "Key \"{}\" should have passed validation",
                key
            );
        }
    }

    #[test]
    fn label_key_invalid() {
        let keys = vec![
            "",
            "too_long_name_too_long_name_too_long_name_too_long_name_too_long_name_",
            "fiberplane.fiberplane.fiberplane.fiberplane.fiberplane.fiberplane.fiberplane.fiberplane.fiberplane.fiberplane.fiberplane.fiberplane.fiberplane.fiberplane.fiberplane.fiberplane.fiberplane.fiberplane.fiberplane.fiberplane.fiberplane.fiberplane.fiberplane.fiberplane.com/name",
            "-name_start_with_non_alpha_numeric",
            "name_end_with_non_alpha_numeric-",
            "fiberplane..com/name",
            "fiberplane.com/invalid/name",
            "/name",
        ];
        for key in keys.into_iter() {
            assert!(
                Label::validate_key(key).is_err(),
                "Key \"{}\" should have failed validation",
                key
            );
        }
    }

    #[test]
    fn label_value_valid() {
        let values = vec![
            "",
            "value",
            "value.with.dot",
            "value_with_underscore",
            "value-with-dash",
        ];
        for value in values.into_iter() {
            assert!(
                Label::validate_value(value).is_ok(),
                "Value \"{}\" should have passed validation",
                value
            );
        }
    }

    #[test]
    fn label_value_invalid() {
        let values = vec![
            "too_long_name_too_long_name_too_long_name_too_long_name_too_long_name_",
            "-value_starting_with_a_dash",
            "value_ending_with_a_dash-",
        ];
        for value in values.into_iter() {
            assert!(
                Label::validate_key(value).is_err(),
                "Value \"{}\" should have failed validation",
                value
            );
        }
    }
}
