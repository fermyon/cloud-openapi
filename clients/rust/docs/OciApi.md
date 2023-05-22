# \OciApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_oci_get**](OciApi.md#api_oci_get) | **GET** /api/oci | 
[**api_oci_name_blobs_uploads_digest_delete**](OciApi.md#api_oci_name_blobs_uploads_digest_delete) | **DELETE** /api/oci/{name}/blobs/uploads/{digest} | 
[**api_oci_name_blobs_uploads_digest_get**](OciApi.md#api_oci_name_blobs_uploads_digest_get) | **GET** /api/oci/{name}/blobs/uploads/{digest} | 
[**api_oci_name_blobs_uploads_digest_patch**](OciApi.md#api_oci_name_blobs_uploads_digest_patch) | **PATCH** /api/oci/{name}/blobs/uploads/{digest} | 
[**api_oci_name_blobs_uploads_digest_put**](OciApi.md#api_oci_name_blobs_uploads_digest_put) | **PUT** /api/oci/{name}/blobs/uploads/{digest} | 
[**api_oci_name_blobs_uploads_post**](OciApi.md#api_oci_name_blobs_uploads_post) | **POST** /api/oci/{name}/blobs/uploads | 
[**api_oci_name_manifests_reference_head**](OciApi.md#api_oci_name_manifests_reference_head) | **HEAD** /api/oci/{name}/manifests/{reference} | 
[**api_oci_name_manifests_reference_put**](OciApi.md#api_oci_name_manifests_reference_put) | **PUT** /api/oci/{name}/manifests/{reference} | 



## api_oci_get

> api_oci_get(api_version)


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


## api_oci_name_blobs_uploads_digest_delete

> api_oci_name_blobs_uploads_digest_delete(digest, name, api_version)


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


## api_oci_name_blobs_uploads_digest_get

> api_oci_name_blobs_uploads_digest_get(digest, name, api_version)


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


## api_oci_name_blobs_uploads_digest_patch

> api_oci_name_blobs_uploads_digest_patch(digest, name, api_version)


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


## api_oci_name_blobs_uploads_digest_put

> api_oci_name_blobs_uploads_digest_put(digest, name, api_version)


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


## api_oci_name_blobs_uploads_post

> api_oci_name_blobs_uploads_post(name, api_version)


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


## api_oci_name_manifests_reference_head

> api_oci_name_manifests_reference_head(name, reference, api_version)


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


## api_oci_name_manifests_reference_put

> api_oci_name_manifests_reference_put(name, reference, api_version)


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

