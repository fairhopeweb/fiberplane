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
pub struct UpdateWorkspace {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "owner", skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    /// This is a mapping from the provider type to the data source selected for that type
    #[serde(rename = "defaultDataSources", skip_serializing_if = "Option::is_none")]
    pub default_data_sources:
        Option<::std::collections::HashMap<String, crate::models::SelectedDataSource>>,
}

impl UpdateWorkspace {
    pub fn new() -> UpdateWorkspace {
        UpdateWorkspace {
            name: None,
            owner: None,
            default_data_sources: None,
        }
    }
}
