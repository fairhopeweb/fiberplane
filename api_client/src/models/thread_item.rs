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
#[serde(tag = "type")]
pub enum ThreadItem {
    #[serde(rename = "comment")]
    CommentThreadItem {
        #[serde(rename = "id")]
        id: String,
        #[serde(rename = "content")]
        content: String,
        #[serde(rename = "formatting", skip_serializing_if = "Option::is_none")]
        formatting: Option<Vec<crate::models::Annotation>>,
        #[serde(rename = "createdBy")]
        created_by: Box<crate::models::UserSummary>,
        #[serde(rename = "createdAt")]
        created_at: String,
        #[serde(rename = "updatedAt")]
        updated_at: String,
    },
    #[serde(rename = "comment_delete")]
    CommentDeleteThreadItem {
        #[serde(rename = "id")]
        id: String,
        #[serde(rename = "createdBy")]
        created_by: Box<crate::models::UserSummary>,
        /// Timestamp when the original comment was created
        #[serde(rename = "createdAt")]
        created_at: String,
        #[serde(rename = "deletedAt")]
        deleted_at: String,
    },
    #[serde(rename = "status_change")]
    StatusChangeThreadItem {
        #[serde(rename = "id")]
        id: String,
        #[serde(rename = "status")]
        status: crate::models::ThreadStatus,
        #[serde(rename = "createdBy")]
        created_by: Box<crate::models::UserSummary>,
        #[serde(rename = "createdAt")]
        created_at: String,
    },
}
