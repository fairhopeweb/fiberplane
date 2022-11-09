# \DefaultApi

All URIs are relative to *https://fiberplane.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**comment_create**](DefaultApi.md#comment_create) | **POST** /api/threads/{threadId}/comments | 
[**comment_delete**](DefaultApi.md#comment_delete) | **DELETE** /api/comments/{commentId} | 
[**comment_get**](DefaultApi.md#comment_get) | **GET** /api/comments/{commentId} | 
[**comment_update**](DefaultApi.md#comment_update) | **PATCH** /api/comments/{commentId} | 
[**data_source_create**](DefaultApi.md#data_source_create) | **POST** /api/workspaces/{workspace_id}/data_sources | 
[**data_source_delete**](DefaultApi.md#data_source_delete) | **DELETE** /api/workspaces/{workspace_id}/data_sources/{data_source_name} | 
[**data_source_get**](DefaultApi.md#data_source_get) | **GET** /api/workspaces/{workspace_id}/data_sources/{data_source_name} | 
[**data_source_list**](DefaultApi.md#data_source_list) | **GET** /api/workspaces/{workspace_id}/data_sources | 
[**data_source_update**](DefaultApi.md#data_source_update) | **PATCH** /api/workspaces/{workspace_id}/data_sources/{data_source_name} | 
[**event_create**](DefaultApi.md#event_create) | **POST** /api/workspaces/{workspace_id}/events | 
[**event_delete**](DefaultApi.md#event_delete) | **DELETE** /api/events/{event_id} | 
[**event_list**](DefaultApi.md#event_list) | **GET** /api/workspaces/{workspace_id}/events | 
[**file_delete**](DefaultApi.md#file_delete) | **DELETE** /api/notebooks/{notebookId}/files/{fileId} | 
[**file_get**](DefaultApi.md#file_get) | **GET** /api/notebooks/{notebookId}/files/{fileId} | 
[**file_upload**](DefaultApi.md#file_upload) | **POST** /api/notebooks/{notebookId}/files | 
[**label_keys_list**](DefaultApi.md#label_keys_list) | **GET** /api/workspaces/{workspace_id}/labels/keys | 
[**label_values_list**](DefaultApi.md#label_values_list) | **GET** /api/workspaces/{workspace_id}/labels/values/{label_key} | 
[**logout**](DefaultApi.md#logout) | **POST** /api/logout | 
[**notebook_cell_append_text**](DefaultApi.md#notebook_cell_append_text) | **POST** /api/notebooks/{notebookId}/cells/{cellId}/append | 
[**notebook_cell_replace_text**](DefaultApi.md#notebook_cell_replace_text) | **POST** /api/notebooks/{notebookId}/cells/{cellId}/replaceText | 
[**notebook_cells_append**](DefaultApi.md#notebook_cells_append) | **POST** /api/notebooks/{notebookId}/cells | 
[**notebook_convert_to_template**](DefaultApi.md#notebook_convert_to_template) | **GET** /api/notebooks/{notebookId}/template.jsonnet | 
[**notebook_create**](DefaultApi.md#notebook_create) | **POST** /api/workspaces/{workspace_id}/notebooks | 
[**notebook_delete**](DefaultApi.md#notebook_delete) | **DELETE** /api/notebooks/{notebookId} | 
[**notebook_get**](DefaultApi.md#notebook_get) | **GET** /api/notebooks/{notebookId} | 
[**notebook_list**](DefaultApi.md#notebook_list) | **GET** /api/workspaces/{workspace_id}/notebooks | 
[**notebook_search**](DefaultApi.md#notebook_search) | **POST** /api/workspaces/{workspace_id}/search/notebooks | 
[**notebook_update**](DefaultApi.md#notebook_update) | **PATCH** /api/notebooks/{notebookId} | 
[**oidc_authorize_google**](DefaultApi.md#oidc_authorize_google) | **GET** /api/oidc/authorize/google | 
[**pinned_notebook_create**](DefaultApi.md#pinned_notebook_create) | **POST** /api/pinnednotebooks | 
[**pinned_notebook_delete**](DefaultApi.md#pinned_notebook_delete) | **DELETE** /api/pinnednotebooks/{notebookId} | 
[**pinned_notebook_list**](DefaultApi.md#pinned_notebook_list) | **GET** /api/workspaces/{workspace_id}/pinnednotebooks | 
[**profile_get**](DefaultApi.md#profile_get) | **GET** /api/profile | 
[**profile_picture_get**](DefaultApi.md#profile_picture_get) | **GET** /api/profile/picture | 
[**profile_picture_update**](DefaultApi.md#profile_picture_update) | **POST** /api/profile/picture | 
[**proxy_create**](DefaultApi.md#proxy_create) | **POST** /api/workspaces/{workspace_id}/proxies | 
[**proxy_delete**](DefaultApi.md#proxy_delete) | **DELETE** /api/workspaces/{workspace_id}/proxies/{proxy_name} | 
[**proxy_get**](DefaultApi.md#proxy_get) | **GET** /api/workspaces/{workspace_id}/proxies/{proxy_name} | 
[**proxy_list**](DefaultApi.md#proxy_list) | **GET** /api/workspaces/{workspace_id}/proxies | 
[**proxy_relay**](DefaultApi.md#proxy_relay) | **POST** /api/workspaces/{workspace_id}/proxies/{proxy_name}/data_sources/{data_source_name}/relay | 
[**template_create**](DefaultApi.md#template_create) | **POST** /api/workspaces/{workspace_id}/templates | 
[**template_delete**](DefaultApi.md#template_delete) | **DELETE** /api/workspaces/{workspace_id}/templates/{templateName} | 
[**template_expand**](DefaultApi.md#template_expand) | **POST** /api/workspaces/{workspace_id}/templates/{templateName}/expand | 
[**template_get**](DefaultApi.md#template_get) | **GET** /api/workspaces/{workspace_id}/templates/{templateName} | 
[**template_list**](DefaultApi.md#template_list) | **GET** /api/workspaces/{workspace_id}/templates | 
[**template_update**](DefaultApi.md#template_update) | **PATCH** /api/workspaces/{workspace_id}/templates/{templateName} | 
[**thread_create**](DefaultApi.md#thread_create) | **POST** /api/notebooks/{notebookId}/threads | 
[**thread_delete**](DefaultApi.md#thread_delete) | **DELETE** /api/threads/{threadId} | 
[**thread_get**](DefaultApi.md#thread_get) | **GET** /api/threads/{threadId} | 
[**thread_reopen**](DefaultApi.md#thread_reopen) | **POST** /api/threads/{threadId}/reopen | 
[**thread_resolve**](DefaultApi.md#thread_resolve) | **POST** /api/threads/{threadId}/resolve | 
[**threads_list**](DefaultApi.md#threads_list) | **GET** /api/notebooks/{notebookId}/threads | 
[**token_create**](DefaultApi.md#token_create) | **POST** /api/tokens | 
[**token_delete**](DefaultApi.md#token_delete) | **DELETE** /api/tokens/{id} | 
[**token_list**](DefaultApi.md#token_list) | **GET** /api/tokens | 
[**trigger_create**](DefaultApi.md#trigger_create) | **POST** /api/workspaces/{workspace_id}/triggers | 
[**trigger_delete**](DefaultApi.md#trigger_delete) | **DELETE** /api/triggers/{triggerId} | 
[**trigger_get**](DefaultApi.md#trigger_get) | **GET** /api/triggers/{triggerId} | 
[**trigger_invoke**](DefaultApi.md#trigger_invoke) | **POST** /api/triggers/{triggerId}/{secretKey} | 
[**trigger_list**](DefaultApi.md#trigger_list) | **GET** /api/workspaces/{workspace_id}/triggers | 
[**workspace_create**](DefaultApi.md#workspace_create) | **POST** /api/workspaces | 
[**workspace_delete**](DefaultApi.md#workspace_delete) | **DELETE** /api/workspaces/{workspace_id} | 
[**workspace_get**](DefaultApi.md#workspace_get) | **GET** /api/workspaces/{workspace_id} | 
[**workspace_invite**](DefaultApi.md#workspace_invite) | **POST** /api/workspaces/{workspace_id}/invitations | 
[**workspace_invite_accept**](DefaultApi.md#workspace_invite_accept) | **POST** /api/invitations/{invitation_id}/{invitation_secret}/accept | 
[**workspace_invite_decline**](DefaultApi.md#workspace_invite_decline) | **POST** /api/invitations/{invitation_id}/{invitation_secret}/decline | 
[**workspace_invite_delete**](DefaultApi.md#workspace_invite_delete) | **DELETE** /api/invitations/{invitation_id} | 
[**workspace_invite_get**](DefaultApi.md#workspace_invite_get) | **GET** /api/workspaces/{workspace_id}/invitations | 
[**workspace_leave**](DefaultApi.md#workspace_leave) | **POST** /api/workspaces/{workspace_id}/leave | 
[**workspace_list**](DefaultApi.md#workspace_list) | **GET** /api/workspaces | 
[**workspace_picture_get**](DefaultApi.md#workspace_picture_get) | **GET** /api/workspaces/{workspace_id}/picture | 
[**workspace_picture_update**](DefaultApi.md#workspace_picture_update) | **POST** /api/workspaces/{workspace_id}/picture | 
[**workspace_update**](DefaultApi.md#workspace_update) | **PATCH** /api/workspaces/{workspace_id} | 
[**workspace_user_remove**](DefaultApi.md#workspace_user_remove) | **DELETE** /api/workspaces/{workspace_id}/users/{user_id} | 
[**workspace_user_update**](DefaultApi.md#workspace_user_update) | **PATCH** /api/workspaces/{workspace_id}/users/{user_id} | 
[**workspace_users_list**](DefaultApi.md#workspace_users_list) | **GET** /api/workspaces/{workspace_id}/users | 



## comment_create

> crate::models::Comment comment_create(thread_id, new_comment)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**thread_id** | **String** |  | [required] |
**new_comment** | [**NewComment**](NewComment.md) |  | [required] |

### Return type

[**crate::models::Comment**](comment.md)

### Authorization

[userToken](../README.md#userToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## comment_delete

> comment_delete(comment_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**comment_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[userToken](../README.md#userToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## comment_get

> crate::models::Comment comment_get(comment_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**comment_id** | **String** |  | [required] |

### Return type

[**crate::models::Comment**](comment.md)

### Authorization

[userToken](../README.md#userToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## comment_update

> crate::models::Comment comment_update(comment_id, update_comment)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**comment_id** | **String** |  | [required] |
**update_comment** | [**UpdateComment**](UpdateComment.md) |  | [required] |

### Return type

[**crate::models::Comment**](comment.md)

### Authorization

[userToken](../README.md#userToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## data_source_create

> crate::models::DataSource data_source_create(workspace_id, new_data_source)


Create an workspace data-source

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace_id** | **String** | The workspace ID | [required] |
**new_data_source** | [**NewDataSource**](NewDataSource.md) | New data source | [required] |

### Return type

[**crate::models::DataSource**](dataSource.md)

### Authorization

[userToken](../README.md#userToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## data_source_delete

> data_source_delete(workspace_id, data_source_name)


Delete a data source

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace_id** | **String** | The workspace ID | [required] |
**data_source_name** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[userToken](../README.md#userToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## data_source_get

> crate::models::DataSource data_source_get(workspace_id, data_source_name)


Get the data source's details

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace_id** | **String** | The workspace ID | [required] |
**data_source_name** | **String** |  | [required] |

### Return type

[**crate::models::DataSource**](dataSource.md)

### Authorization

[userToken](../README.md#userToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## data_source_list

> Vec<crate::models::DataSource> data_source_list(workspace_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace_id** | **String** | The workspace ID | [required] |

### Return type

[**Vec<crate::models::DataSource>**](dataSource.md)

### Authorization

[userToken](../README.md#userToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## data_source_update

> crate::models::DataSource data_source_update(workspace_id, data_source_name, update_data_source)


Update a data source

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace_id** | **String** | The workspace ID | [required] |
**data_source_name** | **String** |  | [required] |
**update_data_source** | [**UpdateDataSource**](UpdateDataSource.md) |  | [required] |

### Return type

[**crate::models::DataSource**](dataSource.md)

### Authorization

[userToken](../README.md#userToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## event_create

> crate::models::Event event_create(workspace_id, new_event)


Creates a new event

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace_id** | **String** | The workspace ID | [required] |
**new_event** | [**NewEvent**](NewEvent.md) | Event creation payload | [required] |

### Return type

[**crate::models::Event**](event.md)

### Authorization

[userToken](../README.md#userToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## event_delete

> event_delete(event_id)


Deletes a event

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**event_id** | **String** | ID of the event | [required] |

### Return type

 (empty response body)

### Authorization

[userToken](../README.md#userToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## event_list

> Vec<crate::models::Event> event_list(workspace_id, occurrence_time_start, occurrence_time_end, labels, sort_by, sort_direction, page, limit)


Get a list of all events matching the query

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace_id** | **String** | The workspace ID | [required] |
**occurrence_time_start** | **String** |  | [required] |
**occurrence_time_end** | **String** |  | [required] |
**labels** | Option<[**::std::collections::HashMap<String, String>**](String.md)> |  |  |
**sort_by** | Option<**String**> | Sort the resulting list by the following field (defaults to occurrence_time) |  |
**sort_direction** | Option<**String**> | Sort the resulting list in the following direction (defaults to ascending) |  |
**page** | Option<**i32**> | Page number which should be displayed; 0-indexed (defaults to 0) |  |
**limit** | Option<**i32**> | Maximum amount of results to display per page (defaults to 20) |  |

### Return type

[**Vec<crate::models::Event>**](event.md)

### Authorization

[userToken](../README.md#userToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## file_delete

> file_delete(notebook_id, file_id)


Delete a file

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**notebook_id** | **String** |  | [required] |
**file_id** | **String** | ID of the file | [required] |

### Return type

 (empty response body)

### Authorization

[userToken](../README.md#userToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## file_get

> std::path::PathBuf file_get(notebook_id, file_id)


Get a file

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**notebook_id** | **String** |  | [required] |
**file_id** | **String** | ID of the file | [required] |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: image/_*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## file_upload

> crate::models::FileSummary file_upload(notebook_id, file)


upload a file

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**notebook_id** | **String** |  | [required] |
**file** | **std::path::PathBuf** |  | [required] |

### Return type

[**crate::models::FileSummary**](fileSummary.md)

### Authorization

[userToken](../README.md#userToken)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## label_keys_list

> Vec<String> label_keys_list(workspace_id, prefix)


Retrieve all label keys

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace_id** | **String** | The workspace ID | [required] |
**prefix** | Option<**String**> | Prefix of the label key |  |

### Return type

**Vec<String>**

### Authorization

[userToken](../README.md#userToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## label_values_list

> Vec<String> label_values_list(workspace_id, label_key, prefix)


Retrieve all label values for a specific key

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace_id** | **String** | The workspace ID | [required] |
**label_key** | **String** | The label key | [required] |
**prefix** | Option<**String**> | Prefix of the label value |  |

### Return type

**Vec<String>**

### Authorization

[userToken](../README.md#userToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## logout

> logout()


Log out of Fiberplane

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[userToken](../README.md#userToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## notebook_cell_append_text

> crate::models::Cell notebook_cell_append_text(notebook_id, cell_id, cell_append_text)


Append the given text and optional formatting to the specified cell

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**notebook_id** | **String** |  | [required] |
**cell_id** | **String** | ID of the notebook cell | [required] |
**cell_append_text** | [**CellAppendText**](CellAppendText.md) | Content and optional formatting to append | [required] |

### Return type

[**crate::models::Cell**](cell.md)

### Authorization

[userToken](../README.md#userToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## notebook_cell_replace_text

> crate::models::Cell notebook_cell_replace_text(notebook_id, cell_id, cell_replace_text)


Replace some text and formatting in the specified cell

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**notebook_id** | **String** |  | [required] |
**cell_id** | **String** | ID of the notebook cell | [required] |
**cell_replace_text** | [**CellReplaceText**](CellReplaceText.md) |  | [required] |

### Return type

[**crate::models::Cell**](cell.md)

### Authorization

[userToken](../README.md#userToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## notebook_cells_append

> Vec<crate::models::Cell> notebook_cells_append(notebook_id, cell)


Append the given cells to the notebook

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**notebook_id** | **String** |  | [required] |
**cell** | [**Vec<crate::models::Cell>**](cell.md) | Cells to append | [required] |

### Return type

[**Vec<crate::models::Cell>**](cell.md)

### Authorization

[userToken](../README.md#userToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## notebook_convert_to_template

> String notebook_convert_to_template(notebook_id)


Convert the notebook to a Template

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**notebook_id** | **String** |  | [required] |

### Return type

**String**

### Authorization

[userToken](../README.md#userToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## notebook_create

> crate::models::Notebook notebook_create(workspace_id, new_notebook)


Create a new notebook

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace_id** | **String** | The workspace ID | [required] |
**new_notebook** | [**NewNotebook**](NewNotebook.md) | new notebook | [required] |

### Return type

[**crate::models::Notebook**](notebook.md)

### Authorization

[userToken](../README.md#userToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## notebook_delete

> notebook_delete(notebook_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**notebook_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[userToken](../README.md#userToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## notebook_get

> crate::models::Notebook notebook_get(notebook_id)


Fetch a single notebook

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**notebook_id** | **String** |  | [required] |

### Return type

[**crate::models::Notebook**](notebook.md)

### Authorization

[userToken](../README.md#userToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## notebook_list

> Vec<crate::models::NotebookSummary> notebook_list(workspace_id)


List all accessible notebooks

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace_id** | **String** | The workspace ID | [required] |

### Return type

[**Vec<crate::models::NotebookSummary>**](notebookSummary.md)

### Authorization

[userToken](../README.md#userToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## notebook_search

> Vec<crate::models::NotebookSummary> notebook_search(workspace_id, notebook_search)


Search for notebooks

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace_id** | **String** | The workspace ID | [required] |
**notebook_search** | [**NotebookSearch**](NotebookSearch.md) | Notebook search payload | [required] |

### Return type

[**Vec<crate::models::NotebookSummary>**](notebookSummary.md)

### Authorization

[userToken](../README.md#userToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## notebook_update

> notebook_update(notebook_id, notebook_patch)


Modifies individual properties of a single notebook

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**notebook_id** | **String** |  | [required] |
**notebook_patch** | [**NotebookPatch**](NotebookPatch.md) | updated properties | [required] |

### Return type

 (empty response body)

### Authorization

[userToken](../README.md#userToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## oidc_authorize_google

> oidc_authorize_google(cli_redirect_port, redirect)


Start the Google OAuth flow to authenticate a user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cli_redirect_port** | Option<**i32**> | The port on localhost to redirect to after the OAuth flow is successful. Used for authorizing the CLI |  |
**redirect** | Option<**String**> | Relative path to redirect to after the OAuth flow is successful. Used for deep linking into the Studio |  |

### Return type

 (empty response body)

### Authorization

[userToken](../README.md#userToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## pinned_notebook_create

> pinned_notebook_create(new_pinned_notebook)


Pin a notebook

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**new_pinned_notebook** | [**NewPinnedNotebook**](NewPinnedNotebook.md) | new notebook | [required] |

### Return type

 (empty response body)

### Authorization

[userToken](../README.md#userToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## pinned_notebook_delete

> pinned_notebook_delete(notebook_id)


Unpin a notebook

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**notebook_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[userToken](../README.md#userToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## pinned_notebook_list

> Vec<crate::models::NotebookSummary> pinned_notebook_list(workspace_id)


List all pinned notebooks for a specific workspace

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace_id** | **String** | The workspace ID | [required] |

### Return type

[**Vec<crate::models::NotebookSummary>**](notebookSummary.md)

### Authorization

[userToken](../README.md#userToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## profile_get

> crate::models::Profile profile_get()


Fetch the profile of the authenticated user

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::Profile**](profile.md)

### Authorization

[userToken](../README.md#userToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## profile_picture_get

> std::path::PathBuf profile_picture_get()


Retrieve profile image

### Parameters

This endpoint does not need any parameter.

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: image/_*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## profile_picture_update

> profile_picture_update(picture)


Upload profile image

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**picture** | **std::path::PathBuf** |  | [required] |

### Return type

 (empty response body)

### Authorization

[userToken](../README.md#userToken)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## proxy_create

> crate::models::Proxy proxy_create(workspace_id, new_proxy)


Create a new proxy

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace_id** | **String** | The workspace ID | [required] |
**new_proxy** | [**NewProxy**](NewProxy.md) | new proxy | [required] |

### Return type

[**crate::models::Proxy**](proxy.md)

### Authorization

[userToken](../README.md#userToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## proxy_delete

> proxy_delete(workspace_id, proxy_name)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace_id** | **String** | The workspace ID | [required] |
**proxy_name** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[userToken](../README.md#userToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## proxy_get

> crate::models::Proxy proxy_get(workspace_id, proxy_name)


Retrieve a single proxy

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace_id** | **String** | The workspace ID | [required] |
**proxy_name** | **String** |  | [required] |

### Return type

[**crate::models::Proxy**](proxy.md)

### Authorization

[userToken](../README.md#userToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## proxy_list

> Vec<crate::models::ProxySummary> proxy_list(workspace_id)


List all proxies

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace_id** | **String** | The workspace ID | [required] |

### Return type

[**Vec<crate::models::ProxySummary>**](proxySummary.md)

### Authorization

[userToken](../README.md#userToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## proxy_relay

> std::path::PathBuf proxy_relay(workspace_id, proxy_name, data_source_name, provider_protocol_version, body)


Relay a query to a remote proxy

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace_id** | **String** | The workspace ID | [required] |
**proxy_name** | **String** |  | [required] |
**data_source_name** | **String** |  | [required] |
**provider_protocol_version** | **String** |  | [required] |
**body** | **std::path::PathBuf** | Message to send to the proxy | [required] |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[userToken](../README.md#userToken)

### HTTP request headers

- **Content-Type**: application/octet-stream
- **Accept**: application/octet-stream

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## template_create

> crate::models::Template template_create(workspace_id, new_template)


Create a new template

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace_id** | **String** | The workspace ID | [required] |
**new_template** | [**NewTemplate**](NewTemplate.md) |  | [required] |

### Return type

[**crate::models::Template**](template.md)

### Authorization

[userToken](../README.md#userToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## template_delete

> template_delete(workspace_id, template_name)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace_id** | **String** | The workspace ID | [required] |
**template_name** | **String** | Name of the template | [required] |

### Return type

 (empty response body)

### Authorization

[userToken](../README.md#userToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## template_expand

> crate::models::Notebook template_expand(workspace_id, template_name, body)


Expand the template into a notebook

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace_id** | **String** | The workspace ID | [required] |
**template_name** | **String** | Name of the template | [required] |
**body** | Option<**serde_json::Value**> | Parameters to pass to the template |  |

### Return type

[**crate::models::Notebook**](notebook.md)

### Authorization

[userToken](../README.md#userToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## template_get

> crate::models::Template template_get(workspace_id, template_name)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace_id** | **String** | The workspace ID | [required] |
**template_name** | **String** | Name of the template | [required] |

### Return type

[**crate::models::Template**](template.md)

### Authorization

[userToken](../README.md#userToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## template_list

> Vec<crate::models::TemplateSummary> template_list(workspace_id, sort_by, sort_direction)


List the templates that have been uploaded

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace_id** | **String** | The workspace ID | [required] |
**sort_by** | Option<**String**> | Sort the resulting list by the following field (defaults to title) |  |
**sort_direction** | Option<**String**> | Sort the resulting list in the following direction (defaults to ascending) |  |

### Return type

[**Vec<crate::models::TemplateSummary>**](templateSummary.md)

### Authorization

[userToken](../README.md#userToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## template_update

> crate::models::Template template_update(workspace_id, template_name, update_template)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace_id** | **String** | The workspace ID | [required] |
**template_name** | **String** | Name of the template | [required] |
**update_template** | [**UpdateTemplate**](UpdateTemplate.md) |  | [required] |

### Return type

[**crate::models::Template**](template.md)

### Authorization

[userToken](../README.md#userToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## thread_create

> crate::models::Thread thread_create(notebook_id, new_thread)


Create a new comment thread

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**notebook_id** | **String** |  | [required] |
**new_thread** | [**NewThread**](NewThread.md) |  | [required] |

### Return type

[**crate::models::Thread**](thread.md)

### Authorization

[userToken](../README.md#userToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## thread_delete

> thread_delete(thread_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**thread_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[userToken](../README.md#userToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## thread_get

> crate::models::Thread thread_get(thread_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**thread_id** | **String** |  | [required] |

### Return type

[**crate::models::Thread**](thread.md)

### Authorization

[userToken](../README.md#userToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## thread_reopen

> crate::models::Thread thread_reopen(thread_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**thread_id** | **String** |  | [required] |

### Return type

[**crate::models::Thread**](thread.md)

### Authorization

[userToken](../README.md#userToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## thread_resolve

> crate::models::Thread thread_resolve(thread_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**thread_id** | **String** |  | [required] |

### Return type

[**crate::models::Thread**](thread.md)

### Authorization

[userToken](../README.md#userToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## threads_list

> Vec<crate::models::ThreadSummary> threads_list(notebook_id, status)


List the threads in the given notebook

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**notebook_id** | **String** |  | [required] |
**status** | Option<[**crate::models::ThreadStatus**](.md)> |  |  |

### Return type

[**Vec<crate::models::ThreadSummary>**](threadSummary.md)

### Authorization

[userToken](../README.md#userToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## token_create

> crate::models::Token token_create(new_token)


Creates a new token

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**new_token** | [**NewToken**](NewToken.md) | Token creation payload | [required] |

### Return type

[**crate::models::Token**](token.md)

### Authorization

[userToken](../README.md#userToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## token_delete

> token_delete(id)


Deletes a token

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID of the token which should be deleted  # Deleting current token  If you want to delete the token with which you sent the request, call `/api/logout` instead.  | [required] |

### Return type

 (empty response body)

### Authorization

[userToken](../README.md#userToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## token_list

> Vec<crate::models::TokenSummary> token_list(sort_by, sort_direction, page, limit)


Gets a list of api tokens

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sort_by** | Option<**String**> | Sort the resulting list by the following field (defaults to title) |  |
**sort_direction** | Option<**String**> | Sort the resulting list in the following direction (defaults to ascending) |  |
**page** | Option<**i32**> | Page number which should be displayed; 0-indexed (defaults to 0) |  |
**limit** | Option<**i32**> | Maximum amount of results to display per page (defaults to 20) |  |

### Return type

[**Vec<crate::models::TokenSummary>**](tokenSummary.md)

### Authorization

[userToken](../README.md#userToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## trigger_create

> crate::models::Trigger trigger_create(workspace_id, new_trigger)


Create a new trigger

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace_id** | **String** | The workspace ID | [required] |
**new_trigger** | [**NewTrigger**](NewTrigger.md) | Template URL or body | [required] |

### Return type

[**crate::models::Trigger**](trigger.md)

### Authorization

[userToken](../README.md#userToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## trigger_delete

> trigger_delete(trigger_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**trigger_id** | **String** | ID of the trigger | [required] |

### Return type

 (empty response body)

### Authorization

[userToken](../README.md#userToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## trigger_get

> crate::models::Trigger trigger_get(trigger_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**trigger_id** | **String** | ID of the trigger | [required] |

### Return type

[**crate::models::Trigger**](trigger.md)

### Authorization

[userToken](../README.md#userToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## trigger_invoke

> crate::models::TriggerInvokeResponse trigger_invoke(trigger_id, secret_key, body)


Invoke a trigger to create a notebook from the associated template

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**trigger_id** | **String** | ID of the trigger | [required] |
**secret_key** | **String** | Secret key of the trigger (included in the response when the trigger is first created) | [required] |
**body** | Option<**serde_json::Value**> | Parameters to pass to the template |  |

### Return type

[**crate::models::TriggerInvokeResponse**](triggerInvokeResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## trigger_list

> Vec<crate::models::Trigger> trigger_list(workspace_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace_id** | **String** | The workspace ID | [required] |

### Return type

[**Vec<crate::models::Trigger>**](trigger.md)

### Authorization

[userToken](../README.md#userToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## workspace_create

> crate::models::Workspace workspace_create(new_workspace)


Create a new workspace

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**new_workspace** | [**NewWorkspace**](NewWorkspace.md) | Workspace creation payload | [required] |

### Return type

[**crate::models::Workspace**](workspace.md)

### Authorization

[userToken](../README.md#userToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## workspace_delete

> workspace_delete(workspace_id)


Delete workspace

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace_id** | **String** | The workspace ID | [required] |

### Return type

 (empty response body)

### Authorization

[userToken](../README.md#userToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## workspace_get

> crate::models::Workspace workspace_get(workspace_id)


Get the workspace details

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace_id** | **String** | The workspace ID | [required] |

### Return type

[**crate::models::Workspace**](workspace.md)

### Authorization

[userToken](../README.md#userToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## workspace_invite

> crate::models::WorkspaceInviteResponse workspace_invite(workspace_id, new_workspace_invite)


Invite a user to a workspace

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace_id** | **String** | The workspace ID | [required] |
**new_workspace_invite** | [**NewWorkspaceInvite**](NewWorkspaceInvite.md) | Workspace invitation payload | [required] |

### Return type

[**crate::models::WorkspaceInviteResponse**](workspace_invite_response.md)

### Authorization

[userToken](../README.md#userToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## workspace_invite_accept

> crate::models::Workspace workspace_invite_accept(invitation_id, invitation_secret)


Accept the workspace invitation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**invitation_id** | **String** | ID of the invitation for which an action should be executed for | [required] |
**invitation_secret** | **String** | Secret key used to verify that the route belongs to a specific email address | [required] |

### Return type

[**crate::models::Workspace**](workspace.md)

### Authorization

[userToken](../README.md#userToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## workspace_invite_decline

> workspace_invite_decline(invitation_id, invitation_secret)


Decline the workspace invitation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**invitation_id** | **String** | ID of the invitation for which an action should be executed for | [required] |
**invitation_secret** | **String** | Secret key used to verify that the route belongs to a specific email address | [required] |

### Return type

 (empty response body)

### Authorization

[userToken](../README.md#userToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## workspace_invite_delete

> workspace_invite_delete(invitation_id)


Delete a pending workspace invitation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**invitation_id** | **String** | ID of the invitation for which an action should be executed for | [required] |

### Return type

 (empty response body)

### Authorization

[userToken](../README.md#userToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## workspace_invite_get

> Vec<crate::models::WorkspaceInvite> workspace_invite_get(workspace_id, sort_by, sort_direction, page, limit)


Get a list of pending workspace invitations

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace_id** | **String** | The workspace ID | [required] |
**sort_by** | Option<**String**> | Sort the resulting list by the following field (defaults to id) |  |
**sort_direction** | Option<**String**> | Sort the resulting list in the following direction (defaults to ascending) |  |
**page** | Option<**i32**> | Page number which should be displayed; 0-indexed (defaults to 0) |  |
**limit** | Option<**i32**> | Maximum amount of results to display per page (defaults to 20) |  |

### Return type

[**Vec<crate::models::WorkspaceInvite>**](workspace_invite.md)

### Authorization

[userToken](../README.md#userToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## workspace_leave

> workspace_leave(workspace_id)


Leave a workspace

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace_id** | **String** | The workspace ID | [required] |

### Return type

 (empty response body)

### Authorization

[userToken](../README.md#userToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## workspace_list

> Vec<crate::models::Workspace> workspace_list(sort_by, sort_direction)


List all workspaces authenticated user has access to

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sort_by** | Option<**String**> | Sort the resulting list by the following field (defaults to id) |  |
**sort_direction** | Option<**String**> | Sort the resulting list in the following direction (defaults to ascending) |  |

### Return type

[**Vec<crate::models::Workspace>**](workspace.md)

### Authorization

[userToken](../README.md#userToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## workspace_picture_get

> std::path::PathBuf workspace_picture_get(workspace_id)


Retrieve workspace image

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace_id** | **String** | The workspace ID | [required] |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: image/_*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## workspace_picture_update

> workspace_picture_update(workspace_id, picture)


Upload workspace image

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace_id** | **String** | The workspace ID | [required] |
**picture** | **std::path::PathBuf** |  | [required] |

### Return type

 (empty response body)

### Authorization

[userToken](../README.md#userToken)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## workspace_update

> crate::models::Workspace workspace_update(workspace_id, update_workspace)


Update workspace settings

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace_id** | **String** | The workspace ID | [required] |
**update_workspace** | [**UpdateWorkspace**](UpdateWorkspace.md) | Workspace update payload | [required] |

### Return type

[**crate::models::Workspace**](workspace.md)

### Authorization

[userToken](../README.md#userToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## workspace_user_remove

> workspace_user_remove(workspace_id, user_id)


Remove a user from the workspace

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace_id** | **String** | The workspace ID | [required] |
**user_id** | **String** | User ID | [required] |

### Return type

 (empty response body)

### Authorization

[userToken](../README.md#userToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## workspace_user_update

> crate::models::User workspace_user_update(workspace_id, user_id, workspace_user_update)


Update the user within a workspace

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace_id** | **String** | The workspace ID | [required] |
**user_id** | **String** | User ID | [required] |
**workspace_user_update** | [**WorkspaceUserUpdate**](WorkspaceUserUpdate.md) | Workspace user update payload | [required] |

### Return type

[**crate::models::User**](user.md)

### Authorization

[userToken](../README.md#userToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## workspace_users_list

> Vec<crate::models::User> workspace_users_list(workspace_id, sort_by, sort_direction)


List all users for a workspace

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace_id** | **String** | The workspace ID | [required] |
**sort_by** | Option<**String**> | Sort the resulting list by the following field (defaults to id) |  |
**sort_direction** | Option<**String**> | Sort the resulting list in the following direction (defaults to ascending) |  |

### Return type

[**Vec<crate::models::User>**](user.md)

### Authorization

[userToken](../README.md#userToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

