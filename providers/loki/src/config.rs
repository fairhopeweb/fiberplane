use serde::Deserialize;
use std::collections::HashMap;
use url::Url;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub url: Url,
    #[serde(flatten)]
    pub auth: Option<Auth>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Auth {
    Basic { username: String, password: String },
    Bearer { token: String },
}

impl Auth {
    pub fn to_headers(&self) -> HashMap<String, String> {
        match self {
            Auth::Basic { username, password } => HashMap::from([(
                "Authorization".to_string(),
                format!(
                    "Basic {}",
                    base64::encode(format!("{}:{}", username, password))
                ),
            )]),
            Auth::Bearer { token } => {
                HashMap::from([("Authorization".to_string(), format!("Bearer {}", token))])
            }
        }
    }
}
