use crate::{names::Name, providers::Error};
use base64uuid::Base64Uuid;
#[cfg(feature = "fp-bindgen")]
use fp_bindgen::prelude::Serializable;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use std::collections::BTreeMap;
use strum_macros::Display;
use time::{serde::rfc3339, OffsetDateTime};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DataSource {
    pub name: Name,
    pub proxy_name: Option<Name>,
    pub id: Base64Uuid,
    pub provider_type: String,
    #[serde(default)]
    pub protocol_version: u8,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config: Option<Map<String, Value>>,
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub status: Option<DataSourceStatus>,
    #[serde(with = "rfc3339")]
    pub created_at: OffsetDateTime,
    #[serde(with = "rfc3339")]
    pub updated_at: OffsetDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Display)]
#[cfg_attr(
    feature = "fp-bindgen",
    derive(Serializable),
    fp(
        rust_plugin_module = "fiberplane_models::data_sources",
        rust_wasmer_runtime_module = "fiberplane_models::data_sources"
    )
)]
#[serde(tag = "status", content = "error", rename_all = "snake_case")]
pub enum DataSourceStatus {
    Connected,
    Error(Error),
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NewDataSource {
    pub name: Name,
    pub provider_type: String,
    #[serde(default)]
    pub protocol_version: u8,
    pub description: Option<String>,
    pub config: Map<String, Value>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UpdateDataSource {
    pub description: Option<String>,
    pub config: Option<Map<String, Value>>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
#[cfg_attr(
    feature = "fp-bindgen",
    derive(Serializable),
    fp(
        rust_plugin_module = "fiberplane_models::data_sources",
        rust_wasmer_runtime_module = "fiberplane_models::data_sources"
    )
)]
#[serde(rename_all = "camelCase")]
pub struct SelectedDataSource {
    /// The name of the selected data source
    pub name: Name,

    /// If this is a proxy data source, the name of the proxy
    pub proxy_name: Option<Name>,
}

pub type ProviderType = String;

/// This is a map from provider type to the selected data source for that type.
pub type SelectedDataSources = BTreeMap<ProviderType, SelectedDataSource>;

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use serde_json::json;

    #[test]
    fn status_serialization() {
        let serialized = serde_json::to_value(&DataSourceStatus::Connected).unwrap();
        assert_eq!(serialized, json!({"status":"connected"}));

        assert_eq!(
            serde_json::to_value(&DataSourceStatus::Error(Error::NotFound)).unwrap(),
            json!({
                "status": "error",
                "error": {
                    "type": "not_found",
                }
            })
        );
    }
}
