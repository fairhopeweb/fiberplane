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
pub struct EncodedBlob {
    #[serde(rename = "data")]
    pub data: String,
    #[serde(rename = "mimeType")]
    pub mime_type: String,
}

impl EncodedBlob {
    pub fn new(data: String, mime_type: String) -> EncodedBlob {
        EncodedBlob { data, mime_type }
    }
}
