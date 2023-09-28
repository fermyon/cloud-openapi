# \CustomDomainsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_custom_domains_domain_name_get**](CustomDomainsApi.md#api_custom_domains_domain_name_get) | **GET** /api/custom-domains/{domainName} | 



## api_custom_domains_domain_name_get

> crate::models::DomainItem api_custom_domains_domain_name_get(domain_name, api_version)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_name** | **String** |  | [required] |
**api_version** | Option<**String**> | The requested API version |  |[default to 1.0]

### Return type

[**crate::models::DomainItem**](DomainItem.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

