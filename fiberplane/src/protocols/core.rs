use super::blobs::EncodedBlob;
use super::comments::UserSummary;
use super::data_sources::SelectedDataSources;
use super::formatting::Formatting;
pub use super::labels::Label;
use crate::query_data::{has_query_data, set_query_field, unset_query_field};
use base64uuid::Base64Uuid;
use fp_bindgen::prelude::Serializable;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};
use std::{ops::Sub, time::SystemTime};
use time::{ext::NumericalDuration, Duration, OffsetDateTime};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Serializable)]
#[fp(
    rust_plugin_module = "fiberplane::protocols::core",
    rust_wasmer_runtime_module = "fiberplane::protocols::core"
)]
#[serde(rename_all = "camelCase")]
pub struct NewNotebook {
    pub title: String,
    pub cells: Vec<Cell>,
    pub time_range: NewTimeRange,

    #[serde(default)]
    pub selected_data_sources: SelectedDataSources,

    #[serde(default)]
    pub labels: Vec<Label>,
}

impl From<Notebook> for NewNotebook {
    fn from(notebook: Notebook) -> Self {
        NewNotebook {
            title: notebook.title,
            cells: notebook.cells,
            time_range: notebook.time_range.into(),
            selected_data_sources: notebook.selected_data_sources,
            labels: notebook.labels,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, Serializable)]
#[fp(
    rust_plugin_module = "fiberplane::protocols::core",
    rust_wasmer_runtime_module = "fiberplane::protocols::core"
)]
#[serde(untagged)]
pub enum NewTimeRange {
    Absolute(TimeRange),
    Relative(RelativeTimeRange),
}

impl From<TimeRange> for NewTimeRange {
    fn from(time_range: TimeRange) -> Self {
        Self::Absolute(time_range)
    }
}

/// A relative time range specified in minutes.
///
/// A negative value means the time range starts at the given amount of
/// `minutes` of to *now*. A positive value (including zero) means the time
/// range starts now and ends `minutes` from now.
///
/// Relative time ranges are expanded to absolute time ranges upon instantiation
/// of a notebook.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize, Serializable)]
#[fp(
    rust_plugin_module = "fiberplane::protocols::core",
    rust_wasmer_runtime_module = "fiberplane::protocols::core"
)]
#[serde(rename_all = "camelCase")]
pub struct RelativeTimeRange {
    pub minutes: i32,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Serializable)]
#[fp(
    rust_plugin_module = "fiberplane::protocols::core",
    rust_wasmer_runtime_module = "fiberplane::protocols::core"
)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum CreatedBy {
    User(UserSummary),
    Trigger(TriggerSummary),
    Unknown,
}

impl CreatedBy {
    pub fn name(&self) -> String {
        match self {
            CreatedBy::User(user) => user.name.clone(),
            CreatedBy::Trigger(trigger) => trigger.title.clone(),
            CreatedBy::Unknown => String::from("Unknown"),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Serializable)]
#[fp(
    rust_plugin_module = "fiberplane::protocols::core",
    rust_wasmer_runtime_module = "fiberplane::protocols::core"
)]
#[serde(rename_all = "camelCase")]
pub struct TriggerSummary {
    pub id: Base64Uuid,
    pub title: String,
    pub template_id: Base64Uuid,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    pub updated_at: OffsetDateTime,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize, Serializable)]
#[fp(
    rust_plugin_module = "fiberplane::protocols::core",
    rust_wasmer_runtime_module = "fiberplane::protocols::core"
)]
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
#[fp(
    rust_plugin_module = "fiberplane::protocols::core",
    rust_wasmer_runtime_module = "fiberplane::protocols::core"
)]
#[serde(rename_all = "camelCase")]
pub struct Notebook {
    pub id: String,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    pub updated_at: OffsetDateTime,
    pub time_range: TimeRange,
    pub title: String,
    pub cells: Vec<Cell>,
    pub revision: u32,
    pub visibility: NotebookVisibility,
    pub read_only: bool,
    pub created_by: CreatedBy,

    #[serde(default)]
    pub selected_data_sources: SelectedDataSources,

    #[serde(default)]
    pub labels: Vec<Label>,
}

/// Representation of a single notebook cell.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Serializable)]
#[fp(
    rust_plugin_module = "fiberplane::protocols::core",
    rust_wasmer_runtime_module = "fiberplane::protocols::core"
)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Cell {
    Checkbox(CheckboxCell),
    Code(CodeCell),
    Discussion(DiscussionCell),
    Divider(DividerCell),
    Elasticsearch(ElasticsearchCell),
    Graph(GraphCell),
    Heading(HeadingCell),
    Image(ImageCell),
    ListItem(ListItemCell),
    Log(LogCell),
    Loki(LokiCell),
    Provider(ProviderCell),
    Table(TableCell),
    Text(TextCell),
}

impl Cell {
    /// Returns the cell's content, if any.
    pub fn content(&self) -> Option<&str> {
        match self {
            Cell::Checkbox(cell) => Some(&cell.content),
            Cell::Code(cell) => Some(&cell.content),
            Cell::Discussion(_) => None,
            Cell::Divider(_) => None,
            Cell::Elasticsearch(cell) => Some(&cell.content),
            Cell::Graph(_) => None,
            Cell::Heading(cell) => Some(&cell.content),
            Cell::Image(_) => None,
            Cell::ListItem(cell) => Some(&cell.content),
            Cell::Log(_) => None,
            Cell::Loki(cell) => Some(&cell.content),
            Cell::Provider(_) => None,
            Cell::Table(_) => None,
            Cell::Text(cell) => Some(&cell.content),
        }
    }

    /// Returns the cell's formatting, if any.
    pub fn formatting(&self) -> Option<&Formatting> {
        match self {
            Cell::Code(_)
            | Cell::Discussion(_)
            | Cell::Divider(_)
            | Cell::Elasticsearch(_)
            | Cell::Graph(_)
            | Cell::Image(_)
            | Cell::Loki(_)
            | Cell::Table(_) => None,
            Cell::Checkbox(cell) => cell.formatting.as_ref(),
            Cell::Heading(cell) => cell.formatting.as_ref(),
            Cell::ListItem(cell) => cell.formatting.as_ref(),
            Cell::Log(cell) => cell.formatting.as_ref(),
            Cell::Provider(cell) => cell.formatting.as_ref(),
            Cell::Text(cell) => cell.formatting.as_ref(),
        }
    }

    pub fn supports_formatting(&self) -> bool {
        match self {
            Cell::Code(_)
            | Cell::Discussion(_)
            | Cell::Divider(_)
            | Cell::Elasticsearch(_)
            | Cell::Image(_)
            | Cell::Loki(_) => false,
            Cell::Checkbox(_)
            | Cell::Graph(_)
            | Cell::Heading(_)
            | Cell::ListItem(_)
            | Cell::Log(_)
            | Cell::Provider(_)
            | Cell::Table(_)
            | Cell::Text(_) => true,
        }
    }

    /// Returns the cell's ID.
    pub fn id(&self) -> &str {
        match self {
            Cell::Checkbox(cell) => &cell.id,
            Cell::Code(cell) => &cell.id,
            Cell::Discussion(cell) => &cell.id,
            Cell::Divider(cell) => &cell.id,
            Cell::Elasticsearch(cell) => &cell.id,
            Cell::Graph(cell) => &cell.id,
            Cell::Heading(cell) => &cell.id,
            Cell::Image(cell) => &cell.id,
            Cell::ListItem(cell) => &cell.id,
            Cell::Log(cell) => &cell.id,
            Cell::Loki(cell) => &cell.id,
            Cell::Provider(cell) => &cell.id,
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
            Cell::Log(cell) => cell.source_ids.iter().map(String::as_str).collect(),
            Cell::Checkbox(_)
            | Cell::Code(_)
            | Cell::Discussion(_)
            | Cell::Divider(_)
            | Cell::Elasticsearch(_)
            | Cell::Graph(_)
            | Cell::Heading(_)
            | Cell::Image(_)
            | Cell::ListItem(_)
            | Cell::Loki(_)
            | Cell::Provider(_)
            | Cell::Table(_)
            | Cell::Text(_) => vec![],
        }
    }

    /// Returns the cell's text, if any.
    pub fn text(&self) -> Option<&str> {
        match self {
            Cell::Log(cell) => Some(&cell.title),
            Cell::Provider(cell) => Some(&cell.title),
            cell => cell.content(),
        }
    }

    /// Returns a copy of the cell with the given content appended.
    #[must_use]
    pub fn with_appended_content(&self, content: &str) -> Self {
        self.with_content(&format!("{}{}", self.content().unwrap_or(""), content))
    }

    /// Returns a copy of the cell with its content replaced by the given
    /// content (without any formatting).
    #[must_use]
    pub fn with_content(&self, content: &str) -> Self {
        match self {
            Cell::Checkbox(cell) => Cell::Checkbox(CheckboxCell {
                id: cell.id.clone(),
                content: content.to_owned(),
                formatting: Some(vec![]),
                ..*cell
            }),
            Cell::Code(cell) => Cell::Code(CodeCell {
                id: cell.id.clone(),
                content: content.to_owned(),
                syntax: cell.syntax.clone(),
                ..*cell
            }),
            Cell::Discussion(cell) => Cell::Discussion(cell.clone()),
            Cell::Divider(cell) => Cell::Divider(cell.clone()),
            Cell::Elasticsearch(cell) => Cell::Elasticsearch(ElasticsearchCell {
                id: cell.id.clone(),
                content: content.to_owned(),
                ..*cell
            }),
            Cell::Graph(cell) => Cell::Graph(cell.clone()),
            Cell::Heading(cell) => Cell::Heading(HeadingCell {
                id: cell.id.clone(),
                content: content.to_owned(),
                formatting: Some(vec![]),
                ..*cell
            }),
            Cell::Image(cell) => Cell::Image(cell.clone()),
            Cell::ListItem(cell) => Cell::ListItem(ListItemCell {
                id: cell.id.clone(),
                content: content.to_owned(),
                formatting: Some(vec![]),
                ..*cell
            }),
            Cell::Log(cell) => Cell::Log(cell.clone()),
            Cell::Loki(cell) => Cell::Loki(LokiCell {
                id: cell.id.clone(),
                content: content.to_owned(),
                ..*cell
            }),
            Cell::Provider(cell) => Cell::Provider(cell.clone()),
            Cell::Table(cell) => Cell::Table(cell.clone()),
            Cell::Text(cell) => Cell::Text(TextCell {
                id: cell.id.clone(),
                content: content.to_owned(),
                formatting: Some(vec![]),
                ..*cell
            }),
        }
    }

    /// Returns a copy of the cell with a new ID.
    #[must_use]
    pub fn with_id(&self, id: &str) -> Self {
        match self {
            Cell::Checkbox(cell) => Cell::Checkbox(CheckboxCell {
                id: id.to_owned(),
                ..cell.clone()
            }),
            Cell::Code(cell) => Cell::Code(CodeCell {
                id: id.to_owned(),
                ..cell.clone()
            }),
            Cell::Discussion(cell) => Cell::Discussion(DiscussionCell {
                id: id.to_owned(),
                ..cell.clone()
            }),
            Cell::Divider(cell) => Cell::Divider(DividerCell {
                id: id.to_owned(),
                ..cell.clone()
            }),
            Cell::Elasticsearch(cell) => Cell::Elasticsearch(ElasticsearchCell {
                id: id.to_owned(),
                ..cell.clone()
            }),
            Cell::Graph(cell) => Cell::Graph(GraphCell {
                id: id.to_owned(),
                ..cell.clone()
            }),
            Cell::Heading(cell) => Cell::Heading(HeadingCell {
                id: id.to_owned(),
                ..cell.clone()
            }),
            Cell::Image(cell) => Cell::Image(ImageCell {
                id: id.to_owned(),
                ..cell.clone()
            }),
            Cell::ListItem(cell) => Cell::ListItem(ListItemCell {
                id: id.to_owned(),
                ..cell.clone()
            }),
            Cell::Log(cell) => Cell::Log(LogCell {
                id: id.to_owned(),
                ..cell.clone()
            }),
            Cell::Loki(cell) => Cell::Loki(LokiCell {
                id: id.to_owned(),
                ..cell.clone()
            }),
            Cell::Provider(cell) => Cell::Provider(ProviderCell {
                id: id.to_owned(),
                ..cell.clone()
            }),
            Cell::Table(cell) => Cell::Table(TableCell {
                id: id.to_owned(),
                ..cell.clone()
            }),
            Cell::Text(cell) => Cell::Text(TextCell {
                id: id.to_owned(),
                ..cell.clone()
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
            Cell::Log(cell) => Cell::Log(LogCell {
                data: cell.data.as_ref().map(|data| {
                    data.iter()
                        .filter(|&(k, _)| source_ids.contains(k))
                        .map(|(k, v)| (k.clone(), v.clone()))
                        .collect()
                }),
                source_ids,
                ..cell.clone()
            }),
            cell => cell.clone(),
        }
    }

    /// Returns a copy of the cell with its text replaced by the given text.
    #[must_use]
    pub fn with_text(&self, text: &str) -> Self {
        match self {
            Cell::Log(cell) => Cell::Log(LogCell {
                id: cell.id.clone(),
                data: cell.data.clone(),
                title: text.to_owned(),
                formatting: Some(vec![]),
                time_range: cell.time_range.clone(),
                source_ids: cell.source_ids.clone(),
                display_fields: cell.display_fields.clone(),
                expanded_indices: cell.expanded_indices.clone(),
                selected_indices: cell.selected_indices.clone(),
                highlighted_indices: cell.highlighted_indices.clone(),
                visibility_filter: cell.visibility_filter.clone(),
                ..*cell
            }),
            Cell::Provider(cell) => Cell::Provider(ProviderCell {
                id: cell.id.clone(),
                formatting: Some(Vec::new()),
                intent: cell.intent.clone(),
                output: cell.output.clone(),
                query_data: cell.query_data.clone(),
                read_only: cell.read_only,
                response: cell.response.clone(),
                title: text.to_owned(),
            }),
            cell => cell.with_content(text),
        }
    }

    /// Returns a copy of the cell with its text replaced by the given text and
    /// formatting.
    ///
    /// **Warning:** For cell types that have text, but which do not support
    ///              rich-text, the formatting will be dropped silently.
    #[must_use]
    pub fn with_rich_text(&self, text: &str, formatting: Formatting) -> Self {
        match self {
            Cell::Checkbox(cell) => Cell::Checkbox(CheckboxCell {
                id: cell.id.clone(),
                content: text.to_owned(),
                formatting: Some(formatting),
                ..*cell
            }),
            Cell::Log(cell) => Cell::Log(LogCell {
                id: cell.id.clone(),
                data: cell.data.clone(),
                formatting: Some(formatting),
                source_ids: cell.source_ids.clone(),
                time_range: cell.time_range.clone(),
                title: text.to_owned(),
                display_fields: cell.display_fields.clone(),
                expanded_indices: cell.expanded_indices.clone(),
                selected_indices: cell.selected_indices.clone(),
                highlighted_indices: cell.highlighted_indices.clone(),
                visibility_filter: cell.visibility_filter.clone(),
                ..*cell
            }),
            Cell::Heading(cell) => Cell::Heading(HeadingCell {
                id: cell.id.clone(),
                content: text.to_owned(),
                formatting: Some(formatting),
                ..*cell
            }),
            Cell::ListItem(cell) => Cell::ListItem(ListItemCell {
                id: cell.id.clone(),
                content: text.to_owned(),
                formatting: Some(formatting),
                ..*cell
            }),
            Cell::Provider(cell) => Cell::Provider(ProviderCell {
                id: cell.id.clone(),
                formatting: Some(formatting),
                intent: cell.intent.clone(),
                output: cell.output.clone(),
                query_data: cell.query_data.clone(),
                read_only: cell.read_only,
                response: cell.response.clone(),
                title: text.to_owned(),
            }),
            Cell::Text(cell) => Cell::Text(TextCell {
                id: cell.id.clone(),
                content: text.to_owned(),
                formatting: Some(formatting),
                ..*cell
            }),
            Cell::Code(_)
            | Cell::Discussion(_)
            | Cell::Divider(_)
            | Cell::Elasticsearch(_)
            | Cell::Graph(_)
            | Cell::Image(_)
            | Cell::Loki(_)
            | Cell::Table(_) => self.with_text(text),
        }
    }

    /// Returns a copy of the cell with the text for the given field replaced by
    /// the given text and optional formatting.
    ///
    /// If no field is given, the text is applied to the cell's main text field,
    /// similar to `with_text()` or `with_rich_text()`, depending on whether any
    /// formatting is given.
    ///
    /// **Warning:** For cell types that have text, but which do not support
    ///              rich-text, any given formatting will be dropped silently.
    #[must_use]
    pub fn with_text_for_field(
        &self,
        text: &str,
        formatting: Option<Formatting>,
        field: Option<&str>,
    ) -> Self {
        match (self, field) {
            (Cell::Provider(cell), Some(field)) => {
                Cell::Provider(cell.with_query_field(field, text))
            }
            (cell, _) => {
                if let Some(formatting) = formatting {
                    cell.with_rich_text(text, formatting)
                } else {
                    cell.with_text(text)
                }
            }
        }
    }

    pub fn id_mut(&mut self) -> &mut String {
        match self {
            Cell::Checkbox(cell) => &mut cell.id,
            Cell::Code(cell) => &mut cell.id,
            Cell::Discussion(cell) => &mut cell.id,
            Cell::Divider(cell) => &mut cell.id,
            Cell::Elasticsearch(cell) => &mut cell.id,
            Cell::Graph(cell) => &mut cell.id,
            Cell::Heading(cell) => &mut cell.id,
            Cell::Image(cell) => &mut cell.id,
            Cell::ListItem(cell) => &mut cell.id,
            Cell::Log(cell) => &mut cell.id,
            Cell::Loki(cell) => &mut cell.id,
            Cell::Provider(cell) => &mut cell.id,
            Cell::Table(cell) => &mut cell.id,
            Cell::Text(cell) => &mut cell.id,
        }
    }

    /// Returns a mutable reference to the formatting array if the cell type supports formatting.
    ///
    /// If the cell type supports formatting but the cell does not have any, this method
    /// will initialize the formatting as an empty array.
    pub fn formatting_mut(&mut self) -> Option<&mut Formatting> {
        let formatting = match self {
            Cell::Checkbox(cell) => Some(&mut cell.formatting),
            Cell::Heading(cell) => Some(&mut cell.formatting),
            Cell::ListItem(cell) => Some(&mut cell.formatting),
            Cell::Log(cell) => Some(&mut cell.formatting),
            Cell::Provider(cell) => Some(&mut cell.formatting),
            Cell::Text(cell) => Some(&mut cell.formatting),
            Cell::Code(_)
            | Cell::Discussion(_)
            | Cell::Divider(_)
            | Cell::Elasticsearch(_)
            | Cell::Graph(_)
            | Cell::Image(_)
            | Cell::Loki(_)
            | Cell::Table(_) => None,
        };

        // Turn the Option<&mut Option<Formatting>> into a Option<&mut Formatting>
        // and initialize the formatting array if necessary
        match formatting {
            Some(Some(formatting)) => Some(formatting),
            Some(formatting @ None) => {
                *formatting = Some(vec![]);
                formatting.as_mut()
            }
            None => None,
        }
    }

    /// Returns a mutable reference to the cell's text, if any.
    pub fn text_mut(&mut self) -> Option<&mut String> {
        match self {
            Cell::Checkbox(cell) => Some(&mut cell.content),
            Cell::Code(cell) => Some(&mut cell.content),
            Cell::Discussion(_) => None,
            Cell::Divider(_) => None,
            Cell::Image(_) => None,
            Cell::Elasticsearch(cell) => Some(&mut cell.content),
            Cell::Graph(_) => None,
            Cell::Heading(cell) => Some(&mut cell.content),
            Cell::ListItem(cell) => Some(&mut cell.content),
            Cell::Log(cell) => Some(&mut cell.title),
            Cell::Loki(cell) => Some(&mut cell.content),
            Cell::Provider(cell) => Some(&mut cell.title),
            Cell::Table(_) => None,
            Cell::Text(cell) => Some(&mut cell.content),
        }
    }
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Eq, Serialize, Serializable)]
#[fp(
    rust_plugin_module = "fiberplane::protocols::core",
    rust_wasmer_runtime_module = "fiberplane::protocols::core"
)]
#[serde(rename_all = "camelCase")]
pub struct CheckboxCell {
    pub id: String,
    pub checked: bool,
    pub content: String,
    /// Optional formatting to be applied to the cell's content.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub formatting: Option<Formatting>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Eq, Serialize, Serializable)]
#[fp(
    rust_plugin_module = "fiberplane::protocols::core",
    rust_wasmer_runtime_module = "fiberplane::protocols::core"
)]
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

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Eq, Serialize, Serializable)]
#[fp(
    rust_plugin_module = "fiberplane::protocols::core",
    rust_wasmer_runtime_module = "fiberplane::protocols::core"
)]
#[serde(rename_all = "camelCase")]
pub struct DividerCell {
    pub id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Serializable)]
#[fp(
    rust_plugin_module = "fiberplane::protocols::core",
    rust_wasmer_runtime_module = "fiberplane::protocols::core"
)]
#[serde(rename_all = "camelCase")]
pub struct GraphCell {
    pub id: String,

    /// Links to the data to render in the graph.
    pub data_links: Vec<String>,

    pub graph_type: GraphType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,

    pub stacking_type: StackingType,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Eq, Serialize, Serializable)]
#[fp(
    rust_plugin_module = "fiberplane::protocols::core",
    rust_wasmer_runtime_module = "fiberplane::protocols::core"
)]
#[serde(rename_all = "camelCase")]
pub struct HeadingCell {
    pub id: String,
    pub heading_type: HeadingType,
    pub content: String,
    /// Optional formatting to be applied to the cell's content.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub formatting: Option<Formatting>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize, Serializable)]
#[fp(
    rust_plugin_module = "fiberplane::protocols::core",
    rust_wasmer_runtime_module = "fiberplane::protocols::core"
)]
#[serde(rename_all = "camelCase")]
pub struct LogCell {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    pub source_ids: Vec<String>,
    /// Optional formatting to be applied to the cell's title.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub formatting: Option<Formatting>,

    #[serde(default = "default_title")]
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<BTreeMap<String, Vec<LogRecord>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_range: Option<LegacyTimeRange>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_fields: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub hide_similar_values: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub expanded_indices: Option<Vec<LogRecordIndex>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility_filter: Option<LogVisibilityFilter>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected_indices: Option<Vec<LogRecordIndex>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub highlighted_indices: Option<Vec<LogRecordIndex>>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize, Serializable)]
#[fp(
    rust_plugin_module = "fiberplane::protocols::core",
    rust_wasmer_runtime_module = "fiberplane::protocols::core"
)]
#[serde(rename_all = "snake_case")]
pub enum LogVisibilityFilter {
    All,
    Selected,
    Highlighted,
}

fn default_title() -> String {
    "".to_string()
}

/// A single expanded row of log records, as identified by [key] and [index]
/// pointing into the source data of the LogCell.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, Serializable)]
#[fp(
    rust_plugin_module = "fiberplane::protocols::core",
    rust_wasmer_runtime_module = "fiberplane::protocols::core"
)]
#[serde(rename_all = "camelCase")]
pub struct LogRecordIndex {
    pub key: String,
    pub index: i32,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Eq, Serialize, Serializable)]
#[fp(
    rust_plugin_module = "fiberplane::protocols::core",
    rust_wasmer_runtime_module = "fiberplane::protocols::core"
)]
#[serde(rename_all = "camelCase")]
pub struct ListItemCell {
    pub id: String,
    pub content: String,
    /// Optional formatting to be applied to the cell's content.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub formatting: Option<Formatting>,
    pub list_type: ListType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_number: Option<u16>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize, Serializable)]
#[fp(
    rust_plugin_module = "fiberplane::protocols::core",
    rust_wasmer_runtime_module = "fiberplane::protocols::core"
)]
#[serde(rename_all = "camelCase")]
pub struct ProviderCell {
    pub id: String,

    /// The intent served by this provider cell.
    ///
    /// See: https://www.notion.so/fiberplane/RFC-45-Provider-Protocol-2-0-Revised-4ec85a0233924b2db0010d8cdae75e16#c8ed5dfbfd764e6bbd5c5b79333f9d6e
    pub intent: String,

    /// Query data encoded as `"<mime-type>,<data>"`, where the MIME type is
    /// either `"application/x-www-form-urlencoded"` or `"multipart/form-data"`.
    /// This is used for storing data for the Query Builder.
    ///
    /// Note: The format follows the specification for data URLs, without the
    ///       `data:` prefix. See: https://developer.mozilla.org/en-US/docs/Web/HTTP/Basics_of_HTTP/Data_URLs
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub query_data: Option<String>,

    /// Optional response data from the provider.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub response: Option<EncodedBlob>,

    /// Optional list of generated output cells.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub output: Option<Vec<Cell>>,

    /// Optional title to assign the cell.
    #[serde(default)]
    pub title: String,

    /// Optional formatting to apply to the title.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub formatting: Option<Formatting>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
}

impl ProviderCell {
    /// Returns a clone of the provider cell, with the query data updated for
    /// the given query field.
    ///
    /// Unsets the query field if the value is empty.
    pub fn with_query_field(&self, field_name: &str, value: &str) -> Self {
        let query_data = self.query_data.as_deref().unwrap_or_default();
        let query_data = if value.is_empty() {
            unset_query_field(query_data, field_name)
        } else {
            set_query_field(query_data, field_name, value)
        };
        Self {
            query_data: if has_query_data(&query_data) {
                Some(query_data)
            } else {
                None
            },
            ..self.clone()
        }
    }
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Eq, Serialize, Serializable)]
#[fp(
    rust_plugin_module = "fiberplane::protocols::core",
    rust_wasmer_runtime_module = "fiberplane::protocols::core"
)]
#[serde(rename_all = "camelCase")]
pub struct ElasticsearchCell {
    pub id: String,
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Eq, Serialize, Serializable)]
#[fp(
    rust_plugin_module = "fiberplane::protocols::core",
    rust_wasmer_runtime_module = "fiberplane::protocols::core"
)]
#[serde(rename_all = "camelCase")]
pub struct LokiCell {
    pub id: String,
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Serializable)]
#[fp(
    rust_plugin_module = "fiberplane::protocols::core",
    rust_wasmer_runtime_module = "fiberplane::protocols::core"
)]
#[serde(rename_all = "camelCase")]
pub struct TableCell {
    pub id: String,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,

    /// The rows that make up the content of the table.
    pub rows: Vec<TableRow>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Serializable)]
#[fp(
    rust_plugin_module = "fiberplane::protocols::core",
    rust_wasmer_runtime_module = "fiberplane::protocols::core"
)]
#[serde(rename_all = "camelCase")]
pub struct TableRow {
    /// The columns that make up the content of this table row.
    pub cols: Vec<TableColumn>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Serializable)]
#[fp(
    rust_plugin_module = "fiberplane::protocols::core",
    rust_wasmer_runtime_module = "fiberplane::protocols::core"
)]
#[serde(rename_all = "camelCase")]
pub struct TableColumn {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub formatting: Option<Formatting>,

    pub text: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Eq, Serialize, Serializable)]
#[fp(
    rust_plugin_module = "fiberplane::protocols::core",
    rust_wasmer_runtime_module = "fiberplane::protocols::core"
)]
#[serde(rename_all = "camelCase")]
pub struct TextCell {
    pub id: String,
    pub content: String,
    /// Optional formatting to be applied to the cell's content.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub formatting: Option<Formatting>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize, Serializable)]
#[fp(
    rust_plugin_module = "fiberplane::protocols::core",
    rust_wasmer_runtime_module = "fiberplane::protocols::core"
)]
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

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize, Serializable)]
#[fp(rust_plugin_module = "fiberplane::protocols::core")]
#[serde(rename_all = "camelCase")]
pub struct DiscussionCell {
    pub id: String,
    pub thread_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq, Serialize, Serializable)]
#[fp(
    rust_plugin_module = "fiberplane::protocols::core",
    rust_wasmer_runtime_module = "fiberplane::protocols::core"
)]
#[serde(rename_all = "snake_case")]
pub enum GraphType {
    Bar,
    Line,
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq, Serialize, Serializable)]
#[fp(
    rust_plugin_module = "fiberplane::protocols::core",
    rust_wasmer_runtime_module = "fiberplane::protocols::core"
)]
#[serde(rename_all = "snake_case")]
pub enum StackingType {
    None,
    Stacked,
    Percentage,
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq, Serialize, Serializable)]
#[fp(
    rust_plugin_module = "fiberplane::protocols::core",
    rust_wasmer_runtime_module = "fiberplane::protocols::core"
)]
#[serde(rename_all = "snake_case")]
pub enum HeadingType {
    H1,
    H2,
    H3,
}

impl Default for HeadingType {
    fn default() -> Self {
        HeadingType::H1
    }
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq, Serialize, Serializable)]
#[fp(
    rust_plugin_module = "fiberplane::protocols::core",
    rust_wasmer_runtime_module = "fiberplane::protocols::core"
)]
#[serde(rename_all = "snake_case")]
pub enum ListType {
    Ordered,
    Unordered,
}

impl Default for ListType {
    fn default() -> Self {
        ListType::Unordered
    }
}

/// A range in time from a given timestamp (inclusive) up to another timestamp (exclusive).
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, Serializable)]
#[fp(
    rust_plugin_module = "fiberplane::protocols::core",
    rust_wasmer_runtime_module = "fiberplane::protocols::core"
)]
pub struct TimeRange {
    pub from: Timestamp,
    pub to: Timestamp,
}

impl From<NewTimeRange> for TimeRange {
    fn from(new_time_range: NewTimeRange) -> Self {
        match new_time_range {
            NewTimeRange::Absolute(time_range) => time_range,
            NewTimeRange::Relative(RelativeTimeRange { minutes }) => {
                let now = OffsetDateTime::now_utc();
                if minutes < 0 {
                    TimeRange {
                        from: (now + (minutes as i64).minutes()).into(),
                        to: now.into(),
                    }
                } else {
                    TimeRange {
                        from: now.into(),
                        to: (now + (minutes as i64).minutes()).into(),
                    }
                }
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize, Serializable)]
#[fp(
    rust_plugin_module = "fiberplane::protocols::core",
    rust_wasmer_runtime_module = "fiberplane::protocols::core"
)]
pub struct Timestamp(#[serde(with = "time::serde::rfc3339")] pub OffsetDateTime);

impl From<OffsetDateTime> for Timestamp {
    fn from(time: OffsetDateTime) -> Self {
        Self(time)
    }
}

impl From<SystemTime> for Timestamp {
    fn from(time: SystemTime) -> Self {
        Self(OffsetDateTime::from(time))
    }
}

impl Sub<Timestamp> for Timestamp {
    type Output = Duration;

    fn sub(self, rhs: Timestamp) -> Self::Output {
        self.0 - rhs.0
    }
}

impl From<Timestamp> for OffsetDateTime {
    fn from(timestamp: Timestamp) -> Self {
        timestamp.0
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Serializable)]
#[fp(
    rust_plugin_module = "fiberplane::protocols::core",
    rust_wasmer_runtime_module = "fiberplane::protocols::core"
)]
#[deprecated(note = "Use `TimeRange` instead")]
pub struct LegacyTimeRange {
    pub from: LegacyTimestamp,
    pub to: LegacyTimestamp,
}

/// Timestamp specified in seconds since the UNIX epoch, with subsecond precision.
#[deprecated(note = "Use `Timestamp` instead")]
pub type LegacyTimestamp = f64;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Serializable)]
#[fp(
    rust_plugin_module = "fiberplane::protocols::core",
    rust_wasmer_runtime_module = "fiberplane::protocols::core"
)]
#[serde(rename_all = "camelCase")]
pub struct LogRecord {
    pub timestamp: LegacyTimestamp,
    pub body: String,
    pub attributes: HashMap<String, String>,
    pub resource: HashMap<String, String>,
    pub trace_id: Option<String>,
    pub span_id: Option<String>,
}

/// Workspace representation.
#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize, Serializable)]
#[fp(
    rust_plugin_module = "fiberplane::protocols::core",
    rust_wasmer_runtime_module = "fiberplane::protocols::core"
)]
#[serde(rename_all = "camelCase")]
pub struct Workspace {
    pub id: Base64Uuid,
    pub name: String,
    #[serde(rename = "type")]
    pub ty: WorkspaceType,
    pub default_data_sources: SelectedDataSources,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    pub updated_at: OffsetDateTime,
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq, Serialize, Serializable)]
#[fp(
    rust_plugin_module = "fiberplane::protocols::core",
    rust_wasmer_runtime_module = "fiberplane::protocols::core"
)]
#[serde(rename_all = "snake_case")]
pub enum WorkspaceType {
    Personal,
    Organization,
}

/// Payload to be able to invite someone to a workspace.
#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize, Serializable)]
#[fp(
    rust_plugin_module = "fiberplane::protocols::core",
    rust_wasmer_runtime_module = "fiberplane::protocols::core"
)]
#[serde(rename_all = "camelCase")]
pub struct NewWorkspaceInvitation {
    pub email: String,
    #[serde(default)]
    pub role: AuthzRoles,
}

/// Response received from create a new workspace endpoint.
#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize, Serializable)]
#[fp(
    rust_plugin_module = "fiberplane::protocols::core",
    rust_wasmer_runtime_module = "fiberplane::protocols::core"
)]
#[serde(rename_all = "camelCase")]
pub struct NewWorkspaceInvitationResponse {
    pub url: String,
}

/// Payload to create a new workspace.
#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize, Serializable)]
#[fp(
    rust_plugin_module = "fiberplane::protocols::core",
    rust_wasmer_runtime_module = "fiberplane::protocols::core"
)]
#[serde(rename_all = "camelCase")]
pub struct NewWorkspace {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_data_sources: Option<SelectedDataSources>,
}

/// Payload to update workspace settings
#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize, Serializable)]
#[fp(
    rust_plugin_module = "fiberplane::protocols::core",
    rust_wasmer_runtime_module = "fiberplane::protocols::core"
)]
#[serde(rename_all = "camelCase")]
pub struct UpdateWorkspace {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<Base64Uuid>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_data_sources: Option<SelectedDataSources>,
}

/// Payload to update a workspace members' role
#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize, Serializable)]
#[fp(
    rust_plugin_module = "fiberplane::protocols::core",
    rust_wasmer_runtime_module = "fiberplane::protocols::core"
)]
#[serde(rename_all = "snake_case")]
pub struct UpdateWorkspaceMember {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<AuthzRoles>,
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq, Serialize, Serializable)]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
#[fp(
    rust_plugin_module = "fiberplane::protocols::core",
    rust_wasmer_runtime_module = "fiberplane::protocols::core"
)]
#[serde(rename_all = "snake_case")]
pub enum AuthzRoles {
    Read,
    Write,
    Admin,
}

impl Default for AuthzRoles {
    fn default() -> Self {
        Self::Write
    }
}
