use crate::protocols::operations::Operation;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ClientRealtimeMessage {
    /// Authenticate this client
    Authenticate(AuthenticateMessage),

    /// Subscribe to changes from a specific Notebook.
    Subscribe(SubscribeMessage),

    /// Unsubscribe to changes from a specific Notebook.
    Unsubscribe(UnsubscribeMessage),

    /// Apply an operation to a specific Notebook.
    ApplyOperation(ApplyOperationMessage),

    /// Request a DebugResponse from the server.
    DebugRequest(DebugRequestMessage),
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ServerRealtimeMessage {
    /// Apply an operation to a specific Notebook.
    ApplyOperation(ApplyOperationMessage),

    /// An Ack message will be sent once an operation is received and processed.
    /// No Ack message will sent if the op_id of the original message was empty.
    /// Gets rejected if sent to the server.
    Ack(AckMessage),

    /// An Err message will be sent once an operation is received, but could not
    /// be processed. It includes the op_id if that was present. Gets rejected if
    /// sent to the server.
    Err(ErrMessage),

    /// Response from a DebugRequest. Contains some useful data regarding the
    /// connection. Gets rejected if sent to the server.
    DebugResponse(DebugResponseMessage),

    /// Reject an apply operation. This happens when a ApplyOperation is sent,
    /// but the notebook has already applied another operation.
    Reject(RejectMessage),
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AuthenticateMessage {
    /// Bearer token
    pub token: String,

    /// Operation ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub op_id: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
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

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UnsubscribeMessage {
    /// ID of the notebook
    pub notebook_id: String,

    /// Operation ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub op_id: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
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

#[derive(Debug, Deserialize, Serialize, Clone)]
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

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ErrMessage {
    /// Error message.
    pub error_message: String,

    /// Operation ID. Empty if the user has not provided a op_id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub op_id: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DebugRequestMessage {
    /// Operation ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub op_id: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
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

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RejectMessage {
    /// The current revision of the notebook.
    pub current_revision: u32,

    /// Operation ID. Empty if the user has not provided a op_id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub op_id: Option<String>,
}
