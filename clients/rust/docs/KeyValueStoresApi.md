# \KeyValueStoresApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_key_value_stores_get**](KeyValueStoresApi.md#api_key_value_stores_get) | **GET** /api/key-value-stores | 
[**api_key_value_stores_store_delete**](KeyValueStoresApi.md#api_key_value_stores_store_delete) | **DELETE** /api/key-value-stores/{store} | 
[**api_key_value_stores_store_links_delete**](KeyValueStoresApi.md#api_key_value_stores_store_links_delete) | **DELETE** /api/key-value-stores/{store}/links | 
[**api_key_value_stores_store_links_post**](KeyValueStoresApi.md#api_key_value_stores_store_links_post) | **POST** /api/key-value-stores/{store}/links | 
[**api_key_value_stores_store_post**](KeyValueStoresApi.md#api_key_value_stores_store_post) | **POST** /api/key-value-stores/{store} | 
[**api_key_value_stores_store_rename_patch**](KeyValueStoresApi.md#api_key_value_stores_store_rename_patch) | **PATCH** /api/key-value-stores/{store}/rename | 



## api_key_value_stores_get

> models::KeyValueStoresList api_key_value_stores_get(app_id, api_version)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_id** | Option<**uuid::Uuid**> |  |  |
**api_version** | Option<**String**> | The requested API version |  |[default to 1.0]

### Return type

[**models::KeyValueStoresList**](KeyValueStoresList.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_key_value_stores_store_delete

> api_key_value_stores_store_delete(store, api_version)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**store** | **String** |  | [required] |
**api_version** | Option<**String**> | The requested API version |  |[default to 1.0]

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_key_value_stores_store_links_delete

> api_key_value_stores_store_links_delete(store, resource_label, api_version)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**store** | **String** |  | [required] |
**resource_label** | [**ResourceLabel**](ResourceLabel.md) |  | [required] |
**api_version** | Option<**String**> | The requested API version |  |[default to 1.0]

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/json, text/json, application/*+json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_key_value_stores_store_links_post

> api_key_value_stores_store_links_post(store, resource_label, api_version)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**store** | **String** |  | [required] |
**resource_label** | [**ResourceLabel**](ResourceLabel.md) |  | [required] |
**api_version** | Option<**String**> | The requested API version |  |[default to 1.0]

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/json, text/json, application/*+json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_key_value_stores_store_post

> api_key_value_stores_store_post(store, api_version, resource_label)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**store** | **String** |  | [required] |
**api_version** | Option<**String**> | The requested API version |  |[default to 1.0]
**resource_label** | Option<[**ResourceLabel**](ResourceLabel.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/json, text/json, application/*+json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_key_value_stores_store_rename_patch

> api_key_value_stores_store_rename_patch(store, body, api_version)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**store** | **String** |  | [required] |
**body** | **String** |  | [required] |
**api_version** | Option<**String**> | The requested API version |  |[default to 1.0]

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/json, text/json, application/*+json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

