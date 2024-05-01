# \AccountsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_accounts_id_delete**](AccountsApi.md#api_accounts_id_delete) | **DELETE** /api/accounts/{id} | 
[**api_accounts_id_get**](AccountsApi.md#api_accounts_id_get) | **GET** /api/accounts/{id} | 
[**api_accounts_id_patch**](AccountsApi.md#api_accounts_id_patch) | **PATCH** /api/accounts/{id} | 



## api_accounts_id_delete

> api_accounts_id_delete(id, api_version)


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


## api_accounts_id_get

> models::AccountDetails api_accounts_id_get(id, api_version)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**api_version** | Option<**String**> | The requested API version |  |[default to 1.0]

### Return type

[**models::AccountDetails**](AccountDetails.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_accounts_id_patch

> api_accounts_id_patch(id, patch_account_details_command, api_version)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**patch_account_details_command** | [**PatchAccountDetailsCommand**](PatchAccountDetailsCommand.md) |  | [required] |
**api_version** | Option<**String**> | The requested API version |  |[default to 1.0]

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/json, text/json, application/*+json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

