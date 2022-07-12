/*
 * Fiberplane API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogRecord {
    #[serde(rename = "timestamp")]
    pub timestamp: f64,
    #[serde(rename = "body")]
    pub body: String,
    #[serde(rename = "attributes")]
    pub attributes: ::std::collections::HashMap<String, String>,
    #[serde(rename = "resource")]
    pub resource: ::std::collections::HashMap<String, String>,
    #[serde(rename = "traceId", skip_serializing_if = "Option::is_none")]
    pub trace_id: Option<String>,
    #[serde(rename = "spanId", skip_serializing_if = "Option::is_none")]
    pub span_id: Option<String>,
}

impl LogRecord {
    pub fn new(timestamp: f64, body: String, attributes: ::std::collections::HashMap<String, String>, resource: ::std::collections::HashMap<String, String>) -> LogRecord {
        LogRecord {
            timestamp,
            body,
            attributes,
            resource,
            trace_id: None,
            span_id: None,
        }
    }
}


