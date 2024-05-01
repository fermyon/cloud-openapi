# AppItem

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**name** | **String** |  | 
**storage_id** | **String** |  | 
**description** | Option<**String**> |  | [optional]
**subdomain** | **String** |  | 
**health_status** | Option<[**crate::models::ApiHealthStatus**](ApiHealthStatus.md)> |  | [optional]
**channels** | [**Vec<crate::models::AppChannelListItem>**](AppChannelListItem.md) |  | 
**domain** | Option<[**crate::models::AppDomainItem**](AppDomainItem.md)> |  | [optional]
**last_modified** | **String** |  | 
**last_deployed** | Option<**String**> |  | [optional]
**created** | **String** |  | 
**latest_revision** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


