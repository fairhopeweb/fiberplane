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
pub struct Point {
    #[serde(rename = "timestamp")]
    pub timestamp: f64,
    #[serde(rename = "value")]
    pub value: f64,
}

impl Point {
    pub fn new(timestamp: f64, value: f64) -> Point {
        Point {
            timestamp,
            value,
        }
    }
}


