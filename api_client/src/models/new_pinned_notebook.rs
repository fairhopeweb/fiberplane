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
pub struct NewPinnedNotebook {
    #[serde(rename = "notebookId")]
    pub notebook_id: String,
}

impl NewPinnedNotebook {
    pub fn new(notebook_id: String) -> NewPinnedNotebook {
        NewPinnedNotebook { notebook_id }
    }
}
