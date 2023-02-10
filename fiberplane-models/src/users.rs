use crate::names::Name;
use crate::workspaces::AuthRole;
use base64uuid::Base64Uuid;
#[cfg(feature = "fp-bindgen")]
use fp_bindgen::prelude::Serializable;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use typed_builder::TypedBuilder;

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize, TypedBuilder)]
#[cfg_attr(
    feature = "fp-bindgen",
    derive(Serializable),
    fp(rust_module = "fiberplane_models::users")
)]
#[non_exhaustive]
#[serde(rename_all = "camelCase")]
pub struct Profile {
    pub id: Base64Uuid,
    pub email: String,
    pub name: String,
    pub default_workspace_id: Base64Uuid,
    pub default_workspace_name: Name,
    pub roles: HashMap<Base64Uuid, AuthRole>,
}

impl Profile {
    pub fn new(
        id: Base64Uuid,
        email: String,
        name: String,
        default_workspace_id: Base64Uuid,
        default_workspace_name: Name,
        roles: Option<HashMap<Base64Uuid, AuthRole>>,
    ) -> Self {
        let roles = roles.unwrap_or_default();
        Self {
            id,
            email,
            name,
            default_workspace_id,
            default_workspace_name,
            roles,
        }
    }
}
