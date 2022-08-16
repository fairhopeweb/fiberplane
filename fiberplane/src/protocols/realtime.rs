use crate::protocols::comments::{Thread, ThreadItem};
use crate::protocols::core::{LabelValidationError, UserType};
use crate::protocols::operations::Operation;
use fp_bindgen::prelude::*;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::fmt::Debug;
use time::OffsetDateTime;

/// Real-time message sent by the client over a WebSocket connection.
#[derive(Clone, Debug, Deserialize, Serialize, Serializable)]
#[fp(rust_plugin_module = "fiberplane::protocols::realtime")]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ClientRealtimeMessage {
    /// Authenticate this client
    Authenticate(AuthenticateMessage),

    /// Subscribe to changes from a specific Notebook.
    Subscribe(SubscribeMessage),

    /// Unsubscribe to changes from a specific Notebook.
    Unsubscribe(UnsubscribeMessage),

    /// Apply an operation to a specific Notebook.
    ApplyOperation(Box<ApplyOperationMessage>),

    /// Apply multiple operations to a specific Notebook.
    ApplyOperationBatch(Box<ApplyOperationBatchMessage>),

    /// Request a DebugResponse from the server.
    DebugRequest(DebugRequestMessage),

    FocusInfo(FocusInfoMessage),

    /// User started typing a comment.
    UserTypingComment(UserTypingCommentClientMessage),
}

impl ClientRealtimeMessage {
    pub fn op_id(&self) -> &Option<String> {
        use ClientRealtimeMessage::*;
        match self {
            Authenticate(msg) => &msg.op_id,
            Subscribe(msg) => &msg.op_id,
            Unsubscribe(msg) => &msg.op_id,
            ApplyOperation(msg) => &msg.op_id,
            ApplyOperationBatch(msg) => &msg.op_id,
            DebugRequest(msg) => &msg.op_id,
            FocusInfo(msg) => &msg.op_id,
            UserTypingComment(msg) => &msg.op_id,
        }
    }
}

/// Real-time message sent by the server over a WebSocket connection.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Serializable)]
#[fp(rust_plugin_module = "fiberplane::protocols::realtime")]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ServerRealtimeMessage {
    /// Apply an operation to a specific Notebook.
    ApplyOperation(Box<ApplyOperationMessage>),

    /// An Ack message will be sent once an operation is received and processed.
    /// No Ack message will sent if the op_id of the original message was empty.
    Ack(AckMessage),

    /// An Err message will be sent once an operation is received, but could not
    /// be processed. It includes the op_id if that was present.
    Err(ErrMessage),

    /// Response from a DebugRequest. Contains some useful data regarding the
    /// connection.
    DebugResponse(DebugResponseMessage),

    /// Notifies a mentioned user of the fact they've been mentioned by someone
    /// else.
    Mention(MentionMessage),

    /// An apply operation got rejected by the server, see message for the
    /// reason.
    Rejected(RejectedMessage),

    /// A user has joined as a subscriber to a notebook.
    SubscriberAdded(SubscriberAddedMessage),

    /// A previously subscribed user has left a notebook.
    SubscriberRemoved(SubscriberRemovedMessage),

    SubscriberChangedFocus(SubscriberChangedFocusMessage),

    /// A new comment thread was added to the notebook.
    ThreadAdded(ThreadAddedMessage),

    /// A new item was added to a comment thread (e.g. a comment or a thread status change).
    ThreadItemAdded(ThreadItemAddedMessage),

    /// A new item was added to a comment thread (e.g. a comment or a thread status change).
    ThreadItemUpdated(ThreadItemUpdatedMessage),

    /// A comment thread was deleted
    ThreadDeleted(ThreadDeletedMessage),

    /// A user started typing a comment
    UserTypingComment(UserTypingCommentServerMessage),
}

#[derive(Clone, Deserialize, PartialEq, Eq, Serialize, Serializable)]
#[fp(rust_plugin_module = "fiberplane::protocols::realtime")]
#[serde(rename_all = "camelCase")]
pub struct AuthenticateMessage {
    /// Bearer token
    pub token: String,

    /// Operation ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub op_id: Option<String>,
}

impl Debug for AuthenticateMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AuthenticateMessage")
            .field("token", &"[REDACTED]")
            .field("op_id", &self.op_id)
            .finish()
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize, Serializable)]
#[fp(rust_plugin_module = "fiberplane::protocols::realtime")]
#[serde(rename_all = "camelCase")]
pub struct SubscribeMessage {
    /// ID of the notebook
    pub notebook_id: String,

    /// The current revision that the client knows about. If this is not the
    /// current revision according to the server, than the server will sent
    /// all operations starting from this revision.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision: Option<u32>,

    /// Operation ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub op_id: Option<String>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize, Serializable)]
#[fp(rust_plugin_module = "fiberplane::protocols::realtime")]
#[serde(rename_all = "camelCase")]
pub struct UnsubscribeMessage {
    /// ID of the notebook
    pub notebook_id: String,

    /// Operation ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub op_id: Option<String>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Serializable)]
#[fp(rust_plugin_module = "fiberplane::protocols::realtime")]
#[serde(rename_all = "camelCase")]
pub struct ApplyOperationMessage {
    /// ID of the notebook
    pub notebook_id: String,

    /// Operation
    pub operation: Operation,

    /// Revision, for a client sending this message it means the desired new
    /// revision. When it is sent from a server it is the actual revision.
    pub revision: u32,

    /// Operation ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub op_id: Option<String>,
}

impl ApplyOperationMessage {
    pub fn new(
        notebook_id: String,
        operation: Operation,
        revision: u32,
        op_id: Option<String>,
    ) -> Self {
        Self {
            notebook_id,
            operation,
            revision,
            op_id,
        }
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Serializable)]
#[fp(rust_plugin_module = "fiberplane::protocols::realtime")]
#[serde(rename_all = "camelCase")]
pub struct ApplyOperationBatchMessage {
    /// ID of the notebook
    pub notebook_id: String,

    /// Operation
    pub operations: Vec<Operation>,

    /// Revision, this will be the revision of the first operation. The other
    /// operations will keep bumping the revision.
    pub revision: u32,

    /// Operation ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub op_id: Option<String>,
}

impl ApplyOperationBatchMessage {
    pub fn new(
        notebook_id: String,
        operations: Vec<Operation>,
        revision: u32,
        op_id: Option<String>,
    ) -> Self {
        Self {
            notebook_id,
            operations,
            revision,
            op_id,
        }
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize, Serializable)]
#[fp(rust_plugin_module = "fiberplane::protocols::realtime")]
#[serde(rename_all = "camelCase")]
pub struct AckMessage {
    /// Operation ID.
    pub op_id: String,
}

impl AckMessage {
    pub fn new(op_id: String) -> Self {
        Self { op_id }
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize, Serializable)]
#[fp(rust_plugin_module = "fiberplane::protocols::realtime")]
#[serde(rename_all = "camelCase")]
pub struct ErrMessage {
    /// Error message.
    pub error_message: String,

    /// Operation ID. Empty if the user has not provided a op_id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub op_id: Option<String>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize, Serializable)]
#[fp(rust_plugin_module = "fiberplane::protocols::realtime")]
#[serde(rename_all = "camelCase")]
pub struct DebugRequestMessage {
    /// Operation ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub op_id: Option<String>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize, Serializable)]
#[fp(rust_plugin_module = "fiberplane::protocols::realtime")]
#[serde(rename_all = "camelCase")]
pub struct DebugResponseMessage {
    /// Session ID.
    pub sid: String,

    /// Notebooks that the user is subscribed to.
    pub subscribed_notebooks: Vec<String>,

    /// Operation ID. Empty if the user has not provided a op_id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub op_id: Option<String>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize, Serializable)]
#[fp(rust_plugin_module = "fiberplane::protocols::realtime")]
#[serde(rename_all = "camelCase")]
pub struct MentionMessage {
    /// ID of the notebook in which the user was mentioned.
    pub notebook_id: String,

    /// ID of the cell in which the user was mentioned.
    pub cell_id: String,

    /// Who mentioned the user?
    pub mentioned_by: MentionedBy,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Serialize, Serializable)]
#[fp(rust_plugin_module = "fiberplane::protocols::realtime")]
#[serde(rename_all = "camelCase")]
pub struct MentionedBy {
    #[serde(rename = "type")]
    pub user_type: UserType,
    pub name: String,
}

/// Message sent when an apply operation was rejected by the server.
#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize, Serializable)]
#[fp(rust_plugin_module = "fiberplane::protocols::realtime")]
#[serde(rename_all = "camelCase")]
pub struct RejectedMessage {
    /// The reason why the apply operation was rejected.
    pub reason: Box<RejectReason>,

    /// Operation ID. Empty if the user has not provided a op_id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub op_id: Option<String>,
}

impl RejectedMessage {
    pub fn new(reason: RejectReason, op_id: Option<String>) -> Self {
        Self {
            reason: Box::new(reason),
            op_id,
        }
    }
}

/// Reason why the apply operation was rejected.
#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize, Serializable)]
#[fp(rust_plugin_module = "fiberplane::protocols::realtime")]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum RejectReason {
    /// The operation referenced an invalid cell index.
    CellIndexOutOfBounds,

    /// The operation referenced a non-existing cell.
    #[serde(rename_all = "camelCase")]
    CellNotFound { cell_id: String },

    /// The operation tried to insert a cell with a non-unique ID.
    #[serde(rename_all = "camelCase")]
    DuplicateCellId { cell_id: String },

    /// A label was submitted for already exists for the notebook.
    DuplicateLabel(DuplicateLabelRejectReason),

    /// The operation failed some miscellaneous precondition.
    #[serde(rename_all = "camelCase")]
    FailedPrecondition { message: String },

    /// A label was submitted that was invalid.
    InvalidLabel(InvalidLabelRejectReason),

    /// Current notebook state does not match old state in operation.
    InconsistentState,

    /// Attempted to perform a text operation on a non-text cell.
    #[serde(rename_all = "camelCase")]
    NoTextCell { cell_id: String },

    /// The requested apply operation was for an old version. The u32 contains
    /// the current revision.
    Outdated(OutdatedRejectReason),
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize, Serializable)]
#[fp(rust_plugin_module = "fiberplane::protocols::realtime")]
#[serde(rename_all = "camelCase")]
pub struct OutdatedRejectReason {
    /// The current revision for the notebook.
    pub current_revision: u32,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize, Serializable)]
#[fp(rust_plugin_module = "fiberplane::protocols::realtime")]
#[serde(rename_all = "camelCase")]
pub struct InvalidLabelRejectReason {
    /// The key of the label that was invalid.
    pub key: String,

    /// The specific reason why the label was invalid.
    pub validation_error: LabelValidationError,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize, Serializable)]
#[fp(rust_plugin_module = "fiberplane::protocols::realtime")]
#[serde(rename_all = "camelCase")]
pub struct DuplicateLabelRejectReason {
    /// The key of the label that was already present.
    pub key: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize, Serializable)]
#[fp(rust_plugin_module = "fiberplane::protocols::realtime")]
#[serde(rename_all = "camelCase")]
pub struct SubscriberAddedMessage {
    /// The ID of the notebook that the user subscribed to.
    pub notebook_id: String,

    /// ID associated with the newly connected session. There can be multiple
    /// sessions for a single (notebook|user) pair. The ID can be used multiple
    /// times for different (notebook|user) pairs. The combination of notebook,
    /// user and session will be unique.
    pub session_id: String,

    /// The moment the session was created.
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,

    /// The last time the user was active in this session.
    #[serde(with = "time::serde::rfc3339")]
    pub updated_at: OffsetDateTime,

    /// User details associated with the session.
    pub user: User,

    /// User's focus within the notebook.
    #[serde(default)]
    pub focus: NotebookFocus,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize, Serializable)]
#[fp(rust_plugin_module = "fiberplane::protocols::realtime")]
#[serde(rename_all = "camelCase")]
pub struct SubscriberRemovedMessage {
    /// The ID of the notebook that the user unsubscribed from.
    pub notebook_id: String,

    /// ID of the session that was removed.
    pub session_id: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize, Serializable)]
#[fp(rust_plugin_module = "fiberplane::protocols::realtime")]
#[serde(rename_all = "camelCase")]
pub struct User {
    /// The ID of the user. Will always be the same for the same user, so can be
    /// used for de-dupping or input for color generation.
    pub id: String,

    /// Name of the user
    pub name: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize, Serializable)]
#[fp(rust_plugin_module = "fiberplane::protocols::realtime")]
#[serde(rename_all = "camelCase")]
pub struct FocusInfoMessage {
    /// ID of the notebook.
    pub notebook_id: String,

    /// User's focus within the notebook.
    #[serde(default)]
    pub focus: NotebookFocus,

    /// Operation ID. Empty if the user has not provided a op_id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub op_id: Option<String>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize, Serializable)]
#[fp(rust_plugin_module = "fiberplane::protocols::realtime")]
#[serde(rename_all = "camelCase")]
pub struct UserTypingCommentClientMessage {
    pub notebook_id: String,
    pub thread_id: String,

    /// Operation ID. Empty if the user has not provided a op_id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub op_id: Option<String>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize, Serializable)]
#[fp(rust_plugin_module = "fiberplane::protocols::realtime")]
#[serde(rename_all = "camelCase")]
pub struct SubscriberChangedFocusMessage {
    /// ID of the session.
    pub session_id: String,

    /// ID of the notebook.
    pub notebook_id: String,

    /// User's focus within the notebook.
    #[serde(default)]
    pub focus: NotebookFocus,
}

/// A single focus position within a notebook.
///
/// Focus can be placed within a cell, and optionally within separate fields
/// within the cell. An offset can be specified to indicate the exact position
/// of the cursor within a text field.
#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize, Serializable)]
#[fp(rust_plugin_module = "fiberplane::protocols::realtime")]
#[serde(rename_all = "camelCase")]
pub struct FocusPosition {
    /// ID of the focused cell.
    ///
    /// May be the ID of an actual cell, or a so-called "surrogate ID", such as
    /// the ID that indicates focus is on the title field.
    pub cell_id: String,

    /// Key to identify which field inside a cell has focus.
    /// May be `None` for cells that have only one (or no) text field.
    /// E.g.: For time range cells, “to” or “from” could be used.
    ///
    /// Note that fields do not necessarily have to be text fields. For example,
    /// we could also use this to indicate the user has focused a button for
    /// graph navigation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field: Option<String>,

    /// Offset within the text field.
    /// May be `None` if the focus is not inside a text field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u32>,
}

/// Specifies the user's focus and optional selection within the notebook.
#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize, Serializable)]
#[fp(rust_plugin_module = "fiberplane::protocols::realtime")]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum NotebookFocus {
    /// The user has no focus within the notebook.
    None,
    /// The user focus is within the notebook and the focus is on a single
    /// position. I.e. there is no selection.
    Collapsed(FocusPosition),
    /// The user has a selection within the notebook that started at the given
    /// anchor position and ends at the given focus position.
    Selection {
        anchor: FocusPosition,
        focus: FocusPosition,
    },
}

impl NotebookFocus {
    pub fn anchor_cell_id(&self) -> Option<&str> {
        match self {
            Self::None => None,
            Self::Collapsed(collapsed) => Some(&collapsed.cell_id),
            Self::Selection { anchor, .. } => Some(&anchor.cell_id),
        }
    }

    pub fn anchor_cell_index(&self, cell_ids: &[&str]) -> Option<usize> {
        cell_ids
            .iter()
            .position(|cell_id| Some(*cell_id) == self.anchor_cell_id())
    }

    pub fn anchor_field(&self) -> Option<&str> {
        match self {
            Self::None => None,
            Self::Collapsed(collapsed) => collapsed.field.as_deref(),
            Self::Selection { anchor, .. } => anchor.field.as_deref(),
        }
    }

    pub fn anchor_offset(&self) -> u32 {
        match self {
            Self::None => 0,
            Self::Collapsed(position) => position.offset.unwrap_or_default(),
            Self::Selection { anchor, .. } => anchor.offset.unwrap_or_default(),
        }
    }

    pub fn anchor_position(&self) -> Option<&FocusPosition> {
        match self {
            Self::None => None,
            Self::Collapsed(position) => Some(position),
            Self::Selection { anchor, .. } => Some(anchor),
        }
    }

    pub fn end_cell_id(&self, cell_ids: &[&str]) -> Option<&str> {
        match self {
            Self::None => None,
            Self::Collapsed(position) => Some(&position.cell_id),
            Self::Selection { anchor, focus } => {
                let anchor_cell_index = self.anchor_cell_index(cell_ids).unwrap_or_default();
                let focus_cell_index = self.focus_cell_index(cell_ids).unwrap_or_default();
                if anchor_cell_index > focus_cell_index {
                    Some(&anchor.cell_id)
                } else {
                    Some(&focus.cell_id)
                }
            }
        }
    }

    pub fn end_offset(&self, cell_ids: &[&str]) -> u32 {
        match self {
            Self::None => 0,
            Self::Collapsed(position) => position.offset.unwrap_or_default(),
            Self::Selection { anchor, focus } => {
                let anchor_cell_index = self.anchor_cell_index(cell_ids).unwrap_or_default();
                let anchor_offset = anchor.offset.unwrap_or_default();
                let focus_cell_index = self.focus_cell_index(cell_ids).unwrap_or_default();
                let focus_offset = focus.offset.unwrap_or_default();
                match anchor_cell_index.cmp(&focus_cell_index) {
                    Ordering::Greater => anchor_offset,
                    Ordering::Equal => std::cmp::max(anchor_offset, focus_offset),
                    Ordering::Less => focus_offset,
                }
            }
        }
    }

    pub fn focus_cell_id(&self) -> Option<&str> {
        match self {
            Self::None => None,
            Self::Collapsed(collapsed) => Some(&collapsed.cell_id),
            Self::Selection { focus, .. } => Some(&focus.cell_id),
        }
    }

    pub fn focus_cell_index(&self, cell_ids: &[&str]) -> Option<usize> {
        cell_ids
            .iter()
            .position(|cell_id| Some(*cell_id) == self.focus_cell_id())
    }

    pub fn focus_field(&self) -> Option<&str> {
        match self {
            Self::None => None,
            Self::Collapsed(collapsed) => collapsed.field.as_deref(),
            Self::Selection { focus, .. } => focus.field.as_deref(),
        }
    }

    pub fn focus_offset(&self) -> u32 {
        match self {
            Self::None => 0,
            Self::Collapsed(position) => position.offset.unwrap_or_default(),
            Self::Selection { focus, .. } => focus.offset.unwrap_or_default(),
        }
    }

    pub fn focus_position(&self) -> Option<&FocusPosition> {
        match self {
            Self::None => None,
            Self::Collapsed(position) => Some(position),
            Self::Selection { focus, .. } => Some(focus),
        }
    }

    pub fn has_selection(&self) -> bool {
        !self.is_collapsed()
    }

    /// Returns whether the cursor position is collapsed, ie. the opposite of
    /// `has_selection()`.
    pub fn is_collapsed(&self) -> bool {
        match self {
            Self::None | Self::Collapsed(_) => true,
            Self::Selection { focus, anchor } => *focus == *anchor,
        }
    }

    pub fn is_none(&self) -> bool {
        matches!(self, Self::None)
    }

    pub fn start_cell_id(&self, cell_ids: &[&str]) -> Option<&str> {
        match self {
            Self::None => None,
            Self::Collapsed(position) => Some(&position.cell_id),
            Self::Selection { anchor, focus } => {
                if self.anchor_cell_index(cell_ids).unwrap_or_default()
                    < self.focus_cell_index(cell_ids).unwrap_or_default()
                {
                    Some(&anchor.cell_id)
                } else {
                    Some(&focus.cell_id)
                }
            }
        }
    }

    pub fn start_offset(&self, cell_ids: &[&str]) -> u32 {
        match self {
            Self::None => 0,
            Self::Collapsed(position) => position.offset.unwrap_or_default(),
            Self::Selection { anchor, focus } => {
                let anchor_cell_index = self.anchor_cell_index(cell_ids).unwrap_or_default();
                let anchor_offset = anchor.offset.unwrap_or_default();
                let focus_cell_index = self.focus_cell_index(cell_ids).unwrap_or_default();
                let focus_offset = focus.offset.unwrap_or_default();
                match anchor_cell_index.cmp(&focus_cell_index) {
                    Ordering::Less => anchor_offset,
                    Ordering::Equal => std::cmp::min(anchor_offset, focus_offset),
                    Ordering::Greater => focus_offset,
                }
            }
        }
    }
}

impl Default for NotebookFocus {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize, Serializable)]
#[fp(rust_plugin_module = "fiberplane::protocols::realtime")]
#[serde(rename_all = "camelCase")]
pub struct ThreadAddedMessage {
    pub notebook_id: String,
    pub thread: Thread,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize, Serializable)]
#[fp(rust_plugin_module = "fiberplane::protocols::realtime")]
#[serde(rename_all = "camelCase")]
pub struct ThreadItemAddedMessage {
    pub notebook_id: String,
    pub thread_id: String,
    pub thread_item: ThreadItem,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize, Serializable)]
#[fp(rust_plugin_module = "fiberplane::protocols::realtime")]
#[serde(rename_all = "camelCase")]
pub struct ThreadItemUpdatedMessage {
    pub notebook_id: String,
    pub thread_id: String,
    pub thread_item: ThreadItem,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize, Serializable)]
#[fp(rust_plugin_module = "fiberplane::protocols::realtime")]
#[serde(rename_all = "camelCase")]
pub struct ThreadDeletedMessage {
    pub notebook_id: String,
    pub thread_id: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize, Serializable)]
#[fp(rust_plugin_module = "fiberplane::protocols::realtime")]
#[serde(rename_all = "camelCase")]
pub struct UserTypingCommentServerMessage {
    pub notebook_id: String,
    pub thread_id: String,
    pub user_id: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_reject_reason() {
        let reason = OutdatedRejectReason {
            current_revision: 1,
        };
        let reason = RejectReason::Outdated(reason);
        let result = serde_json::to_string(&reason);
        if let Err(err) = result {
            panic!("Unexpected error occurred: {:?}", err);
        }
    }
}
