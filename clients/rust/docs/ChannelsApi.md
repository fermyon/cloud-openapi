# \ChannelsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_channels_channel_id_desired_status_put**](ChannelsApi.md#api_channels_channel_id_desired_status_put) | **PUT** /api/channels/{channelId}/desired-status | 
[**api_channels_get**](ChannelsApi.md#api_channels_get) | **GET** /api/channels | 
[**api_channels_id_delete**](ChannelsApi.md#api_channels_id_delete) | **DELETE** /api/channels/{id} | 
[**api_channels_id_get**](ChannelsApi.md#api_channels_id_get) | **GET** /api/channels/{id} | 
[**api_channels_id_healthz_get**](ChannelsApi.md#api_channels_id_healthz_get) | **GET** /api/channels/{id}/healthz | 
[**api_channels_id_logs_get**](ChannelsApi.md#api_channels_id_logs_get) | **GET** /api/channels/{id}/logs | 
[**api_channels_id_logs_raw_get**](ChannelsApi.md#api_channels_id_logs_raw_get) | **GET** /api/channels/{id}/logs/raw | 
[**api_channels_id_patch**](ChannelsApi.md#api_channels_id_patch) | **PATCH** /api/channels/{id} | 
[**api_channels_id_put**](ChannelsApi.md#api_channels_id_put) | **PUT** /api/channels/{id} | 
[**api_channels_post**](ChannelsApi.md#api_channels_post) | **POST** /api/channels | 



## api_channels_channel_id_desired_status_put

> api_channels_channel_id_desired_status_put(channel_id, update_desired_status_command, api_version)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **uuid::Uuid** |  | [required] |
**update_desired_status_command** | [**UpdateDesiredStatusCommand**](UpdateDesiredStatusCommand.md) |  | [required] |
**api_version** | Option<**String**> | The requested API version |  |[default to 1.0]

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/json, text/json, application/*+json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_channels_get

> crate::models::ChannelItemPage api_channels_get(search_text, page_index, page_size, sort_by, is_sorted_ascending, api_version)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**search_text** | Option<**String**> |  |  |[default to ]
**page_index** | Option<**i32**> |  |  |[default to 0]
**page_size** | Option<**i32**> |  |  |[default to 50]
**sort_by** | Option<**String**> |  |  |[default to Name]
**is_sorted_ascending** | Option<**bool**> |  |  |[default to true]
**api_version** | Option<**String**> | The requested API version |  |[default to 1.0]

### Return type

[**crate::models::ChannelItemPage**](ChannelItemPage.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_channels_id_delete

> api_channels_id_delete(id, api_version)


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


## api_channels_id_get

> crate::models::ChannelItem api_channels_id_get(id, api_version)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**api_version** | Option<**String**> | The requested API version |  |[default to 1.0]

### Return type

[**crate::models::ChannelItem**](ChannelItem.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_channels_id_healthz_get

> crate::models::HealthCheckResult api_channels_id_healthz_get(id, api_version)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**api_version** | Option<**String**> | The requested API version |  |[default to 1.0]

### Return type

[**crate::models::HealthCheckResult**](HealthCheckResult.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_channels_id_logs_get

> crate::models::GetChannelLogsVm api_channels_id_logs_get(id, max, since, api_version)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**max** | Option<**i32**> |  |  |
**since** | Option<**String**> |  |  |
**api_version** | Option<**String**> | The requested API version |  |[default to 1.0]

### Return type

[**crate::models::GetChannelLogsVm**](GetChannelLogsVm.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_channels_id_logs_raw_get

> crate::models::GetChannelRawLogsVm api_channels_id_logs_raw_get(id, max, since, api_version)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**max** | Option<**i32**> |  |  |
**since** | Option<**String**> |  |  |
**api_version** | Option<**String**> | The requested API version |  |[default to 1.0]

### Return type

[**crate::models::GetChannelRawLogsVm**](GetChannelRawLogsVm.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_channels_id_patch

> api_channels_id_patch(id, patch_channel_command, api_version)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**patch_channel_command** | [**PatchChannelCommand**](PatchChannelCommand.md) |  | [required] |
**api_version** | Option<**String**> | The requested API version |  |[default to 1.0]

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/json, text/json, application/*+json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_channels_id_put

> api_channels_id_put(id, update_channel_command, api_version)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | [required] |
**update_channel_command** | [**UpdateChannelCommand**](UpdateChannelCommand.md) |  | [required] |
**api_version** | Option<**String**> | The requested API version |  |[default to 1.0]

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/json, text/json, application/*+json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_channels_post

> uuid::Uuid api_channels_post(create_channel_command, api_version)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_channel_command** | [**CreateChannelCommand**](CreateChannelCommand.md) |  | [required] |
**api_version** | Option<**String**> | The requested API version |  |[default to 1.0]

### Return type

[**uuid::Uuid**](uuid::Uuid.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/json, text/json, application/*+json
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

