use crate::data_sources::SelectedDataSources;
pub use crate::labels::Label;
use crate::names::Name;
use crate::timestamps::Timestamp;
use base64uuid::Base64Uuid;
#[cfg(feature = "fp-bindgen")]
use fp_bindgen::prelude::Serializable;
use serde::{Deserialize, Serialize};
use strum_macros::Display;
use time::OffsetDateTime;

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

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize, Display)]
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
pub struct NewWorkspaceInvite {
    pub email: String,
    #[serde(default)]
    pub role: AuthRole,
}

impl NewWorkspaceInvite {
    pub fn new(email: impl Into<String>, role: AuthRole) -> Self {
        Self {
            email: email.into(),
            role,
        }
    }
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
pub struct WorkspaceInviteResponse {
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

impl NewWorkspace {
    pub fn new(name: Name) -> Self {
        Self {
            name,
            display_name: None,
            default_data_sources: None,
        }
    }
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
pub struct WorkspaceUserUpdate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<AuthRole>,
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq, Serialize, Display)]
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
    #[strum(serialize = "Viewer")]
    Read,
    #[strum(serialize = "Editor")]
    Write,
    Admin,
}

impl Default for AuthRole {
    fn default() -> Self {
        Self::Write
    }
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
pub struct WorkspaceInvite {
    pub id: Base64Uuid,
    pub sender: Base64Uuid,
    pub receiver: String,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    pub expires_at: OffsetDateTime,
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
pub struct Membership {
    pub id: Base64Uuid,
    pub email: String,
    pub name: String,
    pub role: AuthRole,
}
