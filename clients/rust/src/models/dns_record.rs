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
pub struct DnsRecord {
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "value")]
    pub value: String,
    #[serde(rename = "ttl")]
    pub ttl: i32,
}

impl DnsRecord {
    pub fn new(r#type: String, name: String, value: String, ttl: i32) -> DnsRecord {
        DnsRecord {
            r#type,
            name,
            value,
            ttl,
        }
    }
}

