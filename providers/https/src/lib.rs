use fiberplane_provider_bindings::*;
use std::convert::TryInto;
use std::{collections::HashMap, env};
use url::Url;

mod config;
mod constants;
mod provider_response;

use config::*;
use constants::*;
use provider_response::HttpsProviderResponse;

static COMMIT_HASH: &str = env!("VERGEN_GIT_SHA");
static BUILD_TIMESTAMP: &str = env!("VERGEN_BUILD_TIMESTAMP");

#[fp_export_impl(fiberplane_provider_bindings)]
async fn get_supported_query_types(config: ProviderConfig) -> Vec<SupportedQueryType> {
    let config = serde_json::from_value::<Config>(config);
    let path_label = config
        .map(|config| {
            if config.api.is_some() {
                "Path to query in the API, starting with /".to_string()
            } else {
                "Address to query, starting with https://".to_string()
            }
        })
        .unwrap_or_else(|_| "Path to query in the API, starting with /".to_string());

    vec![
        SupportedQueryType {
            query_type: PERFORM_QUERY_TYPE.to_string(),
            schema: vec![
                // TODO: Wait for Studio to implement the select field (FP-2590),
                // then use a QueryField::Select to implement the type of query
                QueryField::Text(TextField {
                    name: HTTP_METHOD_PARAM_NAME.to_string(),
                    label: "Type of query".to_string(),
                    multiline: false,
                    supports_highlighting: false,
                    prerequisites: Vec::new(),
                    required: true,
                }),
                QueryField::Text(TextField {
                    name: PATH_PARAM_NAME.to_string(),
                    label: path_label,
                    multiline: false,
                    prerequisites: Vec::new(),
                    required: true,
                    supports_highlighting: false,
                }),
                QueryField::Text(TextField {
                    name: QUERY_PARAM_NAME.to_string(),
                    label: "Query parameters. One pair key=value per line, like 'q=fiberplane'".to_string(),
                    multiline: true,
                    prerequisites: Vec::new(),
                    required: false,
                    supports_highlighting: false,
                }),
                // TODO: Wait for Studio to implement the checkbox field (FP-2593)
                // to add a FORCE_JSON_PARAM_NAME field that is just a
                // checkbox that adds an 'Accept: application/json' header
                QueryField::Text(TextField {
                    name: EXTRA_HEADERS_PARAM_NAME.to_string(),
                    label: "Extra headers to pass. One pair key=value per line, like 'Accept=application/json'".to_string(),
                    multiline: true,
                    supports_highlighting: false,
                    prerequisites: Vec::new(),
                    required: false,
                }),
            ],
            mime_types: vec![CELLS_MSGPACK_MIME_TYPE.to_string()],
        },
        SupportedQueryType {
            query_type: STATUS_QUERY_TYPE.to_string(),
            schema: vec![QueryField::Text(TextField {
                name: PATH_PARAM_NAME.to_string(),
                label: "Nothing to see here, needed to trigger a 'Run' button".to_string(),
                multiline: false,
                prerequisites: Vec::new(),
                required: true,
                supports_highlighting: false,
            })],
            mime_types: vec![
                CELLS_MSGPACK_MIME_TYPE.to_string()],
        },
    ]
}

#[fp_export_impl(fiberplane_provider_bindings)]
async fn invoke2(request: ProviderRequest) -> Result<Blob, Error> {
    log(format!(
        "https provider (commit: {}, built at: {}) invoked for query type \"{}\" and query data \"{:?}\"",
        COMMIT_HASH, BUILD_TIMESTAMP, request.query_type, request.query_data
    ));

    let config: Config =
        serde_json::from_value(request.config.clone()).map_err(|err| Error::Config {
            message: format!("Error parsing config: {:?}", err),
        })?;

    match request.query_type.as_str() {
        PERFORM_QUERY_TYPE => handle_query(config, request).await,
        STATUS_QUERY_TYPE => check_status(config).await,
        _ => Err(Error::UnsupportedRequest),
    }
}

#[fp_export_impl(fiberplane_provider_bindings)]
fn create_cells(query_type: String, response: Blob) -> Result<Vec<Cell>, Error> {
    Err(Error::Invocation {
        message: format!("create_cells is not implemented for this provider, it only returns {} blobs that must be handled by the runtime natively (received a {} blob for {}).", CELLS_MSGPACK_MIME_TYPE, response.mime_type, query_type)
        })
}

/// Send a query to the given URL
async fn send_query(
    url: &Url,
    path_and_query: &str,
    method: HttpRequestMethod,
    headers: Option<HashMap<String, String>>,
    body: Option<Blob>,
) -> Result<HttpsProviderResponse, Error> {
    let url = url
        .join(path_and_query)
        .map_err(|e| Error::Config {
            message: format!("Invalid URL: {:?}", e),
        })?
        .to_string();

    let mut headers = headers.unwrap_or_default();
    if let Some(ref blob) = body {
        headers.insert("Content-Type".to_string(), blob.mime_type.clone());
    };

    let request = HttpRequest {
        url,
        headers: Some(headers),
        method,
        body: body.map(|blob| blob.data),
    };
    log(format!(
        "Sending {:?} request to {}",
        request.method, request.url
    ));

    make_http_request(request).await.try_into()
}

async fn check_status(config: Config) -> Result<Blob, Error> {
    if let Some(api) = config.api {
        let info = send_query(
            &api.base_url,
            &api.health_check_path,
            HttpRequestMethod::Get,
            api.to_headers(),
            None,
        )
        .await?;
        Ok(info.try_into_blob(config.show_headers)?)
    } else {
        Ok(HttpsProviderResponse {
            status: "ok".to_string(),
            headers: None,
            payload: Vec::new(),
        }
        // We do not care about headers for the Ok status response
        .try_into_blob(false)?)
    }
}

async fn handle_query(config: Config, request: ProviderRequest) -> Result<Blob, Error> {
    if request.query_data.mime_type != QUERY_DATA_MIME_TYPE {
        return Err(Error::UnsupportedRequest);
    }
    let mut path = String::new();
    let mut query = String::new();
    let mut url = Err(Error::Invocation {
        message: "no URL given".to_string(),
    });
    let mut headers: Option<HashMap<String, String>> = None;
    let mut method = HttpRequestMethod::Get;
    for (key, value) in form_urlencoded::parse(&request.query_data.data) {
        match key.as_ref() {
            HTTP_METHOD_PARAM_NAME => match value.as_ref().to_uppercase().as_str() {
                "GET" => method = HttpRequestMethod::Get,
                unsupported => {
                    return Err(Error::ValidationError {
                        errors: vec![ValidationError {
                            field_name: HTTP_METHOD_PARAM_NAME.to_string(),
                            message: format!(
                                "{unsupported} is not a supported HTTPS method with this provider."
                            ),
                        }],
                    })
                }
            },
            PATH_PARAM_NAME => {
                if let Some(ref api) = config.api {
                    if value.parse::<Url>().is_ok() {
                        return Err(Error::ValidationError {
                            errors: vec![ValidationError {
                                field_name: PATH_PARAM_NAME.to_string(),
                                message: "a provider with a baseUrl cannot query arbitrary URLs"
                                    .to_string(),
                            }],
                        });
                    }
                    url = Ok(api.base_url.clone());
                    path = value.to_string();
                    if let Some(api_headers) = api.to_headers() {
                        if let Some(h) = headers.as_mut() {
                            for (k, v) in api_headers {
                                h.insert(k, v);
                            }
                        } else {
                            headers = Some(api_headers)
                        }
                    };
                } else if let Ok(full_url) = value.parse::<Url>() {
                    url = Ok(full_url);
                } else {
                    return Err(Error::ValidationError {
                        errors: vec![ValidationError {
                            field_name: PATH_PARAM_NAME.to_string(),
                            message: format!("invalid url: {value:?}"),
                        }],
                    });
                }
            }
            EXTRA_HEADERS_PARAM_NAME => {
                if headers.is_none() {
                    headers = Some(HashMap::new())
                }
                for line in value.as_ref().lines() {
                    if let Some((k, v)) = line.split_once('=') {
                        headers
                            .as_mut()
                            .map(|h| h.insert(k.to_string(), v.to_string()));
                    }
                }
            }
            QUERY_PARAM_NAME => {
                let mut serializer = form_urlencoded::Serializer::new(String::new());
                serializer.extend_pairs(
                    value
                        .as_ref()
                        .lines()
                        .filter_map(|line| line.split_once('=')),
                );
                query = serializer.finish()
            }
            _ => {
                log(format!(
                    "https provider received an unknown query parameter: {}",
                    key.as_ref()
                ));
            }
        }
    }

    let url = url?;

    if !query.is_empty() {
        path = format!("{path}?{query}");
    }

    send_query(&url, &path, method, headers, None)
        .await
        .and_then(|resp| resp.try_into_blob(config.show_headers))
}
