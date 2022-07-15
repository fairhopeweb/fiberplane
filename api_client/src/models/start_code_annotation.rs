/*
 * Fiberplane API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StartCodeAnnotation {
    #[serde(rename = "type")]
    pub _type: crate::models::AnnotationType,
    #[serde(rename = "offset")]
    pub offset: i32,
}

impl StartCodeAnnotation {
    pub fn new(_type: crate::models::AnnotationType, offset: i32) -> StartCodeAnnotation {
        StartCodeAnnotation { _type, offset }
    }
}
