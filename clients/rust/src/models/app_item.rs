/*
 * Fermyon Cloud API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AppItem {
    #[serde(rename = "id")]
    pub id: uuid::Uuid,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "storageId")]
    pub storage_id: String,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "subdomain")]
    pub subdomain: String,
    #[serde(rename = "healthStatus", skip_serializing_if = "Option::is_none")]
    pub health_status: Option<crate::models::ApiHealthStatus>,
    #[serde(rename = "channels")]
    pub channels: Vec<crate::models::AppChannelListItem>,
    #[serde(rename = "domain", skip_serializing_if = "Option::is_none")]
    pub domain: Option<Box<crate::models::AppDomainItem>>,
    #[serde(rename = "lastModified")]
    pub last_modified: String,
    #[serde(rename = "lastDeployed", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub last_deployed: Option<Option<String>>,
    #[serde(rename = "created")]
    pub created: String,
    #[serde(rename = "latestRevision", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub latest_revision: Option<Option<String>>,
}

impl AppItem {
    pub fn new(id: uuid::Uuid, name: String, storage_id: String, subdomain: String, channels: Vec<crate::models::AppChannelListItem>, last_modified: String, created: String) -> AppItem {
        AppItem {
            id,
            name,
            storage_id,
            description: None,
            subdomain,
            health_status: None,
            channels,
            domain: None,
            last_modified,
            last_deployed: None,
            created,
            latest_revision: None,
        }
    }
}


