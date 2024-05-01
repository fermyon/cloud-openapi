# \DeviceCodesApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_device_codes_activate_post**](DeviceCodesApi.md#api_device_codes_activate_post) | **POST** /api/device-codes/activate | 
[**api_device_codes_post**](DeviceCodesApi.md#api_device_codes_post) | **POST** /api/device-codes | 
[**api_device_codes_user_code_get**](DeviceCodesApi.md#api_device_codes_user_code_get) | **GET** /api/device-codes/{userCode} | 



## api_device_codes_activate_post

> api_device_codes_activate_post(activate_device_code_command, api_version)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**activate_device_code_command** | [**ActivateDeviceCodeCommand**](ActivateDeviceCodeCommand.md) |  | [required] |
**api_version** | Option<**String**> | The requested API version |  |[default to 1.0]

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/json, text/json, application/*+json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_device_codes_post

> models::DeviceCodeItem api_device_codes_post(create_device_code_command, api_version)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_device_code_command** | [**CreateDeviceCodeCommand**](CreateDeviceCodeCommand.md) |  | [required] |
**api_version** | Option<**String**> | The requested API version |  |[default to 1.0]

### Return type

[**models::DeviceCodeItem**](DeviceCodeItem.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/json, text/json, application/*+json
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_device_codes_user_code_get

> models::DeviceCodeDetails api_device_codes_user_code_get(user_code, api_version)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_code** | **String** |  | [required] |
**api_version** | Option<**String**> | The requested API version |  |[default to 1.0]

### Return type

[**models::DeviceCodeDetails**](DeviceCodeDetails.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

