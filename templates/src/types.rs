use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TemplateParameterType {
    String,
    Number,
    Boolean,
    // Note: we may add the nested types in the future.
    // If we do, we should add it in a non-breaking way
    // so that serialized objects created with this schema
    // can still be deserialized.
    Object,
    Array,
    /// We can only extract the parameter type from function parameters
    /// that have default values
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TemplateParameter {
    pub name: String,
    #[serde(rename = "type")]
    pub ty: TemplateParameterType,
    pub default_value: Option<Value>,
}
