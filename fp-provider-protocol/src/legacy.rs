#![allow(dead_code)]

use bytes::Bytes;
use fiberplane::protocols::{
    core::{LegacyTimeRange, LegacyTimestamp},
    providers::Error,
};
use fp_bindgen::prelude::Serializable;
use std::collections::HashMap;

/// Legacy `ProviderRequest` from the Provider 1.0 protocol.
#[non_exhaustive]
#[derive(Serializable, Debug)]
#[fp(tag = "type", rename_all = "snake_case")]
pub enum LegacyProviderRequest {
    Proxy(ProxyRequest),
    Logs(QueryLogs),
    /// Check data source status, any issue will be returned as `Error`
    Status,
}

/// Relays requests for a data-source to a proxy server registered with the API.
#[derive(Serializable, Debug)]
#[fp(rename_all = "camelCase")]
pub struct ProxyRequest {
    /// ID of the proxy as known by the API.
    pub proxy_id: String,

    /// Name of the data source exposed by the proxy.
    pub data_source_name: String,

    /// Request data to send to the proxy
    pub request: Bytes,
}

#[derive(Serializable, Debug)]
#[fp(rename_all = "camelCase")]
pub struct QueryLogs {
    pub query: String,
    pub limit: Option<u32>,
    pub time_range: LegacyTimeRange,
}

/// Legacy `ProviderResponse` from the 1.0 protocol.
#[non_exhaustive]
#[derive(Serializable, Debug)]
#[fp(tag = "type", rename_all = "snake_case")]
pub enum LegacyProviderResponse {
    #[fp(rename_all = "camelCase")]
    Error {
        error: Error,
    },
    #[fp(rename_all = "camelCase")]
    LogRecords {
        log_records: Vec<LegacyLogRecord>,
    },
    StatusOk,
}

/// An individual log record
#[derive(Serializable, Debug)]
#[fp(rename_all = "camelCase")]
pub struct LegacyLogRecord {
    pub timestamp: LegacyTimestamp,
    pub body: String,
    pub attributes: HashMap<String, String>,
    pub resource: HashMap<String, String>,
    // TODO these should really be [u8; 16], but arrays are
    // not currently supported by fp-bindgen
    pub trace_id: Option<Bytes>,
    pub span_id: Option<Bytes>,
}
