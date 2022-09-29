use super::{config::PrometheusConfig, constants::*, prometheus::*};
use fiberplane::protocols::providers::{AutoSuggestRequest, Suggestion};
use fp_provider_bindings::*;

/// See: https://prometheus.io/docs/prometheus/latest/querying/functions/
const PROM_QL_FUNCTIONS: &[&str] = &[
    "abs",
    "absent",
    "ceil",
    "changes",
    "clamp_max",
    "clamp_min",
    "day_of_month",
    "day_of_week",
    "days_in_month",
    "delta",
    "deriv",
    "exp",
    "floor",
    "histogram_quantile",
    "holt_winters",
    "hour",
    "idelta",
    "increase",
    "irate",
    "label_join",
    "label_replace",
    "ln",
    "log2",
    "log10",
    "minute",
    "month",
    "predict_linear",
    "rate",
    "resets",
    "round",
    "scalar",
    "sort",
    "sort_desc",
    "sqrt",
    "time",
    "timestamp",
    "vector",
    "year",
    "avg_over_time",
    "min_over_time",
    "max_over_time",
    "sum_over_time",
    "count_over_time",
    "quantile_over_time",
    "stddev_over_time",
    "stdvar_over_time",
];

pub async fn query_suggestions(query_data: Blob, config: PrometheusConfig) -> Result<Blob, Error> {
    let query = AutoSuggestRequest::from_query_data(&query_data)?.query;
    let identifier = extract_identifier(&query);

    let response = make_http_request(HttpRequest {
        body: None,
        headers: None,
        method: HttpRequestMethod::Get,
        url: format!("{}/api/v1/metadata", config.url),
    })
    .await?;

    let mut suggestions = from_metadata(&response.body)?;
    if !identifier.is_empty() {
        suggestions = suggestions
            .into_iter()
            .filter(|suggestion| {
                suggestion.text.contains(identifier)
                    || suggestion
                        .description
                        .as_ref()
                        .map(|description| description.contains(identifier))
                        .unwrap_or_default()
            })
            .collect()
    }
    for &function in PROM_QL_FUNCTIONS {
        if identifier.is_empty() || function.contains(identifier) {
            suggestions.push(Suggestion {
                text: function.to_owned(),
                description: Some("Function".to_owned()),
            })
        }
    }

    Ok(Blob {
        data: rmp_serde::to_vec_named(&suggestions)?.into(),
        mime_type: SUGGESTIONS_MSGPACK_MIME_TYPE.to_owned(),
    })
}

/// Extracts the identifier that is currently being typed from the query. This
/// identifier is used to filter the suggestions. If the identifier is empty,
/// no filtering would be applied.
fn extract_identifier(query: &str) -> &str {
    let chars: Vec<char> = query.chars().collect();
    if let Some((offset, _)) = chars
        .iter()
        .enumerate()
        .rev()
        .find(|(_, &c)| !is_identifier_char(c))
    {
        &query[(offset + 1)..]
    } else {
        query.trim()
    }
}

fn from_metadata(response: &[u8]) -> Result<Vec<Suggestion>, Error> {
    let response = match serde_json::from_slice::<PrometheusMetadataResponse>(response) {
        Ok(response) => response.data,
        Err(error) => {
            return Err(Error::Data {
                message: format!("Error parsing response: {}", error),
            })
        }
    };

    Ok(response
        .into_iter()
        .filter_map(|(name, values)| {
            values.into_iter().next().map(|value| Suggestion {
                text: name,
                description: value.help,
            })
        })
        .collect())
}

fn is_letter(c: char) -> bool {
    ('A'..='Z').contains(&c) || ('a'..='z').contains(&c)
}

fn is_number(c: char) -> bool {
    ('0'..='9').contains(&c)
}

fn is_identifier_char(c: char) -> bool {
    is_letter(c) || is_number(c) || c == '_'
}

#[cfg(test)]
mod tests {
    use crate::auto_suggest::extract_identifier;

    #[test]
    fn test_extract_identifier() {
        assert_eq!(extract_identifier("hello"), "hello");
        assert_eq!(extract_identifier("hello foo"), "foo");
        assert_eq!(extract_identifier("hello!foo"), "foo");
        assert_eq!(extract_identifier("##@!"), "");
    }
}
