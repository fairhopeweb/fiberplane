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
pub struct Token {
    #[serde(rename = "token")]
    pub token: String,
}

impl Token {
    pub fn new(token: String) -> Token {
        Token { token }
    }
}
