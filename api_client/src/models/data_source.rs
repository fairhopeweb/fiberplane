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
#[serde(tag = "type")]
pub enum DataSource {
    #[serde(rename="elasticsearch")]
    ElasticsearchDataSource {
        #[serde(rename = "url")]
        url: String,
    },
    #[serde(rename="loki")]
    LokiDataSource {
        #[serde(rename = "url")]
        url: String,
    },
    #[serde(rename="prometheus")]
    PrometheusDataSource {
        #[serde(rename = "url")]
        url: String,
    },
    #[serde(rename="proxy")]
    ProxyDataSource {
        #[serde(rename = "proxyId")]
        proxy_id: String,
        #[serde(rename = "dataSourceName")]
        data_source_name: String,
        #[serde(rename = "dataSourceType")]
        data_source_type: crate::models::DataSourceType,
    },
}




