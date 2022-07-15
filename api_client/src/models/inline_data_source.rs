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
pub struct InlineDataSource {
    #[serde(rename = "dataSource")]
    pub data_source: Box<crate::models::DataSource>,
}

impl InlineDataSource {
    pub fn new(data_source: crate::models::DataSource) -> InlineDataSource {
        InlineDataSource {
            data_source: Box::new(data_source),
        }
    }
}
