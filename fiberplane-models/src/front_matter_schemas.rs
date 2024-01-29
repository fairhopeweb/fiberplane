use crate::{notebooks::operations::FrontMatterSchemaRow, timestamps::Timestamp};
use base64uuid::Base64Uuid;
#[cfg(feature = "fp-bindgen")]
use fp_bindgen::prelude::Serializable;
use ordered_float::OrderedFloat;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use strum_macros::Display;
use typed_builder::TypedBuilder;

/// A floating-point number that can be ordered and compared using Eq.
///
/// It is not compliant to IEEE standard, and NaN is considered greater than
/// everything and equal to itself.
///
/// Also, this type is serializable using fp-bindgen, transparently to the underlying
/// f64 primitive.
#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize, Default)]
#[repr(transparent)]
pub struct SerializableEqFloat(OrderedFloat<f64>);

#[cfg(feature = "fp-bindgen")]
impl Serializable for SerializableEqFloat {
    fn ident() -> fp_bindgen::types::TypeIdent {
        fp_bindgen::types::TypeIdent::from("f64")
    }

    fn ty() -> fp_bindgen::types::Type {
        // fp_bindgen::types::Type::Primitive(fp_bindgen::primitives::Primitive::F64)

        fp_bindgen::types::Type::Custom(fp_bindgen::types::CustomType {
            ident: fp_bindgen::types::TypeIdent::from("SerializableEqFloat"),
            rs_ty: "f64".to_owned(),
            ts_ty: "number".to_owned(),
            // Not filling the BTreeMap here can be wrong, but as long as fiberplane_models ends up
            // in the dependencies of downstream users, it should be fine.
            rs_dependencies: std::collections::BTreeMap::new(),
            serde_attrs: Vec::new(),
            ts_declaration: None,
        })
    }
}

impl<T: Into<f64>> From<T> for SerializableEqFloat {
    fn from(value: T) -> Self {
        Self(OrderedFloat(value.into()))
    }
}

/// Front Matter Schema representation.
///
/// The order of the elements in the schema drives the order of
/// rendering elements.
#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize, Default)]
#[cfg_attr(
    feature = "fp-bindgen",
    derive(Serializable),
    fp(rust_module = "fiberplane_models::front_matter_schemas")
)]
#[repr(transparent)]
pub struct FrontMatterSchema(pub Vec<FrontMatterSchemaEntry>);

impl std::ops::Deref for FrontMatterSchema {
    type Target = Vec<FrontMatterSchemaEntry>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for FrontMatterSchema {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<Vec<FrontMatterSchemaEntry>> for FrontMatterSchema {
    fn from(value: Vec<FrontMatterSchemaEntry>) -> Self {
        Self(value)
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize, TypedBuilder)]
#[cfg_attr(
    feature = "fp-bindgen",
    derive(Serializable),
    fp(rust_module = "fiberplane_models::front_matter_schemas")
)]
#[non_exhaustive]
#[serde(rename_all = "camelCase")]
pub struct FrontMatterSchemaEntry {
    /// The key to use to target the front matter value in a notebook storage (Notebook::frontmatter).
    ///
    /// Currently, this key is also used to decide the "display" name of the front matter key
    #[builder(setter(into))]
    pub key: String,

    #[builder(setter(into))]
    pub schema: FrontMatterValueSchema,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize, Display)]
#[cfg_attr(
    feature = "fp-bindgen",
    derive(Serializable),
    fp(rust_module = "fiberplane_models::front_matter_schemas")
)]
#[non_exhaustive]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum FrontMatterValueSchema {
    Number(FrontMatterNumberSchema),
    String(FrontMatterStringSchema),
    DateTime(FrontMatterDateTimeSchema),
    User(FrontMatterUserSchema),
}

impl From<FrontMatterUserSchema> for FrontMatterValueSchema {
    fn from(v: FrontMatterUserSchema) -> Self {
        Self::User(v)
    }
}

impl From<FrontMatterDateTimeSchema> for FrontMatterValueSchema {
    fn from(v: FrontMatterDateTimeSchema) -> Self {
        Self::DateTime(v)
    }
}

impl From<FrontMatterStringSchema> for FrontMatterValueSchema {
    fn from(v: FrontMatterStringSchema) -> Self {
        Self::String(v)
    }
}

impl From<FrontMatterNumberSchema> for FrontMatterValueSchema {
    fn from(v: FrontMatterNumberSchema) -> Self {
        Self::Number(v)
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize, TypedBuilder)]
#[cfg_attr(
    feature = "fp-bindgen",
    derive(Serializable),
    fp(rust_module = "fiberplane_models::front_matter_schemas")
)]
#[non_exhaustive]
#[serde(rename_all = "camelCase")]
pub struct FrontMatterNumberSchema {
    #[builder(default, setter(into))]
    pub display_name: String,

    #[builder(default, setter(into, strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_name: Option<String>,

    #[builder(setter(strip_bool))]
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub allow_extra_values: bool,

    /// The list of valid "pre-filled" options one can choose for the field.
    ///
    /// There is a functional difference between `None` and `Some(Vec::new())`:
    /// - When `options.is_none()`, that means the current number field should
    ///   not propose pre-filled values at all: this front matter field is a
    ///   freeform field
    /// - When `options == Some(Vec::new())` (arguably with `allow_extra_values` being true),
    ///   that means that the field is supposed to be a "choose value from an enumerated list"-kind
    ///   of field, but without any pre-existing values being present.
    ///
    /// The difference of intent between those two cases can be used on the front-end side to decide
    /// how to render the front matter cell
    #[builder(default, setter(into, strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<FrontMatterEnumNumberValue>>,

    #[builder(default, setter(into, strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_value: Option<FrontMatterEnumNumberValue>,

    #[builder(default, setter(into, strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,

    #[builder(default, setter(into, strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suffix: Option<String>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize, TypedBuilder)]
#[cfg_attr(
    feature = "fp-bindgen",
    derive(Serializable),
    fp(rust_module = "fiberplane_models::front_matter_schemas")
)]
#[non_exhaustive]
#[serde(rename_all = "camelCase")]
pub struct FrontMatterStringSchema {
    #[builder(default, setter(into))]
    pub display_name: String,

    #[builder(default, setter(into, strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_name: Option<String>,

    /// Whether the field can have multiple values
    // Skip serialization if the bool is false, and defaults to false, and the setter in typed_builder will set the field to true.
    #[builder(setter(strip_bool))]
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub multiple: bool,

    #[builder(setter(strip_bool))]
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub allow_extra_values: bool,

    /// The list of valid "pre-filled" options one can choose for the field.
    ///
    /// There is a functional difference between `None` and `Some(Vec::new())`:
    /// - When `options.is_none()`, that means the current number field should
    ///   not propose pre-filled values at all: this front matter field is a
    ///   freeform field
    /// - When `options == Some(Vec::new())` (arguably with `allow_extra_values` being true),
    ///   that means that the field is supposed to be a "choose value from an enumerated list"-kind
    ///   of field, but without any pre-existing values being present.
    ///
    /// The difference of intent between those two cases can be used on the front-end side to decide
    /// how to render the front matter cell
    #[builder(default, setter(into, strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<FrontMatterEnumStringValue>>,

    #[builder(default, setter(into, strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_value: Option<FrontMatterEnumStringValue>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize, TypedBuilder)]
#[cfg_attr(
    feature = "fp-bindgen",
    derive(Serializable),
    fp(rust_module = "fiberplane_models::front_matter_schemas")
)]
#[non_exhaustive]
#[serde(rename_all = "camelCase")]
pub struct FrontMatterDateTimeSchema {
    #[builder(default, setter(into))]
    pub display_name: String,

    #[builder(default, setter(into, strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_name: Option<String>,

    #[builder(default, setter(into, strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_value: Option<FrontMatterEnumDateTimeValue>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize, TypedBuilder)]
#[cfg_attr(
    feature = "fp-bindgen",
    derive(Serializable),
    fp(rust_module = "fiberplane_models::front_matter_schemas")
)]
#[non_exhaustive]
#[serde(rename_all = "camelCase")]
pub struct FrontMatterUserSchema {
    #[builder(default, setter(into))]
    pub display_name: String,

    #[builder(default, setter(into, strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_name: Option<String>,

    #[builder(default, setter(into, strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_value: Option<FrontMatterEnumBase64UuidValue>,
}

// NOTE: The cleaner way would be to have a generic type FrontMatterEnumValue<T>,
// but it's impossible to _conditionnally_ add the `Serializable` trait bound on
// the inner type T only when there is the "fp-bindgen" feature.

// NOTE: The reason those are struct instead of "just" being the
// inner value is because we are already thinking of adding extra properties (like "color")
// to the known options of an enumeration

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize, TypedBuilder)]
#[cfg_attr(
    feature = "fp-bindgen",
    derive(Serializable),
    fp(rust_module = "fiberplane_models::front_matter_schemas")
)]
#[non_exhaustive]
#[serde(rename_all = "camelCase")]
pub struct FrontMatterEnumBase64UuidValue {
    #[builder(setter(into))]
    value: Base64Uuid,
}

impl From<Base64Uuid> for FrontMatterEnumBase64UuidValue {
    fn from(value: Base64Uuid) -> Self {
        Self { value }
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize, TypedBuilder)]
#[cfg_attr(
    feature = "fp-bindgen",
    derive(Serializable),
    fp(rust_module = "fiberplane_models::front_matter_schemas")
)]
#[non_exhaustive]
#[serde(rename_all = "camelCase")]
pub struct FrontMatterEnumStringValue {
    #[builder(setter(into))]
    value: String,
}

impl<T: Into<String>> From<T> for FrontMatterEnumStringValue {
    fn from(value: T) -> Self {
        Self {
            value: value.into(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize, TypedBuilder)]
#[cfg_attr(
    feature = "fp-bindgen",
    derive(Serializable),
    fp(rust_module = "fiberplane_models::front_matter_schemas")
)]
#[non_exhaustive]
#[serde(rename_all = "camelCase")]
pub struct FrontMatterEnumNumberValue {
    #[builder(setter(into))]
    value: SerializableEqFloat,
}

impl<T: Into<f64>> From<T> for FrontMatterEnumNumberValue {
    fn from(value: T) -> Self {
        Self {
            value: SerializableEqFloat(OrderedFloat(value.into())),
        }
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize, TypedBuilder)]
#[cfg_attr(
    feature = "fp-bindgen",
    derive(Serializable),
    fp(rust_module = "fiberplane_models::front_matter_schemas")
)]
#[non_exhaustive]
#[serde(rename_all = "camelCase")]
pub struct FrontMatterEnumDateTimeValue {
    #[builder(setter(into))]
    value: Timestamp,
}

impl<T: Into<Timestamp>> From<T> for FrontMatterEnumDateTimeValue {
    fn from(value: T) -> Self {
        Self {
            value: value.into(),
        }
    }
}

/// API Payload to update an entry to a front matter schema.
///
/// The payload is received in the context of a known (Notebook,
/// target key) pair, and
/// maps easily to an [`UpdateFrontMatterSchemaOperation`](crate::notebooks::operations::UpdateFrontMatterSchemaOperation)
///
/// Notably, as the _API_ will handle the call, it can fill the ceremonial data
/// related to Operational Transform, such as getting the "old" state and "old" value
/// that are necessary to build a valid `Operation`.
#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize, TypedBuilder)]
#[cfg_attr(
    feature = "fp-bindgen",
    derive(Serializable),
    fp(rust_module = "fiberplane_models::front_matter_schemas")
)]
#[non_exhaustive]
#[serde(rename_all = "camelCase")]
pub struct FrontMatterUpdateRow {
    /// The new schema to use, if unspecified the operation will leave the schema
    /// untouched (so the operation is only being used to edit the associated value).
    ///
    /// If a new schema is specified, and the data type does _not_ match between the
    /// old and the new one, then the old value will be wiped anyway.
    #[builder(default, setter(into, strip_option))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub new_schema: Option<FrontMatterValueSchema>,

    /// The new value to set for the front matter entry.
    ///
    /// If this attribute is `None` or `null` it can mean multiple things depending on
    /// the other attributes:
    /// - if `delete_value` is `false`, this means we want to keep the current value
    ///   + it is impossible to keep the current if the schemas are incompatible. In that
    ///     case we use the `default_value` of the new schema (or nothing if there’s no default)
    /// - if `delete_value` is `true`, this means we want to wipe the value from the front
    ///   matter in all cases.
    #[builder(default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub new_value: Option<Value>,

    /// Switch that controls front matter value edition alongside `new_value`, when
    /// `new_value` is None.
    #[builder(setter(strip_bool))]
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub delete_value: bool,
}

/// API Payload to add an entry to a front matter schema.
///
/// The payload is received in the context of a known Notebook, and
/// maps easily to an [`InsertFrontMatterOperation`](crate::notebooks::operations::InsertFrontMatterOperation)
///
/// Notably, as the _API_ will handle the call, it can fill the ceremonial data
/// related to Operational Transform, such as getting the "old" state and "old" value
/// that are necessary to build a valid `Operation`.
#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize, TypedBuilder)]
#[cfg_attr(
    feature = "fp-bindgen",
    derive(Serializable),
    fp(rust_module = "fiberplane_models::front_matter_schemas")
)]
#[non_exhaustive]
#[serde(rename_all = "camelCase")]
pub struct FrontMatterAddRows {
    /// The index to insert the new front matter schema into.
    ///
    /// If the index is
    pub to_index: u32,

    /// The new entries to add to the front matter schema, with their new values
    pub insertions: Vec<FrontMatterSchemaRow>,
}
