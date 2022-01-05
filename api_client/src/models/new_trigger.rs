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
pub struct NewTrigger {
    #[serde(rename = "templateUrl", skip_serializing_if = "Option::is_none")]
    pub template_url: Option<String>,
    #[serde(rename = "templateBody", skip_serializing_if = "Option::is_none")]
    pub template_body: Option<String>,
}

impl NewTrigger {
    pub fn new() -> NewTrigger {
        NewTrigger {
            template_url: None,
            template_body: None,
        }
    }
}


