use bytes::Bytes;
use fp_bindgen::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// HTTP request options.
#[derive(Clone, Debug, Deserialize, Serialize, Serializable)]
#[fp(rust_plugin_module = "fiberplane::protocols::providers")]
#[serde(rename_all = "camelCase")]
pub struct HttpRequest {
    pub url: String,
    pub method: HttpRequestMethod,
    pub headers: Option<HashMap<String, String>>,
    pub body: Option<Bytes>,
}

/// Possible errors that may happen during an HTTP request.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, Serializable)]
#[fp(rust_plugin_module = "fiberplane::protocols::providers")]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum HttpRequestError {
    Offline,
    NoRoute,
    ConnectionRefused,
    Timeout,
    ResponseTooBig,
    #[fp(rename_all = "camelCase")]
    ServerError {
        status_code: u16,
        response: Bytes,
    },
    #[fp(rename_all = "camelCase")]
    Other {
        reason: String,
    },
}

/// HTTP request method.
// Note: we use SCREAMING_SNAKE_CASE here because this is
// effectively a constant
#[derive(Clone, Debug, Deserialize, Serialize, Serializable)]
#[fp(rust_plugin_module = "fiberplane::protocols::providers")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum HttpRequestMethod {
    Delete,
    Get,
    Head,
    Post,
}

/// Response to an HTTP request.
#[derive(Clone, Debug, Deserialize, Serialize, Serializable)]
#[fp(rust_plugin_module = "fiberplane::protocols::providers")]
#[serde(rename_all = "camelCase")]
pub struct HttpResponse {
    pub body: Bytes,
    pub headers: HashMap<String, String>,
    pub status_code: u16,
}
