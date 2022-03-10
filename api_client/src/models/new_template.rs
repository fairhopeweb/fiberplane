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
pub struct NewTemplate {
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "body")]
    pub body: String,
}

impl NewTemplate {
    pub fn new(title: String, description: String, body: String) -> NewTemplate {
        NewTemplate {
            title,
            description,
            body,
        }
    }
}


