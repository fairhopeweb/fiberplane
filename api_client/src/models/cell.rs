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
pub enum Cell {
    #[serde(rename="checkbox")]
    CheckboxCell {
        #[serde(rename = "id")]
        id: String,
        #[serde(rename = "checked")]
        checked: bool,
        #[serde(rename = "content")]
        content: String,
        #[serde(rename = "formatting", skip_serializing_if = "Option::is_none")]
        formatting: Option<Vec<crate::models::Annotation>>,
        #[serde(rename = "level", skip_serializing_if = "Option::is_none")]
        level: Option<i32>,
        #[serde(rename = "readOnly", skip_serializing_if = "Option::is_none")]
        read_only: Option<bool>,
    },
    #[serde(rename="code")]
    CodeCell {
        #[serde(rename = "id")]
        id: String,
        #[serde(rename = "content")]
        content: String,
        #[serde(rename = "readOnly", skip_serializing_if = "Option::is_none")]
        read_only: Option<bool>,
        #[serde(rename = "syntax", skip_serializing_if = "Option::is_none")]
        syntax: Option<String>,
    },
    #[serde(rename="divider")]
    DividerCell {
        #[serde(rename = "id")]
        id: String,
        #[serde(rename = "readOnly", skip_serializing_if = "Option::is_none")]
        read_only: Option<bool>,
    },
    #[serde(rename="elasticsearch")]
    ElasticsearchCell {
        #[serde(rename = "id")]
        id: String,
        #[serde(rename = "content")]
        content: String,
        #[serde(rename = "readOnly", skip_serializing_if = "Option::is_none")]
        read_only: Option<bool>,
    },
    #[serde(rename="graph")]
    GraphCell {
        #[serde(rename = "id")]
        id: String,
        #[serde(rename = "graphType")]
        graph_type: GraphType,
        #[serde(rename = "stackingType")]
        stacking_type: StackingType,
        #[serde(rename = "readOnly", skip_serializing_if = "Option::is_none")]
        read_only: Option<bool>,
        #[serde(rename = "sourceIds")]
        source_ids: Vec<String>,
        #[serde(rename = "timeRange", skip_serializing_if = "Option::is_none")]
        time_range: Option<Box<crate::models::TimeRange>>,
        #[serde(rename = "title")]
        title: String,
        #[serde(rename = "formatting", skip_serializing_if = "Option::is_none")]
        formatting: Option<Vec<crate::models::Annotation>>,
        #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
        data: Option<::std::collections::HashMap<String, Vec<crate::models::Series>>>,
    },
    #[serde(rename="heading")]
    HeadingCell {
        #[serde(rename = "id")]
        id: String,
        #[serde(rename = "headingType")]
        heading_type: HeadingType,
        #[serde(rename = "content")]
        content: String,
        #[serde(rename = "formatting", skip_serializing_if = "Option::is_none")]
        formatting: Option<Vec<crate::models::Annotation>>,
        #[serde(rename = "readOnly", skip_serializing_if = "Option::is_none")]
        read_only: Option<bool>,
    },
    #[serde(rename="image")]
    ImageCell {
        #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
        url: Option<String>,
        #[serde(rename = "id")]
        id: String,
        #[serde(rename = "fileId", skip_serializing_if = "Option::is_none")]
        file_id: Option<String>,
        #[serde(rename = "progress", skip_serializing_if = "Option::is_none")]
        progress: Option<f32>,
        #[serde(rename = "readOnly", skip_serializing_if = "Option::is_none")]
        read_only: Option<bool>,
        #[serde(rename = "width", skip_serializing_if = "Option::is_none")]
        width: Option<i32>,
        #[serde(rename = "height", skip_serializing_if = "Option::is_none")]
        height: Option<i32>,
        #[serde(rename = "preview", skip_serializing_if = "Option::is_none")]
        preview: Option<String>,
    },
    #[serde(rename="list_item")]
    ListItemCell {
        #[serde(rename = "id")]
        id: String,
        #[serde(rename = "listType")]
        list_type: ListType,
        #[serde(rename = "content")]
        content: String,
        #[serde(rename = "formatting", skip_serializing_if = "Option::is_none")]
        formatting: Option<Vec<crate::models::Annotation>>,
        #[serde(rename = "level", skip_serializing_if = "Option::is_none")]
        level: Option<i32>,
        #[serde(rename = "readOnly", skip_serializing_if = "Option::is_none")]
        read_only: Option<bool>,
        #[serde(rename = "startNumber", skip_serializing_if = "Option::is_none")]
        start_number: Option<i32>,
    },
    #[serde(rename="log")]
    LogCell {
        #[serde(rename = "id")]
        id: String,
        #[serde(rename = "readOnly", skip_serializing_if = "Option::is_none")]
        read_only: Option<bool>,
        #[serde(rename = "sourceIds")]
        source_ids: Vec<String>,
        #[serde(rename = "title")]
        title: String,
        #[serde(rename = "formatting", skip_serializing_if = "Option::is_none")]
        formatting: Option<Vec<crate::models::Annotation>>,
        #[serde(rename = "timeRange", skip_serializing_if = "Option::is_none")]
        time_range: Option<Box<crate::models::TimeRange>>,
        #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
        data: Option<::std::collections::HashMap<String, Vec<crate::models::LogRecord>>>,
    },
    #[serde(rename="loki")]
    LokiCell {
        #[serde(rename = "id")]
        id: String,
        #[serde(rename = "content")]
        content: String,
        #[serde(rename = "readOnly", skip_serializing_if = "Option::is_none")]
        read_only: Option<bool>,
    },
    #[serde(rename="prometheus")]
    PrometheusCell {
        #[serde(rename = "id")]
        id: String,
        #[serde(rename = "content")]
        content: String,
        #[serde(rename = "readOnly", skip_serializing_if = "Option::is_none")]
        read_only: Option<bool>,
    },
    #[serde(rename="table")]
    TableCell {
        #[serde(rename = "id")]
        id: String,
        #[serde(rename = "readOnly", skip_serializing_if = "Option::is_none")]
        read_only: Option<bool>,
        #[serde(rename = "sourceIds")]
        source_ids: Vec<String>,
        #[serde(rename = "title")]
        title: String,
        #[serde(rename = "formatting", skip_serializing_if = "Option::is_none")]
        formatting: Option<Vec<crate::models::Annotation>>,
        #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
        data: Option<::std::collections::HashMap<String, Vec<crate::models::Instant>>>,
    },
    #[serde(rename="text")]
    TextCell {
        #[serde(rename = "id")]
        id: String,
        #[serde(rename = "content")]
        content: String,
        #[serde(rename = "formatting", skip_serializing_if = "Option::is_none")]
        formatting: Option<Vec<crate::models::Annotation>>,
        #[serde(rename = "readOnly", skip_serializing_if = "Option::is_none")]
        read_only: Option<bool>,
    },
}



/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum GraphType {
    #[serde(rename = "bar")]
    Bar,
    #[serde(rename = "line")]
    Line,
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum StackingType {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "stacked")]
    Stacked,
    #[serde(rename = "percentage")]
    Percentage,
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum HeadingType {
    #[serde(rename = "h1")]
    H1,
    #[serde(rename = "h2")]
    H2,
    #[serde(rename = "h3")]
    H3,
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ListType {
    #[serde(rename = "ordered")]
    Ordered,
    #[serde(rename = "unordered")]
    Unordered,
}

