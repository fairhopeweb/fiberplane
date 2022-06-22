# \DefaultApi

All URIs are relative to *https://fiberplane.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**convert_notebook_to_template**](DefaultApi.md#convert_notebook_to_template) | **GET** /api/notebooks/{id}/template.jsonnet | 
[**delete_file**](DefaultApi.md#delete_file) | **DELETE** /api/files/{notebookId}/{fileId} | 
[**delete_notebook**](DefaultApi.md#delete_notebook) | **DELETE** /api/notebooks/{id} | 
[**event_create**](DefaultApi.md#event_create) | **POST** /api/events | 
[**event_delete**](DefaultApi.md#event_delete) | **DELETE** /api/events/{id} | 
[**event_list**](DefaultApi.md#event_list) | **GET** /api/events | 
[**file_upload**](DefaultApi.md#file_upload) | **POST** /api/files/{notebookId} | 
[**get_file**](DefaultApi.md#get_file) | **GET** /api/files/{notebookId}/{fileId} | 
[**get_notebook**](DefaultApi.md#get_notebook) | **GET** /api/notebooks/{id} | 
[**get_profile**](DefaultApi.md#get_profile) | **GET** /api/profile | 
[**get_profile_picture**](DefaultApi.md#get_profile_picture) | **GET** /api/profile/picture | 
[**label_keys_list**](DefaultApi.md#label_keys_list) | **GET** /api/labels/keys | 
[**label_values_list**](DefaultApi.md#label_values_list) | **GET** /api/labels/values/{label_key} | 
[**logout**](DefaultApi.md#logout) | **POST** /api/logout | 
[**notebook_cell_append_text**](DefaultApi.md#notebook_cell_append_text) | **POST** /api/notebooks/{notebookId}/cells/{cellId}/append | 
[**notebook_cell_replace_text**](DefaultApi.md#notebook_cell_replace_text) | **POST** /api/notebooks/{notebookId}/cells/{cellId}/replaceText | 
[**notebook_cells_append**](DefaultApi.md#notebook_cells_append) | **POST** /api/notebooks/{id}/cells | 
[**notebook_create**](DefaultApi.md#notebook_create) | **POST** /api/notebooks | 
[**notebook_list**](DefaultApi.md#notebook_list) | **GET** /api/notebooks | 
[**notebook_search**](DefaultApi.md#notebook_search) | **POST** /api/search/notebooks | 
[**oidc_authorize_google**](DefaultApi.md#oidc_authorize_google) | **GET** /api/oidc/authorize/google | 
[**org_data_source_create**](DefaultApi.md#org_data_source_create) | **POST** /api/datasources | 
[**org_user_list_current**](DefaultApi.md#org_user_list_current) | **GET** /api/organizations/current/users | 
[**patch_notebook**](DefaultApi.md#patch_notebook) | **PATCH** /api/notebooks/{id} | 
[**pinned_notebook_create**](DefaultApi.md#pinned_notebook_create) | **POST** /api/pinnednotebooks | 
[**pinned_notebook_delete**](DefaultApi.md#pinned_notebook_delete) | **DELETE** /api/pinnednotebooks/{notebookId} | 
[**pinned_notebook_list**](DefaultApi.md#pinned_notebook_list) | **GET** /api/pinnednotebooks | 
[**proxy_create**](DefaultApi.md#proxy_create) | **POST** /api/proxies | 
[**proxy_data_sources_list**](DefaultApi.md#proxy_data_sources_list) | **GET** /api/proxies/datasources | 
[**proxy_delete**](DefaultApi.md#proxy_delete) | **DELETE** /api/proxies/{proxyId} | 
[**proxy_get**](DefaultApi.md#proxy_get) | **GET** /api/proxies/{proxyId} | 
[**proxy_list**](DefaultApi.md#proxy_list) | **GET** /api/proxies | 
[**proxy_relay**](DefaultApi.md#proxy_relay) | **POST** /api/proxies/{proxyId}/relay | 
[**template_create**](DefaultApi.md#template_create) | **POST** /api/templates | 
[**template_delete**](DefaultApi.md#template_delete) | **DELETE** /api/templates/{templateId} | 
[**template_example_expand**](DefaultApi.md#template_example_expand) | **POST** /api/templates/examples/{templateId}/expand | 
[**template_example_list**](DefaultApi.md#template_example_list) | **GET** /api/templates/examples | 
[**template_expand**](DefaultApi.md#template_expand) | **POST** /api/templates/{templateId}/expand | 
[**template_get**](DefaultApi.md#template_get) | **GET** /api/templates/{templateId} | 
[**template_list**](DefaultApi.md#template_list) | **GET** /api/templates | 
[**template_update**](DefaultApi.md#template_update) | **PATCH** /api/templates/{templateId} | 
[**trigger_create**](DefaultApi.md#trigger_create) | **POST** /api/triggers | 
[**trigger_delete**](DefaultApi.md#trigger_delete) | **DELETE** /api/triggers/{triggerId} | 
[**trigger_get**](DefaultApi.md#trigger_get) | **GET** /api/triggers/{triggerId} | 
[**trigger_invoke**](DefaultApi.md#trigger_invoke) | **POST** /api/triggers/{triggerId}/{secretKey} | 
[**trigger_list**](DefaultApi.md#trigger_list) | **GET** /api/triggers | 
[**update_profile_picture**](DefaultApi.md#update_profile_picture) | **POST** /api/profile/picture | 



## convert_notebook_to_template

> String convert_notebook_to_template(id)


Convert the notebook to a Template

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID of the notebook | [required] |

### Return type

**String**

### Authorization

[userToken](../README.md#userToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_file

> delete_file(notebook_id, file_id)


Delete a file

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**notebook_id** | **String** | ID of the notebook | [required] |
**file_id** | **String** | ID of the file | [required] |

### Return type

 (empty response body)

### Authorization

[userToken](../README.md#userToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_notebook

> delete_notebook(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID of the notebook | [required] |

### Return type

 (empty response body)

### Authorization

[userToken](../README.md#userToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## event_create

> crate::models::Event event_create(new_event)


Creates a new event

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
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

> event_delete(id)


Deletes a event

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID of the event | [required] |

### Return type

 (empty response body)

### Authorization

[userToken](../README.md#userToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## event_list

> Vec<crate::models::Event> event_list(occurrence_time_start, occurrence_time_end, labels, sort_by, sort_direction, page, limit)


Get a list of all events matching the query

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
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


## file_upload

> crate::models::FileSummary file_upload(notebook_id, file)


upload a file

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**notebook_id** | **String** | ID of the notebook | [required] |
**file** | **std::path::PathBuf** |  | [required] |

### Return type

[**crate::models::FileSummary**](fileSummary.md)

### Authorization

[userToken](../README.md#userToken)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_file

> std::path::PathBuf get_file(notebook_id, file_id)


Get a file

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**notebook_id** | **String** | ID of the notebook | [required] |
**file_id** | **String** | ID of the file | [required] |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: image/_*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_notebook

> crate::models::Notebook get_notebook(id)


Fetch a single notebook

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID of the notebook | [required] |

### Return type

[**crate::models::Notebook**](notebook.md)

### Authorization

[userToken](../README.md#userToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_profile

> crate::models::User get_profile()


Fetch the profile of the authenticated user

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::User**](user.md)

### Authorization

[userToken](../README.md#userToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_profile_picture

> std::path::PathBuf get_profile_picture()


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


## label_keys_list

> Vec<String> label_keys_list(prefix)


Retrieve all label keys

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
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

> Vec<String> label_values_list(label_key, prefix)


Retrieve all label values for a specific key

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
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
**notebook_id** | **String** | ID of the notebook | [required] |
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
**notebook_id** | **String** | ID of the notebook | [required] |
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

> Vec<crate::models::Cell> notebook_cells_append(id, cell)


Append the given cells to the notebook

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID of the notebook | [required] |
**cell** | [**Vec<crate::models::Cell>**](cell.md) | Cells to append | [required] |

### Return type

[**Vec<crate::models::Cell>**](cell.md)

### Authorization

[userToken](../README.md#userToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## notebook_create

> crate::models::Notebook notebook_create(new_notebook)


Create a new notebook

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**new_notebook** | [**NewNotebook**](NewNotebook.md) | new notebook | [required] |

### Return type

[**crate::models::Notebook**](notebook.md)

### Authorization

[userToken](../README.md#userToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## notebook_list

> Vec<crate::models::NotebookSummary> notebook_list()


List all accessible notebooks

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::NotebookSummary>**](notebookSummary.md)

### Authorization

[userToken](../README.md#userToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## notebook_search

> Vec<crate::models::NotebookSummary> notebook_search(notebook_search)


Search for notebooks

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**notebook_search** | [**NotebookSearch**](NotebookSearch.md) | Notebook search payload | [required] |

### Return type

[**Vec<crate::models::NotebookSummary>**](notebookSummary.md)

### Authorization

[userToken](../README.md#userToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

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


## org_data_source_create

> crate::models::OrgDataSource org_data_source_create(new_org_data_source)


Create an organization data-source

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**new_org_data_source** | [**NewOrgDataSource**](NewOrgDataSource.md) | new data-source | [required] |

### Return type

[**crate::models::OrgDataSource**](orgDataSource.md)

### Authorization

[userToken](../README.md#userToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## org_user_list_current

> Vec<crate::models::User> org_user_list_current()


Fetch the users of the current organization

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::User>**](user.md)

### Authorization

[userToken](../README.md#userToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_notebook

> patch_notebook(id, notebook_patch)


Modifies individual properties of a single notebook

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID of the notebook | [required] |
**notebook_patch** | [**NotebookPatch**](NotebookPatch.md) | updated properties | [required] |

### Return type

 (empty response body)

### Authorization

[userToken](../README.md#userToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## pinned_notebook_create

> pinned_notebook_create(new_pinned_notebook)


Create a new notebook

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


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**notebook_id** | **String** | ID of the notebook | [required] |

### Return type

 (empty response body)

### Authorization

[userToken](../README.md#userToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## pinned_notebook_list

> Vec<crate::models::NotebookSummary> pinned_notebook_list()


List all pinned notebooks

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::NotebookSummary>**](notebookSummary.md)

### Authorization

[userToken](../README.md#userToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## proxy_create

> crate::models::Proxy proxy_create(new_proxy)


Create a new proxy

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**new_proxy** | [**NewProxy**](NewProxy.md) | new proxy | [required] |

### Return type

[**crate::models::Proxy**](proxy.md)

### Authorization

[userToken](../README.md#userToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## proxy_data_sources_list

> Vec<crate::models::DataSourceAndProxySummary> proxy_data_sources_list()


Get all of the data sources for all proxies that belong to the same organization as the user

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::DataSourceAndProxySummary>**](dataSourceAndProxySummary.md)

### Authorization

[serviceToken](../README.md#serviceToken), [userToken](../README.md#userToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## proxy_delete

> proxy_delete(proxy_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proxy_id** | **String** | ID of the proxy | [required] |

### Return type

 (empty response body)

### Authorization

[userToken](../README.md#userToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## proxy_get

> crate::models::Proxy proxy_get(proxy_id)


Retrieve a single proxy

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proxy_id** | **String** | ID of the proxy | [required] |

### Return type

[**crate::models::Proxy**](proxy.md)

### Authorization

[userToken](../README.md#userToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## proxy_list

> Vec<crate::models::ProxySummary> proxy_list()


List all proxies

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::ProxySummary>**](proxySummary.md)

### Authorization

[userToken](../README.md#userToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## proxy_relay

> proxy_relay(proxy_id, data_source_name)


Relay a query to a remote proxy

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proxy_id** | **String** | ID of the proxy | [required] |
**data_source_name** | **String** | Name of the data source | [required] |

### Return type

 (empty response body)

### Authorization

[userToken](../README.md#userToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## template_create

> crate::models::Template template_create(new_template)


Create a new template

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
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

> template_delete(template_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**template_id** | **String** | ID of the template | [required] |

### Return type

 (empty response body)

### Authorization

[userToken](../README.md#userToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## template_example_expand

> crate::models::Notebook template_example_expand(template_id, body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**template_id** | **String** | ID of the template | [required] |
**body** | Option<**serde_json::Value**> | Parameters to pass to the template |  |

### Return type

[**crate::models::Notebook**](notebook.md)

### Authorization

[userToken](../README.md#userToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## template_example_list

> Vec<crate::models::Template> template_example_list()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::Template>**](template.md)

### Authorization

[userToken](../README.md#userToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## template_expand

> crate::models::Notebook template_expand(template_id, body)


Expand the template into a notebook

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**template_id** | **String** | ID of the template | [required] |
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

> crate::models::Template template_get(template_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**template_id** | **String** | ID of the template | [required] |

### Return type

[**crate::models::Template**](template.md)

### Authorization

[userToken](../README.md#userToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## template_list

> Vec<crate::models::TemplateSummary> template_list(sort_by, sort_direction)


List the templates that have been uploaded

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
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

> crate::models::Template template_update(template_id, update_template)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**template_id** | **String** | ID of the template | [required] |
**update_template** | [**UpdateTemplate**](UpdateTemplate.md) |  | [required] |

### Return type

[**crate::models::Template**](template.md)

### Authorization

[userToken](../README.md#userToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## trigger_create

> crate::models::Trigger trigger_create(new_trigger)


Create a new trigger

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
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

> Vec<crate::models::Trigger> trigger_list()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::Trigger>**](trigger.md)

### Authorization

[userToken](../README.md#userToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_profile_picture

> update_profile_picture(picture)


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

