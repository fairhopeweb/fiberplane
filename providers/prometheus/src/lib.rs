mod auto_suggest;
mod config;
mod constants;
mod instants;
mod prometheus;
mod timeseries;

use auto_suggest::query_suggestions;
use config::PrometheusConfig;
use constants::*;
use fiberplane::protocols::providers::{
    STATUS_MIME_TYPE, STATUS_QUERY_TYPE, SUGGESTIONS_QUERY_TYPE, TIMESERIES_QUERY_TYPE,
};
use fp_provider_bindings::*;
use instants::query_instants;
use timeseries::{create_graph_cell, query_series};
use url::Url;

#[fp_export_impl(fp_provider_bindings)]
async fn get_supported_query_types(_config: ProviderConfig) -> Vec<SupportedQueryType> {
    vec![
        SupportedQueryType {
            query_type: INSTANTS_QUERY_TYPE.to_owned(),
            schema: vec![QueryField::Text(TextField {
                name: QUERY_PARAM_NAME.to_owned(),
                label: "Enter your Prometheus query".to_owned(),
                multiline: false,
                prerequisites: Vec::new(),
                required: true,
                supports_highlighting: false,
            })],
            mime_types: vec![CELLS_MSGPACK_MIME_TYPE.to_owned()],
        },
        SupportedQueryType {
            query_type: TIMESERIES_QUERY_TYPE.to_owned(),
            schema: vec![
                QueryField::Text(TextField {
                    name: QUERY_PARAM_NAME.to_owned(),
                    label: "Enter your Prometheus query".to_owned(),
                    multiline: false,
                    prerequisites: Vec::new(),
                    required: true,
                    supports_highlighting: false,
                }),
                QueryField::DateTimeRange(DateTimeRangeField {
                    name: TIME_RANGE_PARAM_NAME.to_owned(),
                    label: "Specify a time range".to_owned(),
                    required: true,
                }),
            ],
            mime_types: vec![TIMESERIES_MSGPACK_MIME_TYPE.to_owned()],
        },
        SupportedQueryType {
            query_type: SUGGESTIONS_QUERY_TYPE.to_owned(),
            schema: Vec::new(),
            mime_types: vec![SUGGESTIONS_MSGPACK_MIME_TYPE.to_owned()],
        },
        SupportedQueryType {
            query_type: STATUS_QUERY_TYPE.to_owned(),
            schema: Vec::new(),
            mime_types: vec![STATUS_MIME_TYPE.to_owned()],
        },
    ]
}

#[fp_export_impl(fp_provider_bindings)]
async fn invoke2(request: ProviderRequest) -> Result<Blob, Error> {
    log(format!(
        "Prometheus provider invoked for query type \"{}\" and query data \"{:?}\"",
        request.query_type, request.query_data
    ));

    let config: PrometheusConfig =
        serde_json::from_value(request.config).map_err(|err| Error::Config {
            message: format!("Error parsing config: {:?}", err),
        })?;

    match request.query_type.as_str() {
        INSTANTS_QUERY_TYPE => query_instants(request.query_data, config).await,
        TIMESERIES_QUERY_TYPE => query_series(request.query_data, config).await,
        SUGGESTIONS_QUERY_TYPE => query_suggestions(request.query_data, config).await,
        STATUS_QUERY_TYPE => check_status(config).await,
        _ => Err(Error::UnsupportedRequest),
    }
}

#[fp_export_impl(fp_provider_bindings)]
fn create_cells(query_type: String, _response: Blob) -> Result<Vec<Cell>, Error> {
    log(format!("Creating cells for query type: {query_type}"));

    match query_type.as_str() {
        INSTANTS_QUERY_TYPE => todo!("Instants support is not currently implemented"),
        TIMESERIES_QUERY_TYPE => create_graph_cell(),
        _ => Err(Error::UnsupportedRequest),
    }
}

async fn check_status(config: PrometheusConfig) -> Result<Blob, Error> {
    let mut url = Url::parse(&config.url).map_err(|e| Error::Config {
        message: format!("Invalid prometheus URL: {:?}", e),
    })?;

    // Send a fake query to the query endpoint to check if we can connect to the
    // Prometheus instance. We should get a 200 response even though it won't
    // return any data.
    url.path_segments_mut()
        .map_err(|_| Error::Config {
            message: "Invalid prometheus URL: cannot-be-a-base".to_string(),
        })?
        .push("api")
        .push("v1")
        .push("query");
    url.query_pairs_mut()
        .append_pair("query", "fiberplane_check_status");

    make_http_request(HttpRequest {
        body: None,
        headers: None,
        method: HttpRequestMethod::Get,
        url: url.to_string(),
    })
    .await
    .map_err(|error| Error::Http { error })?;

    Ok(Blob {
        mime_type: STATUS_MIME_TYPE.to_owned(),
        data: "ok".into(),
    })
}
