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
pub struct LogRecordIndex {
    #[serde(rename = "linkIndex")]
    pub link_index: f32,
    #[serde(rename = "recordIndex")]
    pub record_index: f32,
}

impl LogRecordIndex {
    pub fn new(link_index: f32, record_index: f32) -> LogRecordIndex {
        LogRecordIndex {
            link_index,
            record_index,
        }
    }
}
