use super::{
    blobs::EncodedBlob,
    formatting::{translate, AnnotationWithOffset, Formatting},
};
use crate::{
    markdown::formatting_from_markdown,
    query_data::{has_query_data, set_query_field, unset_query_field},
    text_util::char_count,
};
use fp_bindgen::prelude::Serializable;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};
use std::fmt::{self, Display};
use std::str::FromStr;
use thiserror::Error;
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
#[fp(
    rust_plugin_module = "fiberplane::protocols::core",
    rust_wasmer_runtime_module = "fiberplane::protocols::core"
)]
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

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Serializable)]
#[fp(
    rust_plugin_module = "fiberplane::protocols::core",
    rust_wasmer_runtime_module = "fiberplane::protocols::core"
)]
#[serde(rename_all = "snake_case")]
pub enum UserType {
    Anonymous,
    Individual,
    Organization,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Serializable)]
#[fp(
    rust_plugin_module = "fiberplane::protocols::core",
    rust_wasmer_runtime_module = "fiberplane::protocols::core"
)]
#[serde(rename_all = "camelCase")]
pub struct CreatedBy {
    #[serde(rename = "type")]
    pub user_type: UserType,
    pub name: String,
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
    pub data_sources: BTreeMap<String, NotebookDataSource>,

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
    Prometheus(PrometheusCell),
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
            Cell::Prometheus(cell) => Some(&cell.content),
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
            | Cell::Image(_)
            | Cell::Loki(_)
            | Cell::Prometheus(_) => None,
            Cell::Checkbox(cell) => cell.formatting.as_ref(),
            Cell::Graph(cell) => cell.formatting.as_ref(),
            Cell::Heading(cell) => cell.formatting.as_ref(),
            Cell::ListItem(cell) => cell.formatting.as_ref(),
            Cell::Log(cell) => cell.formatting.as_ref(),
            Cell::Provider(cell) => cell.formatting.as_ref(),
            Cell::Table(cell) => cell.formatting.as_ref(),
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
            | Cell::Loki(_)
            | Cell::Prometheus(_) => false,
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
            Cell::Prometheus(cell) => &cell.id,
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
            Cell::Graph(cell) => cell.source_ids.iter().map(String::as_str).collect(),
            Cell::Log(cell) => cell.source_ids.iter().map(String::as_str).collect(),
            Cell::Table(cell) => cell.source_ids.iter().map(String::as_str).collect(),
            Cell::Checkbox(_)
            | Cell::Code(_)
            | Cell::Discussion(_)
            | Cell::Divider(_)
            | Cell::Elasticsearch(_)
            | Cell::Heading(_)
            | Cell::Image(_)
            | Cell::ListItem(_)
            | Cell::Loki(_)
            | Cell::Prometheus(_)
            | Cell::Provider(_)
            | Cell::Text(_) => vec![],
        }
    }

    /// Returns the cell's text, if any.
    pub fn text(&self) -> Option<&str> {
        match self {
            Cell::Graph(cell) => Some(&cell.title),
            Cell::Log(cell) => Some(&cell.title),
            Cell::Provider(cell) => Some(&cell.title),
            Cell::Table(cell) => Some(&cell.title),
            cell => cell.content(),
        }
    }

    /// Returns a copy of the cell with the given content appended.
    #[must_use]
    pub fn with_appended_content(&self, content: &str) -> Self {
        self.with_content(&format!("{}{}", self.content().unwrap_or(""), content))
    }

    /// Returns a copy of the cell with the given rich-text appended.
    #[must_use]
    pub fn with_appended_rich_text(&self, text: &str, formatting: &[AnnotationWithOffset]) -> Self {
        let existing_text = self.text().unwrap_or_default();
        let existing_text_len = char_count(existing_text);
        self.with_rich_text(
            &format!("{}{}", existing_text, text),
            [
                self.formatting()
                    .cloned()
                    .unwrap_or_else(|| formatting_from_markdown(existing_text)),
                translate(formatting, existing_text_len as i64),
            ]
            .concat(),
        )
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
            Cell::Prometheus(cell) => Cell::Prometheus(PrometheusCell {
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
            Cell::Prometheus(cell) => Cell::Prometheus(PrometheusCell {
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
            Cell::Checkbox(cell) => Cell::Checkbox(cell.clone()),
            Cell::Code(cell) => Cell::Code(cell.clone()),
            Cell::Discussion(cell) => Cell::Discussion(cell.clone()),
            Cell::Divider(cell) => Cell::Divider(cell.clone()),
            Cell::Elasticsearch(cell) => Cell::Elasticsearch(cell.clone()),
            Cell::Graph(cell) => Cell::Graph(GraphCell {
                data: cell.data.as_ref().map(|data| {
                    data.iter()
                        .filter(|&(k, _)| source_ids.contains(k))
                        .map(|(k, v)| (k.clone(), v.clone()))
                        .collect()
                }),
                source_ids,
                ..cell.clone()
            }),
            Cell::Heading(cell) => Cell::Heading(cell.clone()),
            Cell::Image(cell) => Cell::Image(cell.clone()),
            Cell::ListItem(cell) => Cell::ListItem(cell.clone()),
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
            Cell::Loki(cell) => Cell::Loki(cell.clone()),
            Cell::Prometheus(cell) => Cell::Prometheus(cell.clone()),
            Cell::Provider(cell) => Cell::Provider(cell.clone()),
            Cell::Table(cell) => Cell::Table(TableCell {
                id: cell.id.clone(),
                data: cell.data.as_ref().map(|data| {
                    data.iter()
                        .filter(|(k, _)| source_ids.contains(k))
                        .map(|(k, v)| (k.clone(), v.clone()))
                        .collect()
                }),
                source_ids,
                ..cell.clone()
            }),
            Cell::Text(cell) => Cell::Text(cell.clone()),
        }
    }

    /// Returns a copy of the cell with its text replaced by the given text.
    #[must_use]
    pub fn with_text(&self, text: &str) -> Self {
        match self {
            Cell::Graph(cell) => Cell::Graph(GraphCell {
                id: cell.id.clone(),
                data: cell.data.clone(),
                formatting: Some(vec![]),
                source_ids: cell.source_ids.clone(),
                time_range: cell.time_range.clone(),
                title: text.to_owned(),
                ..*cell
            }),
            Cell::Log(cell) => Cell::Log(LogCell {
                id: cell.id.clone(),
                data: cell.data.clone(),
                title: text.to_owned(),
                formatting: Some(vec![]),
                time_range: cell.time_range.clone(),
                source_ids: cell.source_ids.clone(),
                display_fields: cell.display_fields.clone(),
                expanded_indices: cell.expanded_indices.clone(),
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
            Cell::Table(cell) => Cell::Table(TableCell {
                id: cell.id.clone(),
                data: cell.data.clone(),
                title: text.to_owned(),
                formatting: Some(vec![]),
                source_ids: cell.source_ids.clone(),
                ..*cell
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
            Cell::Graph(cell) => Cell::Graph(GraphCell {
                id: cell.id.clone(),
                data: cell.data.clone(),
                formatting: Some(formatting),
                source_ids: cell.source_ids.clone(),
                time_range: cell.time_range.clone(),
                title: text.to_owned(),
                ..*cell
            }),
            Cell::Table(cell) => Cell::Table(TableCell {
                id: cell.id.clone(),
                data: cell.data.clone(),
                formatting: Some(formatting),
                source_ids: cell.source_ids.clone(),
                title: text.to_owned(),
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
            | Cell::Image(_)
            | Cell::Loki(_)
            | Cell::Prometheus(_) => self.with_text(text),
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
            Cell::Prometheus(cell) => &mut cell.id,
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
            Cell::Graph(cell) => Some(&mut cell.formatting),
            Cell::Heading(cell) => Some(&mut cell.formatting),
            Cell::ListItem(cell) => Some(&mut cell.formatting),
            Cell::Log(cell) => Some(&mut cell.formatting),
            Cell::Provider(cell) => Some(&mut cell.formatting),
            Cell::Table(cell) => Some(&mut cell.formatting),
            Cell::Text(cell) => Some(&mut cell.formatting),
            Cell::Code(_)
            | Cell::Discussion(_)
            | Cell::Divider(_)
            | Cell::Elasticsearch(_)
            | Cell::Image(_)
            | Cell::Loki(_)
            | Cell::Prometheus(_) => None,
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
            Cell::Graph(cell) => Some(&mut cell.title),
            Cell::Heading(cell) => Some(&mut cell.content),
            Cell::ListItem(cell) => Some(&mut cell.content),
            Cell::Log(cell) => Some(&mut cell.title),
            Cell::Loki(cell) => Some(&mut cell.content),
            Cell::Prometheus(cell) => Some(&mut cell.content),
            Cell::Provider(cell) => Some(&mut cell.title),
            Cell::Table(cell) => Some(&mut cell.title),
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
    /// Optional formatting to be applied to the cell's title.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub formatting: Option<Formatting>,
    pub graph_type: GraphType,
    pub stacking_type: StackingType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    pub source_ids: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_range: Option<TimeRange>,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<BTreeMap<String, Vec<Series>>>,
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

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Serializable)]
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
    pub time_range: Option<TimeRange>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_fields: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub hide_similar_values: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub expanded_indices: Option<Vec<ExpandedIndex>>,
}

/// A single expanded row of log records, as identified by [key] and [index]
/// pointing into the source data of the LogCell.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, Serializable)]
#[fp(
    rust_plugin_module = "fiberplane::protocols::core",
    rust_wasmer_runtime_module = "fiberplane::protocols::core"
)]
#[serde(rename_all = "camelCase")]
pub struct ExpandedIndex {
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

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Eq, Serialize, Serializable)]
#[fp(
    rust_plugin_module = "fiberplane::protocols::core",
    rust_wasmer_runtime_module = "fiberplane::protocols::core"
)]
#[serde(rename_all = "camelCase")]
pub struct PrometheusCell {
    pub id: String,
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    pub source_ids: Vec<String>,
    /// Optional formatting to be applied to the cell's title.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub formatting: Option<Formatting>,

    #[serde(default = "default_title")]
    pub title: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<BTreeMap<String, Vec<Instant>>>,
}

fn default_title() -> String {
    "".to_string()
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
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize, Serializable)]
#[fp(
    rust_plugin_module = "fiberplane::protocols::core",
    rust_wasmer_runtime_module = "fiberplane::protocols::core"
)]
pub struct TimeRange {
    pub from: Timestamp,
    pub to: Timestamp,
}

/// Timestamp specified in seconds since the UNIX epoch, with subsecond precision.
pub type Timestamp = f64;

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize, Serializable)]
#[fp(
    rust_plugin_module = "fiberplane::protocols::core",
    rust_wasmer_runtime_module = "fiberplane::protocols::core"
)]
pub struct Metric {
    pub name: String,
    pub labels: HashMap<String, String>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Serializable)]
#[fp(
    rust_plugin_module = "fiberplane::protocols::core",
    rust_wasmer_runtime_module = "fiberplane::protocols::core"
)]
pub struct Point {
    pub timestamp: Timestamp,
    pub value: f64,
}

/// A single data-point in time, with meta-data about the metric it was taken from.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Serializable)]
#[fp(
    rust_plugin_module = "fiberplane::protocols::core",
    rust_wasmer_runtime_module = "fiberplane::protocols::core"
)]
#[serde(rename_all = "camelCase")]
pub struct Instant {
    pub metric: Metric,
    pub point: Point,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Serializable)]
#[fp(
    rust_plugin_module = "fiberplane::protocols::core",
    rust_wasmer_runtime_module = "fiberplane::protocols::core"
)]
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
#[fp(
    rust_plugin_module = "fiberplane::protocols::core",
    rust_wasmer_runtime_module = "fiberplane::protocols::core"
)]
#[serde(rename_all = "camelCase")]
pub struct Series {
    pub metric: Metric,
    pub points: Vec<Point>,
    #[serde(default = "default_visible")]
    pub visible: bool,
}

fn default_visible() -> bool {
    true
}

/// NotebookDataSource represents the way a data-source can be embedded in a
/// Notebook.
#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize, Serializable)]
#[fp(
    rust_plugin_module = "fiberplane::protocols::core",
    rust_wasmer_runtime_module = "fiberplane::protocols::core"
)]
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
#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize, Serializable)]
#[fp(
    rust_plugin_module = "fiberplane::protocols::core",
    rust_wasmer_runtime_module = "fiberplane::protocols::core"
)]
#[serde(rename_all = "camelCase")]
pub struct InlineDataSource {
    /// The actual data-source.
    pub data_source: DataSource,
}

/// OrganizationDataSource represents a data-source as stored for a organization
/// on the API.
#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize, Serializable)]
#[fp(
    rust_plugin_module = "fiberplane::protocols::core",
    rust_wasmer_runtime_module = "fiberplane::protocols::core"
)]
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
#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize, Serializable)]
#[fp(
    rust_plugin_module = "fiberplane::protocols::core",
    rust_wasmer_runtime_module = "fiberplane::protocols::core"
)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum DataSource {
    Prometheus(PrometheusDataSource),
    Proxy(ProxyDataSource),
    Elasticsearch(ElasticsearchDataSource),
    Loki(LokiDataSource),
    Sentry(SentryDataSource),
}

impl DataSource {
    pub fn data_source_type(&self) -> DataSourceType {
        match self {
            DataSource::Prometheus(_) => DataSourceType::Prometheus,
            DataSource::Proxy(_) => DataSourceType::Proxy,
            DataSource::Elasticsearch(_) => DataSourceType::Elasticsearch,
            DataSource::Loki(_) => DataSourceType::Loki,
            DataSource::Sentry(_) => DataSourceType::Sentry,
        }
    }
}

impl From<&DataSource> for DataSourceType {
    fn from(data_source: &DataSource) -> Self {
        data_source.data_source_type()
    }
}

#[derive(
    Clone, Debug, PartialEq, Serialize, Deserialize, Serializable, Hash, PartialOrd, Eq, Ord,
)]
#[fp(
    rust_plugin_module = "fiberplane::protocols::core",
    rust_wasmer_runtime_module = "fiberplane::protocols::core"
)]
#[serde(rename_all = "camelCase")]
pub enum DataSourceType {
    Prometheus,
    Proxy,
    Elasticsearch,
    Loki,
    Sentry,
}

impl From<&DataSourceType> for &'static str {
    fn from(data_source_type: &DataSourceType) -> Self {
        match data_source_type {
            DataSourceType::Prometheus => "prometheus",
            DataSourceType::Proxy => "proxy",
            DataSourceType::Elasticsearch => "elasticsearch",
            DataSourceType::Loki => "loki",
            DataSourceType::Sentry => "sentry",
        }
    }
}

#[derive(thiserror::Error, Debug, PartialEq, Eq)]
#[error("Unexpected data source type: {0}")]
pub struct UnexpectedDataSourceType(String);

impl FromStr for DataSourceType {
    type Err = UnexpectedDataSourceType;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "prometheus" => Ok(DataSourceType::Prometheus),
            "elasticsearch" => Ok(DataSourceType::Elasticsearch),
            "loki" => Ok(DataSourceType::Loki),
            "proxy" => Ok(DataSourceType::Proxy),
            _ => Err(UnexpectedDataSourceType(s.to_string())),
        }
    }
}

impl Display for DataSourceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

/// A data-source for Prometheus. Currently only requires a url. This should be
/// a full URL starting with http:// or https:// the domain, and optionally a
/// port and a path.
#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize, Serializable)]
#[fp(
    rust_plugin_module = "fiberplane::protocols::core",
    rust_wasmer_runtime_module = "fiberplane::protocols::core"
)]
#[serde(rename_all = "camelCase")]
pub struct PrometheusDataSource {
    pub url: String,
}

/// A data-source for Elasticsearch. Currently only requires a url. This should be
/// a full URL starting with http:// or https:// the domain, and optionally a
/// port and a path.
#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize, Serializable)]
#[fp(
    rust_plugin_module = "fiberplane::protocols::core",
    rust_wasmer_runtime_module = "fiberplane::protocols::core"
)]
#[serde(rename_all = "camelCase")]
pub struct ElasticsearchDataSource {
    pub url: String,
    /// Parse the timestamp out of fields with the given names
    #[serde(default)]
    pub timestamp_field_names: Vec<String>,
    /// Parse the body out of fields with the given names
    #[serde(default)]
    pub body_field_names: Vec<String>,
}

/// A data-source for Loki. Currently only requires a url. This should be
/// a full URL starting with http:// or https:// the domain, and optionally a
/// port and a path.
#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize, Serializable)]
#[fp(
    rust_plugin_module = "fiberplane::protocols::core",
    rust_wasmer_runtime_module = "fiberplane::protocols::core"
)]
#[serde(rename_all = "camelCase")]
pub struct LokiDataSource {
    pub url: String,
}

/// Relays requests for a data-source to a proxy server registered with the API.
#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize, Serializable)]
#[fp(
    rust_plugin_module = "fiberplane::protocols::core",
    rust_wasmer_runtime_module = "fiberplane::protocols::core"
)]
#[serde(rename_all = "camelCase")]
pub struct ProxyDataSource {
    /// ID of the proxy as known by the API.
    pub proxy_id: String,

    /// Name of the data source exposed by the proxy.
    pub data_source_name: String,

    /// Provider type
    pub data_source_type: DataSourceType,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize, Serializable)]
#[fp(
    rust_plugin_module = "fiberplane::protocols::core",
    rust_wasmer_runtime_module = "fiberplane::protocols::core"
)]
#[serde(rename_all = "camelCase")]
pub struct SentryDataSource {
    pub token: String,
    pub organization_slug: String,
    pub project_slug: String,
}

/// Labels that are associated with a Notebook.
#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize, Serializable)]
#[fp(
    rust_plugin_module = "fiberplane::protocols::core",
    rust_wasmer_runtime_module = "fiberplane::protocols::core"
)]
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

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Serializable, Error)]
#[fp(
    rust_plugin_module = "fiberplane::protocols::core",
    rust_wasmer_runtime_module = "fiberplane::protocols::core"
)]
#[serde(rename_all = "snake_case")]
pub enum LabelValidationError {
    #[error("The key in the label was empty")]
    EmptyKey,

    #[error("The name portion of the key was empty")]
    EmptyName,

    #[error("The name portion of the key was too long")]
    NameTooLong,

    #[error("The name portion of the key contains invalid characters")]
    NameInvalidCharacters,

    #[error("The prefix portion of the key was empty")]
    EmptyPrefix,

    #[error("The prefix portion of the key was too long")]
    PrefixTooLong,

    #[error("The prefix portion of the key contains invalid characters")]
    PrefixInvalidCharacters,

    #[error("The value is too long")]
    ValueTooLong,

    #[error("The value contains invalid characters")]
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
