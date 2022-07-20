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
pub struct CommentThreadItem {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "content")]
    pub content: String,
    #[serde(rename = "formatting", skip_serializing_if = "Option::is_none")]
    pub formatting: Option<Vec<crate::models::Annotation>>,
    #[serde(rename = "createdBy")]
    pub created_by: Box<crate::models::UserSummary>,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
    #[serde(rename = "type")]
    pub _type: crate::models::ThreadItemType,
}

impl CommentThreadItem {
    pub fn new(
        id: String,
        content: String,
        created_by: crate::models::UserSummary,
        created_at: String,
        updated_at: String,
        _type: crate::models::ThreadItemType,
    ) -> CommentThreadItem {
        CommentThreadItem {
            id,
            content,
            formatting: None,
            created_by: Box::new(created_by),
            created_at,
            updated_at,
            _type,
        }
    }
}
