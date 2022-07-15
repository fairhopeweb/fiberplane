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
pub struct TableCell {
    #[serde(rename = "type")]
    pub _type: crate::models::CellType,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "readOnly", skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    #[serde(rename = "sourceIds")]
    pub source_ids: Vec<String>,
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "formatting", skip_serializing_if = "Option::is_none")]
    pub formatting: Option<Vec<crate::models::Annotation>>,
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<::std::collections::HashMap<String, Vec<crate::models::Instant>>>,
}

impl TableCell {
    pub fn new(
        _type: crate::models::CellType,
        id: String,
        source_ids: Vec<String>,
        title: String,
    ) -> TableCell {
        TableCell {
            _type,
            id,
            read_only: None,
            source_ids,
            title,
            formatting: None,
            data: None,
        }
    }
}
