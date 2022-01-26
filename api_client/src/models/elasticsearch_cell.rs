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
pub struct ElasticsearchCell {
    #[serde(rename = "type")]
    pub _type: crate::models::CellType,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "content")]
    pub content: String,
    #[serde(rename = "readOnly", skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
}

impl ElasticsearchCell {
    pub fn new(_type: crate::models::CellType, id: String, content: String) -> ElasticsearchCell {
        ElasticsearchCell {
            _type,
            id,
            content,
            read_only: None,
        }
    }
}


