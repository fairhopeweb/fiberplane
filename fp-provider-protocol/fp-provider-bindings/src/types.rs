#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub use fiberplane::protocols::formatting::Annotation;
pub use fiberplane::protocols::formatting::AnnotationWithOffset;
pub use fiberplane::protocols::blobs::Blob;
pub use fiberplane::protocols::providers::ButtonField;
pub use fiberplane::protocols::core::Cell;
pub use fiberplane::protocols::core::CheckboxCell;
pub use fiberplane::protocols::providers::CheckboxField;
pub use fiberplane::protocols::core::CodeCell;
pub use fiberplane::protocols::providers::DateField;
pub use fiberplane::protocols::providers::DateTimeField;
pub use fiberplane::protocols::providers::DateTimeRangeField;
pub use fiberplane::protocols::core::DiscussionCell;
pub use fiberplane::protocols::core::DividerCell;
pub use fiberplane::protocols::blobs::EncodedBlob;
pub use fiberplane::protocols::providers::Error;
pub use fiberplane::protocols::providers::FileField;
pub use fiberplane::protocols::core::GraphCell;
pub use fiberplane::protocols::core::GraphType;
pub use fiberplane::protocols::core::HeadingCell;
pub use fiberplane::protocols::core::HeadingType;
pub use fiberplane::protocols::providers::HttpRequest;
pub use fiberplane::protocols::providers::HttpRequestError;
pub use fiberplane::protocols::providers::HttpRequestMethod;
pub use fiberplane::protocols::providers::HttpResponse;
pub use fiberplane::protocols::core::ImageCell;
pub use fiberplane::protocols::labels::Label;
pub use fiberplane::protocols::providers::LabelField;
pub use fiberplane::protocols::core::ListItemCell;
pub use fiberplane::protocols::core::ListType;
pub use fiberplane::protocols::core::LogCell;
pub use fiberplane::protocols::core::LogRecordIndex;
pub use fiberplane::protocols::core::LogVisibilityFilter;
pub use fiberplane::protocols::formatting::Mention;
pub use fiberplane::protocols::providers::NumberField;
pub use fiberplane::protocols::core::ProviderCell;
pub use fiberplane::protocols::providers::ProviderRequest;
pub use fiberplane::protocols::providers::QueryField;
pub use fiberplane::protocols::providers::SelectField;
pub use fiberplane::protocols::core::StackingType;
pub use fiberplane::protocols::providers::SupportedQueryType;
pub use fiberplane::protocols::core::TableCell;
pub use fiberplane::protocols::core::TableColumn;
pub use fiberplane::protocols::core::TableRow;
pub use fiberplane::protocols::core::TextCell;
pub use fiberplane::protocols::providers::TextField;
pub use fiberplane::protocols::core::Timestamp;
pub use fiberplane::protocols::providers::ValidationError;

pub type Formatting = Vec<AnnotationWithOffset>;

/// An individual log record
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LegacyLogRecord {
    pub timestamp: LegacyTimestamp,
    pub body: String,
    pub attributes: HashMap<String, String>,
    pub resource: HashMap<String, String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trace_id: Option<bytes::Bytes>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub span_id: Option<bytes::Bytes>,
}

/// Legacy `ProviderRequest` from the Provider 1.0 protocol.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum LegacyProviderRequest {
    Proxy(ProxyRequest),
    Logs(QueryLogs),
    /// Check data source status, any issue will be returned as `Error`
    Status,
}

/// Legacy `ProviderResponse` from the 1.0 protocol.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum LegacyProviderResponse {
    #[serde(rename_all = "camelCase")]
    Error { error: Error },
    #[serde(rename_all = "camelCase")]
    LogRecords { log_records: Vec<LegacyLogRecord> },
    StatusOk,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct LegacyTimeRange {
    pub from: LegacyTimestamp,
    pub to: LegacyTimestamp,
}

pub type LegacyTimestamp = f64;

pub type ProviderConfig = serde_json::Value;

/// Relays requests for a data-source to a proxy server registered with the API.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProxyRequest {
    /// ID of the proxy as known by the API.
    pub proxy_id: String,

    /// Name of the data source exposed by the proxy.
    pub data_source_name: String,

    /// Request data to send to the proxy
    pub request: bytes::Bytes,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryLogs {
    pub query: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    pub time_range: LegacyTimeRange,
}

pub type QuerySchema = Vec<QueryField>;
