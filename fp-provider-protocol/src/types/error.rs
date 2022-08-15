use super::HttpRequestError;
use fp_bindgen::prelude::Serializable;

#[derive(Debug, Serializable)]
#[fp(tag = "type", rename_all = "snake_case")]
#[allow(dead_code)]
pub enum Error {
    UnsupportedRequest,
    ValidationError {
        /// List of errors, so all fields that failed validation can
        /// be highlighted at once.
        errors: Vec<ValidationError>,
    },
    #[fp(rename_all = "camelCase")]
    Http {
        error: HttpRequestError,
    },
    #[fp(rename_all = "camelCase")]
    Data {
        message: String,
    },
    #[fp(rename_all = "camelCase")]
    Deserialization {
        message: String,
    },
    #[fp(rename_all = "camelCase")]
    Config {
        message: String,
    },
    #[fp(rename_all = "camelCase")]
    Other {
        message: String,
    },
}

#[derive(Debug, Serializable)]
#[fp(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct ValidationError {
    /// Refers to a field from the query schema.
    field_name: String,
    /// Description of why the validation failed.
    message: String,
}
