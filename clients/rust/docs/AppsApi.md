# \AppsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_apps_get**](AppsApi.md#api_apps_get) | **GET** /api/apps | 
[**api_apps_id_delete**](AppsApi.md#api_apps_id_delete) | **DELETE** /api/apps/{id} | 
[**api_apps_id_events_get**](AppsApi.md#api_apps_id_events_get) | **GET** /api/apps/{id}/events | 
[**api_apps_id_get**](AppsApi.md#api_apps_id_get) | **GET** /api/apps/{id} | 
[**api_apps_id_logs_get**](AppsApi.md#api_apps_id_logs_get) | **GET** /api/apps/{id}/logs | 
[**api_apps_id_logs_raw_get**](AppsApi.md#api_apps_id_logs_raw_get) | **GET** /api/apps/{id}/logs/raw | 
[**api_apps_id_patch**](AppsApi.md#api_apps_id_patch) | **PATCH** /api/apps/{id} | 
[**api_apps_id_put**](AppsApi.md#api_apps_id_put) | **PUT** /api/apps/{id} | 
[**api_apps_id_request_count_get**](AppsApi.md#api_apps_id_request_count_get) | **GET** /api/apps/{id}/request-count | 
[**api_apps_post**](AppsApi.md#api_apps_post) | **POST** /api/apps | 



## api_apps_get

> crate::models::AppItemPage api_apps_get(search_text, page_index, page_size, sort_by, is_sorted_ascending, exact_match, api_version)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**search_text** | Option<**String**> |  |  |[default to ]
**page_index** | Option<**i32**> |  |  |[default to 0]
**page_size** | Option<**i32**> |  |  |[default to 50]
**sort_by** | Option<**String**> |  |  |[default to Name]
**is_sorted_ascending** | Option<**bool**> |  |  |[default to true]
**exact_match** | Option<**bool**> |  |  |[default to false]
**api_version** | Option<**String**> | The requested API version |  |[default to 1.0]

### Return type

[**crate::models::AppItemPage**](AppItemPage.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_apps_id_delete

> api_apps_id_delete(id, api_version)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**api_version** | Option<**String**> | The requested API version |  |[default to 1.0]

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_apps_id_events_get

> Vec<crate::models::AppEventItem> api_apps_id_events_get(id, from, to, api_version)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**from** | Option<**String**> |  |  |
**to** | Option<**String**> |  |  |
**api_version** | Option<**String**> | The requested API version |  |[default to 1.0]

### Return type

[**Vec<crate::models::AppEventItem>**](AppEventItem.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_apps_id_get

> crate::models::AppItem api_apps_id_get(id, api_version)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**api_version** | Option<**String**> | The requested API version |  |[default to 1.0]

### Return type

[**crate::models::AppItem**](AppItem.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_apps_id_logs_get

> crate::models::GetAppLogsVm api_apps_id_logs_get(id, max, since, api_version)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**max** | Option<**i32**> |  |  |
**since** | Option<**String**> |  |  |
**api_version** | Option<**String**> | The requested API version |  |[default to 1.0]

### Return type

[**crate::models::GetAppLogsVm**](GetAppLogsVm.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_apps_id_logs_raw_get

> crate::models::GetAppRawLogsVm api_apps_id_logs_raw_get(id, max, since, api_version)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**max** | Option<**i32**> |  |  |
**since** | Option<**String**> |  |  |
**api_version** | Option<**String**> | The requested API version |  |[default to 1.0]

### Return type

[**crate::models::GetAppRawLogsVm**](GetAppRawLogsVm.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_apps_id_patch

> api_apps_id_patch(id, patch_app_command, api_version)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**patch_app_command** | [**PatchAppCommand**](PatchAppCommand.md) |  | [required] |
**api_version** | Option<**String**> | The requested API version |  |[default to 1.0]

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/json, text/json, application/*+json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_apps_id_put

> api_apps_id_put(id, update_app_command, api_version)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**update_app_command** | [**UpdateAppCommand**](UpdateAppCommand.md) |  | [required] |
**api_version** | Option<**String**> | The requested API version |  |[default to 1.0]

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/json, text/json, application/*+json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_apps_id_request_count_get

> crate::models::AppRequestCountItem api_apps_id_request_count_get(id, from, to, api_version)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**from** | Option<**String**> |  |  |
**to** | Option<**String**> |  |  |
**api_version** | Option<**String**> | The requested API version |  |[default to 1.0]

### Return type

[**crate::models::AppRequestCountItem**](AppRequestCountItem.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_apps_post

> uuid::Uuid api_apps_post(create_app_command, api_version)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_app_command** | [**CreateAppCommand**](CreateAppCommand.md) |  | [required] |
**api_version** | Option<**String**> | The requested API version |  |[default to 1.0]

### Return type

[**uuid::Uuid**](uuid::Uuid.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/json, text/json, application/*+json
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

