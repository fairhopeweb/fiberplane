use fp_bindgen::prelude::Serializable;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{collections::BTreeMap, time::SystemTime};
use time::OffsetDateTime;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, Serializable)]
#[fp(rust_plugin_module = "fiberplane::protocols::providers")]
pub struct OtelTimestamp(#[serde(with = "time::serde::rfc3339")] pub OffsetDateTime);

impl From<OffsetDateTime> for OtelTimestamp {
    fn from(time: OffsetDateTime) -> Self {
        Self(time)
    }
}

impl From<SystemTime> for OtelTimestamp {
    fn from(time: SystemTime) -> Self {
        Self(OffsetDateTime::from(time))
    }
}

impl From<OtelTimestamp> for OffsetDateTime {
    fn from(timestamp: OtelTimestamp) -> Self {
        timestamp.0
    }
}

/// A single event.
///
/// Events occur at a given time and optionally last until a given end time.
/// They may contain both event-specific metadata as well as OpenTelemetry
/// metadata.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, Serializable)]
#[fp(rust_plugin_module = "fiberplane::protocols::providers")]
#[serde(rename_all = "camelCase")]
pub struct Event {
    pub time: OtelTimestamp,
    pub end_time: Option<OtelTimestamp>,

    #[serde(flatten)]
    pub otel: OtelMetadata,

    pub title: String,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub severity: Option<OtelSeverityNumber>,

    pub labels: BTreeMap<String, String>,
}

/// A single metric value.
///
/// Metric values are taken at a specific timestamp and contain a floating-point
/// value as well as OpenTelemetry metadata.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Serializable)]
#[fp(rust_plugin_module = "fiberplane::protocols::providers")]
#[serde(rename_all = "camelCase")]
pub struct Metric {
    pub time: OtelTimestamp,

    #[serde(flatten)]
    pub otel: OtelMetadata,

    pub value: f64,
}

/// Metadata following the OpenTelemetry metadata spec.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize, Serializable)]
#[fp(rust_plugin_module = "fiberplane::protocols::providers")]
#[serde(rename_all = "camelCase")]
pub struct OtelMetadata {
    pub attributes: BTreeMap<String, Value>,

    pub resource: BTreeMap<String, Value>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trace_id: Option<OtelTraceId>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub span_id: Option<OtelSpanId>,
}

/// SeverityNumber, as specified by OpenTelemetry:
///  https://github.com/open-telemetry/opentelemetry-specification/blob/main/specification/logs/data-model.md#field-severitynumber
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, Serializable)]
#[fp(rust_plugin_module = "fiberplane::protocols::providers")]
#[serde(rename_all = "camelCase")]
pub struct OtelSeverityNumber(u8);

/// Span ID, as specified by OpenTelemetry:
///  https://opentelemetry.io/docs/reference/specification/overview/#spancontext
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, Serializable)]
#[fp(rust_plugin_module = "fiberplane::protocols::providers")]
#[serde(rename_all = "camelCase")]
pub struct OtelSpanId([u8; 8]);

/// Trace ID, as specified by OpenTelemetry:
///  https://opentelemetry.io/docs/reference/specification/overview/#spancontext
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, Serializable)]
#[fp(rust_plugin_module = "fiberplane::protocols::providers")]
#[serde(rename_all = "camelCase")]
pub struct OtelTraceId([u8; 16]);

/// A series of metrics over time, with metadata.
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize, Serializable)]
#[fp(rust_plugin_module = "fiberplane::protocols::providers")]
#[serde(rename_all = "camelCase")]
pub struct Timeseries {
    pub name: String,
    pub labels: BTreeMap<String, String>,
    pub metrics: Vec<Metric>,
    #[serde(flatten)]
    pub otel: OtelMetadata,

    /// Whether the series should be rendered. Can be toggled by the user.
    #[serde(default)]
    pub visible: bool,
}
