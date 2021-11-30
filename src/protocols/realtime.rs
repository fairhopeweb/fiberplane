use crate::protocols::operations::Operation;
use fp_bindgen::prelude::*;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

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

#[derive(Clone, Debug, Deserialize, Serialize, Serializable)]
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

    /// Reject an apply operation. This happens when a ApplyOperation is sent,
    /// but the notebook has already applied another operation.
    Reject(RejectMessage),

    /// A user has joined as a subscriber to a notebook.
    SubscriberAdded(SubscriberAddedMessage),

    /// A previously subscribed user has left a notebook.
    SubscriberRemoved(SubscriberRemovedMessage),

    SubscriberChangedFocus(SubscriberChangedFocusMessage),
}

#[derive(Clone, Debug, Deserialize, Serialize, Serializable)]
#[fp(rust_plugin_module = "fiberplane::protocols::realtime")]
#[serde(rename_all = "camelCase")]
pub struct AuthenticateMessage {
    /// Bearer token
    pub token: String,

    /// Operation ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub op_id: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize, Serializable)]
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

#[derive(Clone, Debug, Deserialize, Serialize, Serializable)]
#[fp(rust_plugin_module = "fiberplane::protocols::realtime")]
#[serde(rename_all = "camelCase")]
pub struct UnsubscribeMessage {
    /// ID of the notebook
    pub notebook_id: String,

    /// Operation ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub op_id: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize, Serializable)]
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

#[derive(Clone, Debug, Deserialize, Serialize, Serializable)]
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

#[derive(Clone, Debug, Deserialize, Serialize, Serializable)]
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

#[derive(Clone, Debug, Deserialize, Serialize, Serializable)]
#[fp(rust_plugin_module = "fiberplane::protocols::realtime")]
#[serde(rename_all = "camelCase")]
pub struct ErrMessage {
    /// Error message.
    pub error_message: String,

    /// Operation ID. Empty if the user has not provided a op_id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub op_id: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize, Serializable)]
#[fp(rust_plugin_module = "fiberplane::protocols::realtime")]
#[serde(rename_all = "camelCase")]
pub struct DebugRequestMessage {
    /// Operation ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub op_id: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize, Serializable)]
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

#[derive(Clone, Debug, Deserialize, Serialize, Serializable)]
#[fp(rust_plugin_module = "fiberplane::protocols::realtime")]
#[serde(rename_all = "camelCase")]
pub struct RejectMessage {
    /// The current revision of the notebook.
    pub current_revision: u32,

    /// Operation ID. Empty if the user has not provided a op_id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub op_id: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize, Serializable)]
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
    #[serde(with = "crate::serde_rfc3339")]
    pub created_at: OffsetDateTime,

    /// The last time the user was active in this session.
    #[serde(with = "crate::serde_rfc3339")]
    pub updated_at: OffsetDateTime,

    /// User details associated with the session.
    pub user: User,

    /// ID of the focused cell. Empty if no cell is focused.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub focused_cell_id: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize, Serializable)]
#[fp(rust_plugin_module = "fiberplane::protocols::realtime")]
#[serde(rename_all = "camelCase")]
pub struct SubscriberRemovedMessage {
    /// The ID of the notebook that the user unsubscribed from.
    pub notebook_id: String,

    /// ID of the session that was removed.
    pub session_id: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, Serializable)]
#[fp(rust_plugin_module = "fiberplane::protocols::realtime")]
#[serde(rename_all = "camelCase")]
pub struct User {
    /// The ID of the user. Will always be the same for the same user, so can be
    /// used for de-dupping or input for color generation.
    pub id: String,

    /// Name of the user
    pub name: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, Serializable)]
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

#[derive(Clone, Debug, Deserialize, Serialize, Serializable)]
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
