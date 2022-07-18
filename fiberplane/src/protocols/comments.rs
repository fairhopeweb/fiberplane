use crate::protocols::formatting::Formatting;
use fp_bindgen::prelude::Serializable;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize, Serializable)]
#[serde(rename_all = "camelCase")]
pub struct Thread {
    pub id: String,
    pub items: Vec<ThreadItem>,
    pub status: ThreadStatus,
    pub created_by: UserSummary,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq, Serialize, Serializable)]
#[serde(rename_all = "snake_case")]
pub enum ThreadStatus {
    Open,
    Resolved,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize, Serializable)]
#[serde(rename_all = "camelCase")]
pub struct ThreadSummary {
    pub id: String,
    pub first_item: ThreadItem,
    /// These are sorted in chronological order so the last one is the most recent.
    pub recent_items: Vec<ThreadItem>,
    pub status: ThreadStatus,
    pub created_by: UserSummary,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize, Serializable)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ThreadItem {
    Comment(Comment),
    StatusChange(ThreadStatusChange),
    CommentDeleted(CommentDeleted),
}

impl ThreadItem {
    pub fn id(&self) -> &str {
        match self {
            ThreadItem::Comment(item) => &item.id,
            ThreadItem::StatusChange(item) => &item.id,
            ThreadItem::CommentDeleted(item) => &item.id,
        }
    }

    pub fn created_at(&self) -> &OffsetDateTime {
        match self {
            ThreadItem::Comment(item) => &item.created_at,
            ThreadItem::StatusChange(item) => &item.created_at,
            ThreadItem::CommentDeleted(item) => &item.created_at,
        }
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize, Serializable)]
#[serde(rename_all = "camelCase")]
pub struct ThreadStatusChange {
    pub id: String,
    pub status: ThreadStatus,
    pub created_by: UserSummary,
    pub created_at: OffsetDateTime,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize, Serializable)]
#[serde(rename_all = "camelCase")]
pub struct CommentDeleted {
    pub id: String,
    pub created_by: UserSummary,
    pub created_at: OffsetDateTime,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize, Serializable)]
#[serde(rename_all = "camelCase")]
pub struct Comment {
    pub id: String,
    pub created_by: UserSummary,
    pub content: String, // limit of 2048 characters
    pub formatting: Formatting,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize, Serializable)]
#[serde(rename_all = "camelCase")]
pub struct UserSummary {
    pub id: String,
    pub name: String,
}
