use crate::protocols::core::{LabelValidationError, UserType};
use crate::protocols::operations::Operation;
use fp_bindgen::prelude::*;
use serde::{Deserialize, Serialize};
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
}

#[derive(Clone, Deserialize, PartialEq, Serialize, Serializable)]
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

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Serializable)]
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

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Serializable)]
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

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Serializable)]
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

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Serializable)]
#[fp(rust_plugin_module = "fiberplane::protocols::realtime")]
#[serde(rename_all = "camelCase")]
pub struct ErrMessage {
    /// Error message.
    pub error_message: String,

    /// Operation ID. Empty if the user has not provided a op_id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub op_id: Option<String>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Serializable)]
#[fp(rust_plugin_module = "fiberplane::protocols::realtime")]
#[serde(rename_all = "camelCase")]
pub struct DebugRequestMessage {
    /// Operation ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub op_id: Option<String>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Serializable)]
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

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Serializable)]
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

#[derive(Debug, Clone, Deserialize, PartialEq, Serialize, Serializable)]
#[fp(rust_plugin_module = "fiberplane::protocols::realtime")]
#[serde(rename_all = "camelCase")]
pub struct MentionedBy {
    #[serde(rename = "type")]
    pub user_type: UserType,
    pub name: String,
}

/// Message sent when an apply operation was rejected by the server.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Serializable)]
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
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Serializable)]
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

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Serializable)]
#[fp(rust_plugin_module = "fiberplane::protocols::realtime")]
#[serde(rename_all = "camelCase")]
pub struct OutdatedRejectReason {
    /// The current revision for the notebook.
    pub current_revision: u32,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Serializable)]
#[fp(rust_plugin_module = "fiberplane::protocols::realtime")]
#[serde(rename_all = "camelCase")]
pub struct InvalidLabelRejectReason {
    /// The key of the label that was invalid.
    pub key: String,

    /// The specific reason why the label was invalid.
    pub validation_error: LabelValidationError,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Serializable)]
#[fp(rust_plugin_module = "fiberplane::protocols::realtime")]
#[serde(rename_all = "camelCase")]
pub struct DuplicateLabelRejectReason {
    /// The key of the label that was already present.
    pub key: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Serializable)]
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
    #[serde(with = "time_util::serde_rfc3339")]
    pub created_at: OffsetDateTime,

    /// The last time the user was active in this session.
    #[serde(with = "time_util::serde_rfc3339")]
    pub updated_at: OffsetDateTime,

    /// User details associated with the session.
    pub user: User,

    /// ID of the focused cell. Empty if no cell is focused.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub focused_cell_id: Option<String>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Serializable)]
#[fp(rust_plugin_module = "fiberplane::protocols::realtime")]
#[serde(rename_all = "camelCase")]
pub struct SubscriberRemovedMessage {
    /// The ID of the notebook that the user unsubscribed from.
    pub notebook_id: String,

    /// ID of the session that was removed.
    pub session_id: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Serializable)]
#[fp(rust_plugin_module = "fiberplane::protocols::realtime")]
#[serde(rename_all = "camelCase")]
pub struct User {
    /// The ID of the user. Will always be the same for the same user, so can be
    /// used for de-dupping or input for color generation.
    pub id: String,

    /// Name of the user
    pub name: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Serializable)]
#[fp(rust_plugin_module = "fiberplane::protocols::realtime")]
#[serde(rename_all = "camelCase")]
pub struct FocusInfoMessage {
    /// ID of the notebook.
    pub notebook_id: String,

    /// ID of the focused cell. Empty if no cell is focused.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cell_id: Option<String>,

    /// Operation ID. Empty if the user has not provided a op_id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub op_id: Option<String>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Serializable)]
#[fp(rust_plugin_module = "fiberplane::protocols::realtime")]
#[serde(rename_all = "camelCase")]
pub struct SubscriberChangedFocusMessage {
    /// ID of the session.
    pub session_id: String,

    /// ID of the notebook.
    pub notebook_id: String,

    /// ID of the focused cell. Empty if no cell is focused.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cell_id: Option<String>,
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
