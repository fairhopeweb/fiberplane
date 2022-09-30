# Notebook

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** |  | 
**revision** | **i32** |  | 
**title** | **String** |  | 
**cells** | [**Vec<crate::models::Cell>**](cell.md) |  | 
**created_at** | **String** |  | 
**created_by** | [**crate::models::CreatedBy**](createdBy.md) |  | 
**read_only** | Option<**bool**> |  | [optional]
**time_range** | [**crate::models::TimeRange**](timeRange.md) |  | 
**updated_at** | **String** |  | 
**visibility** | Option<[**crate::models::NotebookVisibility**](notebookVisibility.md)> |  | [optional]
**labels** | [**Vec<crate::models::Label>**](label.md) |  | 
**selected_data_sources** | [**::std::collections::HashMap<String, crate::models::SelectedDataSource>**](selectedDataSource.md) | This is a mapping from the provider type to the data source selected for that type | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


