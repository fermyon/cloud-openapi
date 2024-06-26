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
pub struct LogLine {
    #[serde(rename = "time", skip_serializing_if = "Option::is_none")]
    pub time: Option<String>,
    #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(rename = "line", skip_serializing_if = "Option::is_none")]
    pub line: Option<String>,
}

impl LogLine {
    pub fn new() -> LogLine {
        LogLine {
            time: None,
            source: None,
            line: None,
        }
    }
}

