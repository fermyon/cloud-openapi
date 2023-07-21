# \PaymentsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_payments_customer_portal_get**](PaymentsApi.md#api_payments_customer_portal_get) | **GET** /api/payments/customer-portal | 
[**api_payments_plans_get**](PaymentsApi.md#api_payments_plans_get) | **GET** /api/payments/plans | 
[**api_payments_setup_checkout_post**](PaymentsApi.md#api_payments_setup_checkout_post) | **POST** /api/payments/setup-checkout | 



## api_payments_customer_portal_get

> crate::models::PaymentIntegrationUrl api_payments_customer_portal_get(api_version)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | Option<**String**> | The requested API version |  |[default to 1.0]

### Return type

[**crate::models::PaymentIntegrationUrl**](PaymentIntegrationUrl.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_payments_plans_get

> Vec<crate::models::Plan> api_payments_plans_get(api_version)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | Option<**String**> | The requested API version |  |[default to 1.0]

### Return type

[**Vec<crate::models::Plan>**](Plan.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_payments_setup_checkout_post

> crate::models::PaymentIntegrationUrl api_payments_setup_checkout_post(api_version)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | Option<**String**> | The requested API version |  |[default to 1.0]

### Return type

[**crate::models::PaymentIntegrationUrl**](PaymentIntegrationUrl.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

