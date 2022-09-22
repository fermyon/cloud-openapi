# \DeviceCodesApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_device_codes_activate_post**](DeviceCodesApi.md#api_device_codes_activate_post) | **POST** /api/device-codes/activate | 
[**api_device_codes_post**](DeviceCodesApi.md#api_device_codes_post) | **POST** /api/device-codes | 
[**api_device_codes_user_code_get**](DeviceCodesApi.md#api_device_codes_user_code_get) | **GET** /api/device-codes/{userCode} | 



## api_device_codes_activate_post

> api_device_codes_activate_post(activate_device_code_command)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**activate_device_code_command** | Option<[**ActivateDeviceCodeCommand**](ActivateDeviceCodeCommand.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/json, text/json, application/*+json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_device_codes_post

> crate::models::DeviceCodeItem api_device_codes_post(create_device_code_command)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_device_code_command** | Option<[**CreateDeviceCodeCommand**](CreateDeviceCodeCommand.md)> |  |  |

### Return type

[**crate::models::DeviceCodeItem**](DeviceCodeItem.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/json, text/json, application/*+json
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_device_codes_user_code_get

> crate::models::DeviceCodeDetails api_device_codes_user_code_get(user_code)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_code** | **String** |  | [required] |

### Return type

[**crate::models::DeviceCodeDetails**](DeviceCodeDetails.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

