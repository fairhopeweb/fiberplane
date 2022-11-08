# Cell

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**_type** | [**crate::models::CellType**](cellType.md) |  | 
**id** | **String** |  | 
**checked** | **bool** |  | 
**content** | **String** |  | 
**formatting** | Option<[**Vec<crate::models::Annotation>**](annotation.md)> |  | [optional]
**level** | Option<**i32**> |  | [optional]
**read_only** | Option<**bool**> |  | [optional]
**syntax** | Option<**String**> |  | [optional]
**thread_id** | **String** |  | 
**data_links** | **Vec<String>** |  | 
**graph_type** | **String** |  | 
**stacking_type** | **String** |  | 
**heading_type** | **String** |  | 
**url** | Option<**String**> |  | [optional]
**file_id** | Option<**String**> |  | [optional]
**progress** | Option<**f32**> |  | [optional]
**width** | Option<**i32**> |  | [optional]
**height** | Option<**i32**> |  | [optional]
**preview** | Option<**String**> |  | [optional]
**list_type** | **String** |  | 
**start_number** | Option<**i32**> |  | [optional]
**hide_similar_values** | Option<**bool**> |  | [optional]
**display_fields** | Option<**Vec<String>**> |  | [optional]
**expanded_indices** | Option<[**Vec<crate::models::LogRecordIndex>**](logRecordIndex.md)> |  | [optional]
**visibility_filter** | Option<[**crate::models::LogCellVisibility**](logCellVisibility.md)> |  | [optional]
**selected_indices** | Option<[**Vec<crate::models::LogRecordIndex>**](logRecordIndex.md)> |  | [optional]
**highlighted_indices** | Option<[**Vec<crate::models::LogRecordIndex>**](logRecordIndex.md)> |  | [optional]
**intent** | **String** |  | 
**query_data** | Option<**String**> |  | [optional]
**response** | Option<[**crate::models::EncodedBlob**](encodedBlob.md)> |  | [optional]
**output** | [**Vec<crate::models::Cell>**](cell.md) |  | 
**title** | **String** |  | 
**rows** | [**crate::models::TableRow**](tableRow.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


