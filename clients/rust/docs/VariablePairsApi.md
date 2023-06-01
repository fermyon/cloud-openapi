# \VariablePairsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_variable_pairs_create_post**](VariablePairsApi.md#api_variable_pairs_create_post) | **POST** /api/variable-pairs/create | 
[**api_variable_pairs_delete**](VariablePairsApi.md#api_variable_pairs_delete) | **DELETE** /api/variable-pairs | 
[**api_variable_pairs_list_get**](VariablePairsApi.md#api_variable_pairs_list_get) | **GET** /api/variable-pairs/list | 



## api_variable_pairs_create_post

> api_variable_pairs_create_post(create_variable_pair_command, api_version)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_variable_pair_command** | [**CreateVariablePairCommand**](CreateVariablePairCommand.md) |  | [required] |
**api_version** | Option<**String**> | The requested API version |  |[default to 1.0]

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/json, text/json, application/*+json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_variable_pairs_delete

> api_variable_pairs_delete(delete_variable_pair_command, api_version)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**delete_variable_pair_command** | [**DeleteVariablePairCommand**](DeleteVariablePairCommand.md) |  | [required] |
**api_version** | Option<**String**> | The requested API version |  |[default to 1.0]

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/json, text/json, application/*+json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_variable_pairs_list_get

> crate::models::VariablesList api_variable_pairs_list_get(list_variables_query, api_version)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**list_variables_query** | [**ListVariablesQuery**](ListVariablesQuery.md) |  | [required] |
**api_version** | Option<**String**> | The requested API version |  |[default to 1.0]

### Return type

[**crate::models::VariablesList**](VariablesList.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/json, text/json, application/*+json
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

