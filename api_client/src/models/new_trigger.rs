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
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "templateId")]
    pub template_id: String,
    #[serde(rename = "defaultArguments", skip_serializing_if = "Option::is_none")]
    pub default_arguments: Option<serde_json::Value>,
}

impl NewTrigger {
    pub fn new(title: String, template_id: String) -> NewTrigger {
        NewTrigger {
            title,
            template_id,
            default_arguments: None,
        }
    }
}
