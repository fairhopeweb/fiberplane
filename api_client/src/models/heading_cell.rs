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
pub struct HeadingCell {
    #[serde(rename = "type")]
    pub _type: crate::models::CellType,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "headingType")]
    pub heading_type: HeadingType,
    #[serde(rename = "content")]
    pub content: String,
    #[serde(rename = "readOnly", skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
}

impl HeadingCell {
    pub fn new(_type: crate::models::CellType, id: String, heading_type: HeadingType, content: String) -> HeadingCell {
        HeadingCell {
            _type,
            id,
            heading_type,
            content,
            read_only: None,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum HeadingType {
    #[serde(rename = "h1")]
    H1,
    #[serde(rename = "h2")]
    H2,
    #[serde(rename = "h3")]
    H3,
}

