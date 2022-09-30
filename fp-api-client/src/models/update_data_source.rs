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
pub struct UpdateDataSource {
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "config", skip_serializing_if = "Option::is_none")]
    pub config: Option<serde_json::Value>,
}

impl UpdateDataSource {
    pub fn new() -> UpdateDataSource {
        UpdateDataSource {
            description: None,
            config: None,
        }
    }
}
