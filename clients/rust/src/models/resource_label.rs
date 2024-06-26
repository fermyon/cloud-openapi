/*
 * Fermyon Cloud API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceLabel {
    #[serde(rename = "label")]
    pub label: String,
    #[serde(rename = "appId")]
    pub app_id: uuid::Uuid,
    #[serde(rename = "appName", skip_serializing_if = "Option::is_none")]
    pub app_name: Option<String>,
}

impl ResourceLabel {
    pub fn new(label: String, app_id: uuid::Uuid) -> ResourceLabel {
        ResourceLabel {
            label,
            app_id,
            app_name: None,
        }
    }
}

