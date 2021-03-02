use crate::protocols::operations::Operation;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ClientRealtimeMessage {
    /// Subscribe to changes from a specific Notebook.
    Subscribe(SubscribeMessage),

    /// Unsubscribe to changes from a specific Notebook.
    Unsubscribe(UnsubscribeMessage),

    /// Apply an operation to a specific Notebook.
    ApplyOperation(ApplyOperationMessage),

    /// Request a DebugResponse from the server.
    DebugRequest(DebugRequestMessage),
}

#[derive(Deserialize, Serialize, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ServerRealtimeMessage {
    /// Apply an operation to a specific Notebook.
    ApplyOperation(ApplyOperationMessage),

    /// An Ack message will be sent once an operation is received and processed.
    /// No Ack message will sent if the opid of the original message was empty.
    /// Gets rejected if sent to the server.
    Ack(AckMessage),

    /// An Err message will be sent once an operation is received, but could not
    /// be processed. It includes the opid if that was present. Gets rejected if
    /// sent to the server.
    Err(ErrMessage),

    /// Response from a DebugRequest. Contains some useful data regarding the
    /// connection. Gets rejected if sent to the server.
    DebugResponse(DebugResponseMessage),
}

#[derive(Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SubscribeMessage {
    /// ID of the notebook
    pub notebook_id: String,

    /// Operation ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opid: Option<String>,
}

#[derive(Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UnsubscribeMessage {
    /// ID of the notebook
    pub notebook_id: String,

    /// Operation ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opid: Option<String>,
}

#[derive(Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ApplyOperationMessage {
    /// ID of the notebook
    pub notebook_id: String,

    /// Operation
    pub operation: Operation,

    /// Operation ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opid: Option<String>,
}

impl ApplyOperationMessage {
    pub fn new(notebook_id: String, operation: Operation, opid: Option<String>) -> Self {
        Self {
            notebook_id,
            operation,
            opid,
        }
    }
}

#[derive(Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AckMessage {
    /// Operation ID.
    pub opid: String,
}

#[derive(Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ErrMessage {
    /// Error message.
    pub error_message: String,

    /// Operation ID. Empty if the user has not provided a opid.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opid: Option<String>,
}

#[derive(Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DebugRequestMessage {
    /// Operation ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opid: Option<String>,
}

#[derive(Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DebugResponseMessage {
    /// Session ID.
    pub sid: String,

    /// Notebooks that the user is subscribed to.
    pub subscribed_notebooks: Vec<String>,

    /// Operation ID. Empty if the user has not provided a opid.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opid: Option<String>,
}
