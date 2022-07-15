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
pub struct Notebook {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "revision")]
    pub revision: i32,
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "cells")]
    pub cells: Vec<crate::models::Cell>,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "createdBy")]
    pub created_by: Box<crate::models::CreatedBy>,
    #[serde(rename = "dataSources", skip_serializing_if = "Option::is_none")]
    pub data_sources:
        Option<::std::collections::HashMap<String, crate::models::NotebookDataSource>>,
    #[serde(rename = "readOnly", skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    #[serde(rename = "timeRange")]
    pub time_range: Box<crate::models::TimeRange>,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
    #[serde(rename = "visibility", skip_serializing_if = "Option::is_none")]
    pub visibility: Option<crate::models::NotebookVisibility>,
    #[serde(rename = "labels")]
    pub labels: Vec<crate::models::Label>,
}

impl Notebook {
    pub fn new(
        id: String,
        revision: i32,
        title: String,
        cells: Vec<crate::models::Cell>,
        created_at: String,
        created_by: crate::models::CreatedBy,
        time_range: crate::models::TimeRange,
        updated_at: String,
        labels: Vec<crate::models::Label>,
    ) -> Notebook {
        Notebook {
            id,
            revision,
            title,
            cells,
            created_at,
            created_by: Box::new(created_by),
            data_sources: None,
            read_only: None,
            time_range: Box::new(time_range),
            updated_at,
            visibility: None,
            labels,
        }
    }
}
