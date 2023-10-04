# \SqlDatabasesApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_sql_databases_create_post**](SqlDatabasesApi.md#api_sql_databases_create_post) | **POST** /api/sql-databases/create | 
[**api_sql_databases_database_links_delete**](SqlDatabasesApi.md#api_sql_databases_database_links_delete) | **DELETE** /api/sql-databases/{database}/links | 
[**api_sql_databases_database_links_post**](SqlDatabasesApi.md#api_sql_databases_database_links_post) | **POST** /api/sql-databases/{database}/links | 
[**api_sql_databases_database_rename_patch**](SqlDatabasesApi.md#api_sql_databases_database_rename_patch) | **PATCH** /api/sql-databases/{database}/rename | 
[**api_sql_databases_delete**](SqlDatabasesApi.md#api_sql_databases_delete) | **DELETE** /api/sql-databases | 
[**api_sql_databases_execute_post**](SqlDatabasesApi.md#api_sql_databases_execute_post) | **POST** /api/sql-databases/execute | 
[**api_sql_databases_get**](SqlDatabasesApi.md#api_sql_databases_get) | **GET** /api/sql-databases | 



## api_sql_databases_create_post

> api_sql_databases_create_post(create_sql_database_command, api_version)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_sql_database_command** | [**CreateSqlDatabaseCommand**](CreateSqlDatabaseCommand.md) |  | [required] |
**api_version** | Option<**String**> | The requested API version |  |[default to 1.0]

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/json, text/json, application/*+json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_sql_databases_database_links_delete

> api_sql_databases_database_links_delete(database, resource_label, api_version)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**database** | **String** |  | [required] |
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


## api_sql_databases_database_links_post

> api_sql_databases_database_links_post(database, resource_label, api_version)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**database** | **String** |  | [required] |
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


## api_sql_databases_database_rename_patch

> api_sql_databases_database_rename_patch(database, body, api_version)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**database** | **String** |  | [required] |
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


## api_sql_databases_delete

> api_sql_databases_delete(delete_sql_database_command, api_version)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**delete_sql_database_command** | [**DeleteSqlDatabaseCommand**](DeleteSqlDatabaseCommand.md) |  | [required] |
**api_version** | Option<**String**> | The requested API version |  |[default to 1.0]

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/json, text/json, application/*+json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_sql_databases_execute_post

> api_sql_databases_execute_post(execute_sql_statement_command, api_version)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**execute_sql_statement_command** | [**ExecuteSqlStatementCommand**](ExecuteSqlStatementCommand.md) |  | [required] |
**api_version** | Option<**String**> | The requested API version |  |[default to 1.0]

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/json, text/json, application/*+json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_sql_databases_get

> crate::models::DatabasesList api_sql_databases_get(get_sql_databases_query, api_version)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**get_sql_databases_query** | [**GetSqlDatabasesQuery**](GetSqlDatabasesQuery.md) |  | [required] |
**api_version** | Option<**String**> | The requested API version |  |[default to 1.0]

### Return type

[**crate::models::DatabasesList**](DatabasesList.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/json, text/json, application/*+json
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

