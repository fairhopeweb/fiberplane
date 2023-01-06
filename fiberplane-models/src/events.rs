use base64uuid::Base64Uuid;
#[cfg(feature = "fp-bindgen")]
use fp_bindgen::prelude::Serializable;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use time::OffsetDateTime;

#[derive(Clone, Deserialize, Eq, PartialEq, Serialize, Debug)]
#[cfg_attr(
    feature = "fp-bindgen",
    derive(Serializable),
    fp(
        rust_plugin_module = "fiberplane_models::events",
        rust_wasmer_runtime_module = "fiberplane_models::events"
    )
)]
#[serde(rename_all = "camelCase")]
pub struct Event {
    pub id: Base64Uuid,
    pub title: String,
    pub labels: HashMap<String, Option<String>>,
    #[serde(with = "time::serde::rfc3339")]
    pub occurrence_time: OffsetDateTime,

    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    pub updated_at: OffsetDateTime,
}

#[derive(Clone, Default, Deserialize, Eq, PartialEq, Serialize, Debug)]
#[cfg_attr(
    feature = "fp-bindgen",
    derive(Serializable),
    fp(
        rust_plugin_module = "fiberplane_models::events",
        rust_wasmer_runtime_module = "fiberplane_models::events"
    )
)]
#[serde(rename_all = "camelCase")]
pub struct NewEvent {
    pub title: String,
    #[serde(default)]
    pub labels: Option<HashMap<String, Option<String>>>,
    #[serde(default, with = "time::serde::rfc3339::option")]
    pub time: Option<OffsetDateTime>,
}
