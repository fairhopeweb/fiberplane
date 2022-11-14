mod auto_suggest;
mod constants;
mod instants;
mod prometheus;
mod timeseries;

use auto_suggest::query_suggestions;
use constants::*;
use fiberplane::protocols::providers::{
    STATUS_MIME_TYPE, STATUS_QUERY_TYPE, SUGGESTIONS_QUERY_TYPE, TIMESERIES_QUERY_TYPE,
};
use fp_provider_bindings::*;
use grafana_common::{query_direct_and_proxied, Config};
use instants::query_instants;
use serde_json::Value;
use std::env;
use timeseries::{create_graph_cell, query_series};

static COMMIT_HASH: &str = env!("VERGEN_GIT_SHA");
static BUILD_TIMESTAMP: &str = env!("VERGEN_BUILD_TIMESTAMP");

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
                QueryField::Checkbox(CheckboxField {
                    name: LIVE_PARAM_NAME.to_owned(),
                    label: "Enable live mode".to_owned(),
                    checked: false,
                    value: "true".to_owned(),
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
        "Prometheus provider (commit: {}, built at: {}) invoked for query type \"{}\" and query data \"{:?}\"",
        COMMIT_HASH, BUILD_TIMESTAMP, request.query_type, request.query_data
    ));

    let config: Config = serde_json::from_value(request.config).map_err(|err| Error::Config {
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

async fn check_status(config: Config) -> Result<Blob, Error> {
    // Send a fake query to the query endpoint to check if we can connect to the Prometheus
    // instance. We should get a 200 response even though it won't return any data.
    query_direct_and_proxied::<Value>(
        &config,
        "prometheus",
        "api/v1/query?query=fiberplane_check_status",
        None,
    )
    .await?;

    Ok(Blob {
        mime_type: STATUS_MIME_TYPE.to_owned(),
        data: "ok".into(),
    })
}
