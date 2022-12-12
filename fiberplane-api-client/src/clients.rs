use crate::builder::ApiClientBuilder;
use anyhow::{Context as _, Result};
use reqwest::{header, Client, Method, RequestBuilder, Url};
use std::time::Duration;

pub fn default_config(
    timeout: Option<Duration>,
    user_agent: Option<&str>,
    default_headers: Option<header::HeaderMap>,
) -> Result<Client> {
    let mut headers = default_headers.unwrap_or_default();
    headers.insert(
        header::USER_AGENT,
        header::HeaderValue::from_str(user_agent.unwrap_or("Fiberplane Rust API client"))?,
    );

    Ok(Client::builder()
        .connect_timeout(timeout.unwrap_or_else(|| Duration::from_secs(10)))
        .default_headers(headers)
        .build()?)
}

pub fn production_client() -> Result<ApiClient> {
    let url = "https://studio.fiberplane.com/";

    let config = default_config(Some(Duration::from_secs(5)), None, None)?;

    Ok(ApiClient {
        client: config,
        server: Url::parse(url).context("Failed to parse base url from Open API document")?,
    })
}

pub fn non_production_client(env: Option<&str>) -> Result<ApiClient> {
    let env = env.unwrap_or("dev");
    let url = &format!("https://{env}.fiberplane.io/", env = env);

    let config = default_config(Some(Duration::from_secs(5)), None, None)?;

    Ok(ApiClient {
        client: config,
        server: Url::parse(url).context("Failed to parse base url from Open API document")?,
    })
}

#[derive(Debug)]
pub struct ApiClient {
    pub client: Client,
    pub server: Url,
}

impl ApiClient {
    pub fn request(&self, method: Method, endpoint: &str) -> Result<RequestBuilder> {
        let url = self.server.join(endpoint)?;

        Ok(self.client.request(method, url))
    }

    pub fn builder(base_url: Url) -> ApiClientBuilder {
        ApiClientBuilder::new(base_url)
    }
}
