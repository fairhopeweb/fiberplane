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
pub struct ThreadSummary {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "firstItem", skip_serializing_if = "Option::is_none")]
    pub first_item: Option<Box<crate::models::ThreadItem>>,
    /// Most recent thread items, sorted in chronological order
    #[serde(rename = "recentItems", skip_serializing_if = "Option::is_none")]
    pub recent_items: Option<Vec<crate::models::ThreadItem>>,
    #[serde(rename = "status")]
    pub status: crate::models::ThreadStatus,
    #[serde(rename = "createdBy")]
    pub created_by: Box<crate::models::UserSummary>,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
}

impl ThreadSummary {
    pub fn new(
        id: String,
        status: crate::models::ThreadStatus,
        created_by: crate::models::UserSummary,
        created_at: String,
        updated_at: String,
    ) -> ThreadSummary {
        ThreadSummary {
            id,
            first_item: None,
            recent_items: None,
            status,
            created_by: Box::new(created_by),
            created_at,
            updated_at,
        }
    }
}
