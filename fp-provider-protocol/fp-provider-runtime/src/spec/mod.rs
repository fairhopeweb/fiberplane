use rand::Rng;
use reqwest::Url;
use std::collections::HashMap;
use std::time::SystemTime;
use tracing::{debug, error, instrument, trace};

mod bindings;
pub mod types;

pub use bindings::*;
use types::*;

const MAX_HTTP_RESPONSE_SIZE: usize = 1024 * 1024 * 2; //2MiB

#[instrument(skip_all, fields(
    url = ?req.url,
    method = ?req.method,
    headers = ?req.headers.as_ref().map(|headers| headers.keys().map(|key| key.as_str()).collect::<Vec<_>>()).unwrap_or_default(),
    body_size = ?req.body.as_ref().map(|body| body.len()).unwrap_or_default())
)]
pub async fn make_http_request(req: HttpRequest) -> Result<HttpResponse, HttpRequestError> {
    let url = Url::parse(&req.url).map_err(|error| HttpRequestError::Other {
        reason: error.to_string(),
    })?;

    match url.scheme() {
        "http" | "https" => Ok(()),
        scheme => Err(HttpRequestError::Other {
            reason: format!("Only http and https schemes are supported, got {}", scheme),
        }),
    }?;

    trace!("making HTTP request");

    let client = reqwest::Client::new();
    let mut builder = match req.method {
        HttpRequestMethod::Delete => client.delete(url),
        HttpRequestMethod::Get => client.get(url),
        HttpRequestMethod::Head => client.head(url),
        HttpRequestMethod::Post => client.post(url),
    };
    if let Some(body) = req.body {
        builder = builder.body(body);
    }
    if let Some(headers) = req.headers {
        for (key, value) in headers.iter() {
            builder = builder.header(key, value);
        }
    }

    let response = builder.send().await.map_err(|error| {
        if error.is_timeout() {
            debug!("HTTP request timed out");
            HttpRequestError::Timeout
        } else {
            debug!(?error, "HTTP request error");
            HttpRequestError::Other {
                reason: error.to_string(),
            }
        }
    })?;

    trace!(
        status = ?response.status(),
        content_length = ?response.content_length(),
        "Got successful HTTP response",
    );

    let status_code = response.status().as_u16();
    let mut headers = HashMap::new();
    for (key, value) in response.headers().iter() {
        if let Ok(value) = value.to_str() {
            headers.insert(key.to_string(), value.to_owned());
        } else {
            error!("HTTP header containing invalid utf8 omitted in response");
        }
    }

    let body = response.bytes().await.map_err(|error| {
        error!(?error, "Failed to get response bytes");
        HttpRequestError::Other {
            reason: error.to_string(),
        }
    })?;

    trace!("Fetched {} bytes", body.len());

    let body = body.to_vec();

    match status_code {
        _ if body.len() > MAX_HTTP_RESPONSE_SIZE => Err(HttpRequestError::ResponseTooBig),
        200..=299 => Ok(HttpResponse {
            body: body.into(),
            headers,
            status_code,
        }),
        _ => Err(HttpRequestError::ServerError {
            response: body.into(),
            status_code,
        }),
    }
}

fn now() -> Timestamp {
    Timestamp::from(SystemTime::now())
}

fn log(message: String) {
    eprintln!("Provider log: {}", message);
}

fn random(len: u32) -> Vec<u8> {
    let mut rng = rand::thread_rng();
    let mut vec = Vec::with_capacity(len as usize);
    for _ in 0..vec.capacity() {
        vec.push(rng.gen());
    }
    vec
}
