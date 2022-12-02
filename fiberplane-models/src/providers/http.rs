use crate::debug_print_bytes;
use bytes::Bytes;
#[cfg(feature = "fp-bindgen")]
use fp_bindgen::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::{self, Debug, Formatter};

/// HTTP request options.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(
    feature = "fp-bindgen",
    derive(Serializable),
    fp(rust_plugin_module = "fiberplane_models::providers")
)]
#[serde(rename_all = "camelCase")]
pub struct HttpRequest {
    pub url: String,
    pub method: HttpRequestMethod,
    pub headers: Option<HashMap<String, String>>,
    pub body: Option<Bytes>,
}

/// Possible errors that may happen during an HTTP request.
#[derive(Clone, Deserialize, Eq, PartialEq, Serialize)]
#[cfg_attr(
    feature = "fp-bindgen",
    derive(Serializable),
    fp(rust_plugin_module = "fiberplane_models::providers")
)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum HttpRequestError {
    Offline,
    NoRoute,
    ConnectionRefused,
    Timeout,
    ResponseTooBig,
    #[cfg_attr(feature = "fp-bindgen", fp(rename_all = "camelCase"))]
    ServerError {
        status_code: u16,
        response: Bytes,
    },
    #[cfg_attr(feature = "fp-bindgen", fp(rename_all = "camelCase"))]
    Other {
        reason: String,
    },
}

impl Debug for HttpRequestError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Offline => f.write_str("Offline"),
            Self::NoRoute => f.write_str("NoRoute"),
            Self::ConnectionRefused => f.write_str("ConnectionRefused"),
            Self::Timeout => f.write_str("Timeout"),
            Self::ResponseTooBig => f.write_str("ResponseTooBig"),
            Self::ServerError {
                status_code,
                response,
            } => f
                .debug_struct("ServerError")
                .field("status_code", status_code)
                .field("response_length", &response.len())
                .field("response", &debug_print_bytes(response))
                .finish(),
            Self::Other { reason } => f.debug_struct("Other").field("reason", reason).finish(),
        }
    }
}

/// HTTP request method.
// Note: we use SCREAMING_SNAKE_CASE here because this is
// effectively a constant
#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(
    feature = "fp-bindgen",
    derive(Serializable),
    fp(rust_plugin_module = "fiberplane_models::providers")
)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum HttpRequestMethod {
    Delete,
    Get,
    Head,
    Post,
}

/// Response to an HTTP request.
#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(
    feature = "fp-bindgen",
    derive(Serializable),
    fp(rust_plugin_module = "fiberplane_models::providers")
)]
#[serde(rename_all = "camelCase")]
pub struct HttpResponse {
    pub body: Bytes,
    pub headers: HashMap<String, String>,
    pub status_code: u16,
}

impl Debug for HttpResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("HttpResponse")
            .field("status_code", &self.status_code)
            .field("headers", &self.headers)
            .field("body_length", &self.body.len())
            .field("body", &debug_print_bytes(&self.body))
            .finish()
    }
}
