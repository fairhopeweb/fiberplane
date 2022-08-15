# LogCell

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**_type** | [**crate::models::CellType**](cellType.md) |  | 
**id** | **String** |  | 
**read_only** | Option<**bool**> |  | [optional]
**source_ids** | **Vec<String>** |  | 
**title** | **String** |  | 
**formatting** | Option<[**Vec<crate::models::Annotation>**](annotation.md)> |  | [optional]
**time_range** | Option<[**crate::models::TimeRange**](timeRange.md)> |  | [optional]
**hide_similar_values** | Option<**bool**> |  | [optional]
**display_fields** | Option<**Vec<String>**> |  | [optional]
**expanded_indices** | Option<[**Vec<crate::models::ExpandedIndex>**](expandedIndex.md)> |  | [optional]
**data** | Option<[**::std::collections::HashMap<String, Vec<crate::models::LogRecord>>**](array.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


