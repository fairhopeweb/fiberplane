#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub use fiberplane_models::blobs::Blob;
pub use fiberplane_models::blobs::EncodedBlob;
pub use fiberplane_models::formatting::Annotation;
pub use fiberplane_models::formatting::AnnotationWithOffset;
pub use fiberplane_models::formatting::Mention;
pub use fiberplane_models::labels::Label;
pub use fiberplane_models::notebooks::Cell;
pub use fiberplane_models::notebooks::CheckboxCell;
pub use fiberplane_models::notebooks::CodeCell;
pub use fiberplane_models::notebooks::DiscussionCell;
pub use fiberplane_models::notebooks::DividerCell;
pub use fiberplane_models::notebooks::GraphCell;
pub use fiberplane_models::notebooks::GraphType;
pub use fiberplane_models::notebooks::HeadingCell;
pub use fiberplane_models::notebooks::HeadingType;
pub use fiberplane_models::notebooks::ImageCell;
pub use fiberplane_models::notebooks::ListItemCell;
pub use fiberplane_models::notebooks::ListType;
pub use fiberplane_models::notebooks::LogCell;
pub use fiberplane_models::notebooks::LogRecordIndex;
pub use fiberplane_models::notebooks::LogVisibilityFilter;
pub use fiberplane_models::notebooks::ProviderCell;
pub use fiberplane_models::notebooks::StackingType;
pub use fiberplane_models::notebooks::TableCell;
pub use fiberplane_models::notebooks::TableColumn;
pub use fiberplane_models::notebooks::TableRow;
pub use fiberplane_models::notebooks::TextCell;
pub use fiberplane_models::timestamps::Timestamp;

/// Defines a field that produces a boolean value.
///
/// For JSON/YAML encoding, the value will be represented as a native boolean.
/// In the case of "application/x-www-form-urlencoded", it will be represented
/// by the value defined in the `value` field, which will be either present or
/// not, similar to the encoding of HTML forms.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CheckboxField {
    /// Whether the checkbox should be initially checked if no query data is
    /// present.
    pub checked: bool,

    /// Suggested label to display along the checkbox.
    pub label: String,

    /// Name of the field as it will be included in the encoded query or config
    /// object.
    pub name: String,

    /// Whether the checkbox must be checked.
    ///
    /// This allows for the use case of implementing Terms of Service checkboxes
    /// in config forms.
    pub required: bool,

    /// Value of the field as it will be included in the encoded query. Note
    /// that only checked checkboxes will be included.
    ///
    /// If the data is encoded using either JSON or YAML, the checkbox state is
    /// encoded as a boolean and this value will not be used.
    pub value: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ConfigField {
    Checkbox(CheckboxField),
    Integer(IntegerField),
    Select(SelectField),
    Text(TextField),
}

pub type ConfigSchema = Vec<ConfigField>;

/// Defines a field that produces two `DateTime` values, a "from" and a "to"
/// value.
///
/// For JSON/YAML encoding, the value will be represented as an object with
/// `from` and `to` fields. In the case of "application/x-www-form-urlencoded",
/// it will be represented as a single string and the "from" and "to" parts will
/// be separated by a space.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DateTimeRangeField {
    /// Suggested label to display along the field.
    pub label: String,

    /// Name of the field as it will be included in the encoded query or config
    /// object.
    pub name: String,

    /// Suggested placeholder to display when there is no value.
    pub placeholder: String,

    /// Whether a value is required.
    pub required: bool,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Error {
    UnsupportedRequest,
    ValidationError {
        /// List of errors, so all fields that failed validation can
        /// be highlighted at once.
        errors: Vec<ValidationError>,
    },
    #[serde(rename_all = "camelCase")]
    Http {
        error: HttpRequestError,
    },
    #[serde(rename_all = "camelCase")]
    Data {
        message: String,
    },
    #[serde(rename_all = "camelCase")]
    Deserialization {
        message: String,
    },
    #[serde(rename_all = "camelCase")]
    Config {
        message: String,
    },
    NotFound,
    ProxyDisconnected,
    Invocation {
        message: String,
    },
    #[serde(rename_all = "camelCase")]
    Other {
        message: String,
    },
}

/// Defines a field that allows files to be uploaded as part of the query data.
///
/// Query data that includes files will be encoded using "multipart/form-data".
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FileField {
    /// Name of the field as it will be included in the encoded query or config
    /// object.
    pub name: String,

    /// Suggested label to display along the field.
    pub label: String,

    /// Whether multiple files may be uploaded.
    pub multiple: bool,

    /// Whether a file is required.
    pub required: bool,
}

pub type Formatting = Vec<AnnotationWithOffset>;

/// HTTP request options.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HttpRequest {
    pub url: String,
    pub method: HttpRequestMethod,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub headers: Option<HashMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub body: Option<bytes::Bytes>,
}

/// Possible errors that may happen during an HTTP request.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum HttpRequestError {
    Offline,
    NoRoute,
    ConnectionRefused,
    Timeout,
    ResponseTooBig,
    #[serde(rename_all = "camelCase")]
    ServerError {
        status_code: u16,
        response: bytes::Bytes,
    },
    #[serde(rename_all = "camelCase")]
    Other {
        reason: String,
    },
}

/// HTTP request method.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum HttpRequestMethod {
    Delete,
    Get,
    Head,
    Post,
}

/// Response to an HTTP request.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HttpResponse {
    pub body: bytes::Bytes,
    pub headers: HashMap<String, String>,
    pub status_code: u16,
}

/// Defines a field that allows integer numbers to be entered.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IntegerField {
    /// Name of the field as it will be included in the encoded query or config
    /// object.
    pub name: String,

    /// Suggested label to display along the field.
    pub label: String,

    /// Optional maximum value to be entered.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max: Option<i32>,

    /// Optional minimal value to be entered.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min: Option<i32>,

    /// Suggested placeholder to display when there is no value.
    pub placeholder: String,

    /// Whether a value is required.
    pub required: bool,

    /// Specifies the granularity that any specified numbers must adhere to.
    ///
    /// If omitted, `step` defaults to "1", meaning only integers are allowed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub step: Option<i32>,
}

/// Defines a field that allows labels to be selected.
///
/// For JSON/YAML encoding, the value will be represented as a string or an
/// array of strings, depending on the value of the `multiple` field. In the
/// case of "application/x-www-form-urlencoded", the value is always a single
/// string and multiple labels will be space-separated.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LabelField {
    /// Name of the field as it will be included in the encoded query or config
    /// object.
    pub name: String,

    /// Suggested label to display along the field (not to be confused with
    /// labels to be selected).
    pub label: String,

    /// Whether multiple labels may be selected.
    pub multiple: bool,

    /// Suggested placeholder to display when there is no value.
    pub placeholder: String,

    /// Whether a value is required.
    pub required: bool,
}

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
    Error {
        error: Error,
    },
    #[serde(rename_all = "camelCase")]
    LogRecords {
        log_records: Vec<LegacyLogRecord>,
    },
    StatusOk,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct LegacyTimeRange {
    pub from: LegacyTimestamp,
    pub to: LegacyTimestamp,
}

pub type LegacyTimestamp = f64;

pub type ProviderConfig = serde_json::Value;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProviderRequest {
    /// Query type that is part of the
    /// [Intent](https://www.notion.so/fiberplane/RFC-45-Provider-Protocol-2-0-Revised-4ec85a0233924b2db0010d8cdae75e16#c8ed5dfbfd764e6bbd5c5b79333f9d6e)
    /// through which the provider is invoked.
    pub query_type: String,

    /// Query data.
    ///
    /// This is usually populated from the [ProviderCell::query_data] field,
    /// meaning the MIME type will be `"application/x-www-form-urlencoded"`
    /// when produced by Studio's Query Builder.
    pub query_data: Blob,

    /// Configuration for the data source.
    pub config: ProviderConfig,

    /// Optional response from a previous invocation.
    /// May be used for implementing things like filtering without additional
    /// server roundtrip.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub previous_response: Option<Blob>,
}

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
#[serde(tag = "type", rename_all = "snake_case")]
pub enum QueryField {
    Checkbox(CheckboxField),
    DateTimeRange(DateTimeRangeField),
    File(FileField),
    Label(LabelField),
    Integer(IntegerField),
    Select(SelectField),
    Text(TextField),
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

/// Defines a field that allows selection from a predefined list of options.
///
/// Values to be selected from can be either hard-coded in the schema, or
/// (only for query forms) fetched on-demand the same way as auto-suggestions.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SelectField {
    /// Name of the field as it will be included in the encoded query or config
    /// object.
    pub name: String,

    /// Suggested label to display along the field.
    pub label: String,

    /// Whether multiple values may be selected.
    pub multiple: bool,

    /// A list of options to select from.
    ///
    /// For query forms, if this array is left empty, the auto-suggest mechanism
    /// can fetch options when the user starts typing in this field.
    pub options: Vec<String>,

    /// Suggested placeholder to display when there is no value.
    pub placeholder: String,

    /// An optional list of fields that should be filled in before allowing the
    /// user to fill in this field. This forces a certain ordering in the data
    /// entry, which enables richer auto-suggestions, since the filled in
    /// prerequisite fields can provide additional context.
    pub prerequisites: Vec<String>,

    /// Whether a value is required.
    pub required: bool,
}

/// Defines a query type supported by a provider.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SupportedQueryType {
    /// User-friendly label to use for the query type.
    pub label: String,

    /// The query type supported by the provider.
    ///
    /// There are predefined query types, such as "table" and "log", but
    /// providers may also implement custom query types, which it should prefix
    /// with "x-".
    pub query_type: String,

    /// The query schema defining the format of the `query_data` to be submitted
    /// with queries of this type.
    pub schema: QuerySchema,

    /// MIME types supported for extraction. Any MIME type specified here should
    /// be valid as an argument to `extract_data()` when passed a response from
    /// queries of this type.
    ///
    /// E.g.:
    /// ```
    /// vec![
    ///     "application/vnd.fiberplane.events",
    ///     "application/vnd.fiberplane.metrics"
    /// ];
    /// ```
    pub mime_types: Vec<String>,
}

/// Defines a free-form text entry field.
///
/// This is commonly used for filter text and query entry. For the latter case,
/// `supports_highlighting` can be set to `true` if the provider supports syntax
/// highlighting for the query language.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TextField {
    /// Suggested label to display along the form field.
    pub label: String,

    /// Whether multi-line input is useful for this provider.
    pub multiline: bool,

    /// Name of the field as it will be included in the encoded query or config
    /// object.
    pub name: String,

    /// Suggested placeholder to display when there is no value.
    pub placeholder: String,

    /// An optional list of fields that should be filled in before allowing the
    /// user to fill in this field. This forces a certain ordering in the data
    /// entry, which enables richer auto-suggestions, since the filled in
    /// prerequisite fields can provide additional context.
    pub prerequisites: Vec<String>,

    /// Whether a value is required.
    pub required: bool,

    /// Whether the provider implements syntax highlighting for this field.
    /// See `highlight_field()` in the protocol definition.
    pub supports_highlighting: bool,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ValidationError {
    /// Refers to a field from the query schema.
    pub field_name: String,

    /// Description of why the validation failed.
    pub message: String,
}
