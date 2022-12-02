use crate::data_sources::SelectedDataSources;
pub use crate::labels::Label;
use crate::names::Name;
use crate::timestamps::Timestamp;
use base64uuid::Base64Uuid;
#[cfg(feature = "fp-bindgen")]
use fp_bindgen::prelude::Serializable;
use serde::{Deserialize, Serialize};

/// Workspace representation.
#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[cfg_attr(
    feature = "fp-bindgen",
    derive(Serializable),
    fp(
        rust_plugin_module = "fiberplane_models::workspaces",
        rust_wasmer_runtime_module = "fiberplane_models::workspaces"
    )
)]
#[serde(rename_all = "camelCase")]
pub struct Workspace {
    pub id: Base64Uuid,
    pub name: Name,
    pub display_name: String,
    #[serde(rename = "type")]
    pub ty: WorkspaceType,
    pub owner_id: Base64Uuid,
    pub default_data_sources: SelectedDataSources,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[cfg_attr(
    feature = "fp-bindgen",
    derive(Serializable),
    fp(
        rust_plugin_module = "fiberplane_models::workspaces",
        rust_wasmer_runtime_module = "fiberplane_models::workspaces"
    )
)]
#[serde(rename_all = "snake_case")]
pub enum WorkspaceType {
    Personal,
    Organization,
}

/// Payload to be able to invite someone to a workspace.
#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[cfg_attr(
    feature = "fp-bindgen",
    derive(Serializable),
    fp(
        rust_plugin_module = "fiberplane_models::workspaces",
        rust_wasmer_runtime_module = "fiberplane_models::workspaces"
    )
)]
#[serde(rename_all = "camelCase")]
pub struct NewWorkspaceInvitation {
    pub email: String,
    #[serde(default)]
    pub role: AuthRole,
}

/// Response received from create a new workspace endpoint.
#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[cfg_attr(
    feature = "fp-bindgen",
    derive(Serializable),
    fp(
        rust_plugin_module = "fiberplane_models::workspaces",
        rust_wasmer_runtime_module = "fiberplane_models::workspaces"
    )
)]
#[serde(rename_all = "camelCase")]
pub struct NewWorkspaceInvitationResponse {
    pub url: String,
}

/// Payload to create a new organization workspace.
#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[cfg_attr(
    feature = "fp-bindgen",
    derive(Serializable),
    fp(
        rust_plugin_module = "fiberplane_models::workspaces",
        rust_wasmer_runtime_module = "fiberplane_models::workspaces"
    )
)]
#[serde(rename_all = "camelCase")]
pub struct NewWorkspace {
    pub name: Name,
    /// The display name of the workspace. The `name` will be used if none is provided
    pub display_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_data_sources: Option<SelectedDataSources>,
}

/// Payload to update workspace settings
#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[cfg_attr(
    feature = "fp-bindgen",
    derive(Serializable),
    fp(
        rust_plugin_module = "fiberplane_models::workspaces",
        rust_wasmer_runtime_module = "fiberplane_models::workspaces"
    )
)]
#[serde(rename_all = "camelCase")]
pub struct UpdateWorkspace {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<Base64Uuid>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_data_sources: Option<SelectedDataSources>,
}

/// Payload to update a workspace members' role
#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[cfg_attr(
    feature = "fp-bindgen",
    derive(Serializable),
    fp(
        rust_plugin_module = "fiberplane_models::workspaces",
        rust_wasmer_runtime_module = "fiberplane_models::workspaces"
    )
)]
#[serde(rename_all = "snake_case")]
pub struct UpdateWorkspaceMember {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<AuthRole>,
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
#[cfg_attr(
    feature = "fp-bindgen",
    derive(Serializable),
    fp(
        rust_plugin_module = "fiberplane_models::workspaces",
        rust_wasmer_runtime_module = "fiberplane_models::workspaces"
    )
)]
#[serde(rename_all = "snake_case")]
pub enum AuthRole {
    Read,
    Write,
    Admin,
}

impl Default for AuthRole {
    fn default() -> Self {
        Self::Write
    }
}
