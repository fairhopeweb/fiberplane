use crate::formatting::Formatting;
use base64uuid::Base64Uuid;
#[cfg(feature = "fp-bindgen")]
use fp_bindgen::prelude::Serializable;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[cfg_attr(
    feature = "fp-bindgen",
    derive(Serializable),
    fp(rust_module = "fiberplane_models::comments")
)]
#[serde(rename_all = "camelCase")]
pub struct Thread {
    pub id: Base64Uuid,
    pub items: Vec<ThreadItem>,
    pub status: ThreadStatus,
    pub created_by: UserSummary,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    pub updated_at: OffsetDateTime,
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[cfg_attr(
    feature = "fp-bindgen",
    derive(Serializable),
    fp(rust_module = "fiberplane_models::comments")
)]
#[serde(rename_all = "snake_case")]
pub enum ThreadStatus {
    Open,
    Resolved,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[cfg_attr(
    feature = "fp-bindgen",
    derive(Serializable),
    fp(rust_module = "fiberplane_models::comments")
)]
#[serde(rename_all = "camelCase")]
pub struct ThreadSummary {
    pub id: Base64Uuid,
    pub item_count: u32,
    pub first_item: Option<ThreadItem>,
    /// These are sorted in chronological order so the last one is the most recent.
    #[serde(default)]
    pub recent_items: Vec<ThreadItem>,
    pub status: ThreadStatus,
    pub created_by: UserSummary,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    pub updated_at: OffsetDateTime,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[cfg_attr(
    feature = "fp-bindgen",
    derive(Serializable),
    fp(rust_module = "fiberplane_models::comments")
)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ThreadItem {
    Comment(Comment),
    StatusChange(ThreadStatusChange),
    CommentDelete(CommentDelete),
}

impl ThreadItem {
    pub fn id(&self) -> Base64Uuid {
        match self {
            ThreadItem::Comment(item) => item.id,
            ThreadItem::StatusChange(item) => item.id,
            ThreadItem::CommentDelete(item) => item.id,
        }
    }

    pub fn created_at(&self) -> &OffsetDateTime {
        match self {
            ThreadItem::Comment(item) => &item.created_at,
            ThreadItem::StatusChange(item) => &item.created_at,
            ThreadItem::CommentDelete(item) => &item.created_at,
        }
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[cfg_attr(
    feature = "fp-bindgen",
    derive(Serializable),
    fp(rust_module = "fiberplane_models::comments")
)]
#[serde(rename_all = "camelCase")]
pub struct ThreadStatusChange {
    pub id: Base64Uuid,
    pub status: ThreadStatus,
    pub created_by: UserSummary,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[cfg_attr(
    feature = "fp-bindgen",
    derive(Serializable),
    fp(rust_module = "fiberplane_models::comments")
)]
#[serde(rename_all = "camelCase")]
pub struct CommentDelete {
    pub id: Base64Uuid,
    pub created_by: UserSummary,
    /// Timestamp when the original comment was created
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    pub deleted_at: OffsetDateTime,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[cfg_attr(
    feature = "fp-bindgen",
    derive(Serializable),
    fp(rust_module = "fiberplane_models::comments")
)]
#[serde(rename_all = "camelCase")]
pub struct Comment {
    pub id: Base64Uuid,
    pub created_by: UserSummary,
    pub content: String, // limit of 2048 characters
    pub formatting: Formatting,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    pub updated_at: OffsetDateTime,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[cfg_attr(
    feature = "fp-bindgen",
    derive(Serializable),
    fp(rust_module = "fiberplane_models::comments")
)]
#[serde(rename_all = "camelCase")]
pub struct UserSummary {
    pub id: String,
    pub name: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[cfg_attr(
    feature = "fp-bindgen",
    derive(Serializable),
    fp(rust_module = "fiberplane_models::comments")
)]
#[serde(rename_all = "camelCase")]
pub struct NewComment {
    pub id: Option<Base64Uuid>,
    pub content: String,
    #[serde(default)]
    pub formatting: Formatting,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[cfg_attr(
    feature = "fp-bindgen",
    derive(Serializable),
    fp(rust_module = "fiberplane_models::comments")
)]
#[serde(rename_all = "camelCase")]
pub struct UpdateComment {
    pub content: String,
    #[serde(default)]
    pub formatting: Formatting,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[cfg_attr(
    feature = "fp-bindgen",
    derive(Serializable),
    fp(rust_module = "fiberplane_models::comments")
)]
#[serde(rename_all = "camelCase")]
pub struct NewThread {
    pub id: Option<Base64Uuid>,
    pub referenced_cell_id: Option<Base64Uuid>,
    pub comment: Option<NewComment>,
}
