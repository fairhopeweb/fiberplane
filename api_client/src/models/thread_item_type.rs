/*
 * Fiberplane API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0
 *
 * Generated by: https://openapi-generator.tech
 */

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ThreadItemType {
    #[serde(rename = "comment")]
    Comment,
    #[serde(rename = "status_change")]
    StatusChange,
    #[serde(rename = "comment_delete")]
    CommentDelete,
}

impl ToString for ThreadItemType {
    fn to_string(&self) -> String {
        match self {
            Self::Comment => String::from("comment"),
            Self::StatusChange => String::from("status_change"),
            Self::CommentDelete => String::from("comment_delete"),
        }
    }
}
