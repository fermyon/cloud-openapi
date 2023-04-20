# \OciApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**v2_get**](OciApi.md#v2_get) | **GET** /v2 | 
[**v2_name_blobs_uploads_digest_delete**](OciApi.md#v2_name_blobs_uploads_digest_delete) | **DELETE** /v2/{name}/blobs/uploads/{digest} | 
[**v2_name_blobs_uploads_digest_get**](OciApi.md#v2_name_blobs_uploads_digest_get) | **GET** /v2/{name}/blobs/uploads/{digest} | 
[**v2_name_blobs_uploads_digest_patch**](OciApi.md#v2_name_blobs_uploads_digest_patch) | **PATCH** /v2/{name}/blobs/uploads/{digest} | 
[**v2_name_blobs_uploads_digest_put**](OciApi.md#v2_name_blobs_uploads_digest_put) | **PUT** /v2/{name}/blobs/uploads/{digest} | 
[**v2_name_blobs_uploads_post**](OciApi.md#v2_name_blobs_uploads_post) | **POST** /v2/{name}/blobs/uploads | 
[**v2_name_manifests_reference_head**](OciApi.md#v2_name_manifests_reference_head) | **HEAD** /v2/{name}/manifests/{reference} | 
[**v2_name_manifests_reference_put**](OciApi.md#v2_name_manifests_reference_put) | **PUT** /v2/{name}/manifests/{reference} | 



## v2_get

> v2_get(api_version)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | Option<**String**> | The requested API version |  |[default to 1.0]

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v2_name_blobs_uploads_digest_delete

> v2_name_blobs_uploads_digest_delete(digest, name, api_version)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**digest** | **String** |  | [required] |
**name** | **String** |  | [required] |
**api_version** | Option<**String**> | The requested API version |  |[default to 1.0]

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v2_name_blobs_uploads_digest_get

> v2_name_blobs_uploads_digest_get(digest, name, api_version)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**digest** | **String** |  | [required] |
**name** | **String** |  | [required] |
**api_version** | Option<**String**> | The requested API version |  |[default to 1.0]

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v2_name_blobs_uploads_digest_patch

> v2_name_blobs_uploads_digest_patch(digest, name, api_version)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**digest** | **String** |  | [required] |
**name** | **String** |  | [required] |
**api_version** | Option<**String**> | The requested API version |  |[default to 1.0]

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v2_name_blobs_uploads_digest_put

> v2_name_blobs_uploads_digest_put(digest, name, api_version)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**digest** | **String** |  | [required] |
**name** | **String** |  | [required] |
**api_version** | Option<**String**> | The requested API version |  |[default to 1.0]

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v2_name_blobs_uploads_post

> v2_name_blobs_uploads_post(name, api_version)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |
**api_version** | Option<**String**> | The requested API version |  |[default to 1.0]

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v2_name_manifests_reference_head

> v2_name_manifests_reference_head(name, reference, api_version)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |
**reference** | **String** |  | [required] |
**api_version** | Option<**String**> | The requested API version |  |[default to 1.0]

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v2_name_manifests_reference_put

> v2_name_manifests_reference_put(name, reference, api_version)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |
**reference** | **String** |  | [required] |
**api_version** | Option<**String**> | The requested API version |  |[default to 1.0]

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

