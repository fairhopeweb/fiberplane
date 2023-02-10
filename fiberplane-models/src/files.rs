use base64uuid::Base64Uuid;
#[cfg(feature = "fp-bindgen")]
use fp_bindgen::prelude::Serializable;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(Clone, Default, Deserialize, Eq, PartialEq, Serialize, TypedBuilder)]
#[cfg_attr(
    feature = "fp-bindgen",
    derive(Serializable),
    fp(rust_module = "fiberplane_models::files")
)]
#[non_exhaustive]
#[serde(rename_all = "camelCase")]
pub struct UploadData {
    pub file: Vec<u8>,
}

#[derive(Clone, Default, Deserialize, Eq, PartialEq, Serialize, TypedBuilder)]
#[cfg_attr(
    feature = "fp-bindgen",
    derive(Serializable),
    fp(rust_module = "fiberplane_models::files")
)]
#[non_exhaustive]
#[serde(rename_all = "camelCase")]
pub struct ProfileUploadData {
    pub picture: Vec<u8>,
}

#[derive(Clone, Default, Deserialize, Eq, PartialEq, Serialize, TypedBuilder)]
#[cfg_attr(
    feature = "fp-bindgen",
    derive(Serializable),
    fp(rust_module = "fiberplane_models::files")
)]
#[non_exhaustive]
#[serde(rename_all = "camelCase")]
pub struct FileSummary {
    pub file_id: Base64Uuid,
}
