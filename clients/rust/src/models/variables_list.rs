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
pub struct VariablesList {
    #[serde(rename = "vars")]
    pub vars: Vec<String>,
}

impl VariablesList {
    pub fn new(vars: Vec<String>) -> VariablesList {
        VariablesList {
            vars,
        }
    }
}

