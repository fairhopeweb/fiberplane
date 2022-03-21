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
pub struct DataSourceAndProxySummary {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "type")]
    pub _type: crate::models::DataSourceType,
    #[serde(rename = "status")]
    pub status: crate::models::DataSourceConnectionStatus,
    #[serde(rename = "errorMessage", skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "proxy")]
    pub proxy: Box<crate::models::ProxySummary>,
}

impl DataSourceAndProxySummary {
    pub fn new(name: String, _type: crate::models::DataSourceType, status: crate::models::DataSourceConnectionStatus, proxy: crate::models::ProxySummary) -> DataSourceAndProxySummary {
        DataSourceAndProxySummary {
            name,
            _type,
            status,
            error_message: None,
            proxy: Box::new(proxy),
        }
    }
}


