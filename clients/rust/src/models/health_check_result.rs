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
pub struct HealthCheckResult {
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<models::ApiHealthStatus>,
}

impl HealthCheckResult {
    pub fn new() -> HealthCheckResult {
        HealthCheckResult {
            status: None,
        }
    }
}

