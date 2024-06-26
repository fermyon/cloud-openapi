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

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AppEventType {
    #[serde(rename = "Unknown")]
    Unknown,
    #[serde(rename = "Create")]
    Create,
    #[serde(rename = "Update")]
    Update,
    #[serde(rename = "Delete")]
    Delete,

}

impl ToString for AppEventType {
    fn to_string(&self) -> String {
        match self {
            Self::Unknown => String::from("Unknown"),
            Self::Create => String::from("Create"),
            Self::Update => String::from("Update"),
            Self::Delete => String::from("Delete"),
        }
    }
}

impl Default for AppEventType {
    fn default() -> AppEventType {
        Self::Unknown
    }
}

