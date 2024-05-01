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
pub struct DeleteVariablePairCommand {
    #[serde(rename = "appId")]
    pub app_id: uuid::Uuid,
    #[serde(rename = "variable")]
    pub variable: String,
}

impl DeleteVariablePairCommand {
    pub fn new(app_id: uuid::Uuid, variable: String) -> DeleteVariablePairCommand {
        DeleteVariablePairCommand {
            app_id,
            variable,
        }
    }
}

