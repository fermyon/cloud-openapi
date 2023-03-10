# \PersonalAccessTokensApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_personal_access_tokens_get**](PersonalAccessTokensApi.md#api_personal_access_tokens_get) | **GET** /api/personal-access-tokens | 
[**api_personal_access_tokens_id_delete**](PersonalAccessTokensApi.md#api_personal_access_tokens_id_delete) | **DELETE** /api/personal-access-tokens/{id} | 
[**api_personal_access_tokens_post**](PersonalAccessTokensApi.md#api_personal_access_tokens_post) | **POST** /api/personal-access-tokens | 



## api_personal_access_tokens_get

> crate::models::PersonalAccessTokenItemPage api_personal_access_tokens_get(search_text, page_index, page_size, sort_by, is_sorted_ascending, api_version)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**search_text** | Option<**String**> |  |  |[default to ]
**page_index** | Option<**i32**> |  |  |[default to 0]
**page_size** | Option<**i32**> |  |  |[default to 50]
**sort_by** | Option<**String**> |  |  |[default to CreatedAt]
**is_sorted_ascending** | Option<**bool**> |  |  |[default to false]
**api_version** | Option<**String**> | The requested API version |  |[default to 1.0]

### Return type

[**crate::models::PersonalAccessTokenItemPage**](PersonalAccessTokenItemPage.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_personal_access_tokens_id_delete

> api_personal_access_tokens_id_delete(id, api_version)


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


## api_personal_access_tokens_post

> crate::models::PersonalAccessTokenValue api_personal_access_tokens_post(create_personal_access_token_command, api_version)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_personal_access_token_command** | [**CreatePersonalAccessTokenCommand**](CreatePersonalAccessTokenCommand.md) |  | [required] |
**api_version** | Option<**String**> | The requested API version |  |[default to 1.0]

### Return type

[**crate::models::PersonalAccessTokenValue**](PersonalAccessTokenValue.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/json, text/json, application/*+json
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

