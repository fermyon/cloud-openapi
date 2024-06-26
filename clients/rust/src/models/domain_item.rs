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
pub struct DomainItem {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "validationStatus")]
    pub validation_status: String,
    #[serde(rename = "validatedAt", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub validated_at: Option<Option<String>>,
    #[serde(rename = "lastModified", skip_serializing_if = "Option::is_none")]
    pub last_modified: Option<String>,
    #[serde(rename = "dnsRecords")]
    pub dns_records: Vec<models::DnsRecord>,
}

impl DomainItem {
    pub fn new(name: String, validation_status: String, dns_records: Vec<models::DnsRecord>) -> DomainItem {
        DomainItem {
            name,
            validation_status,
            validated_at: None,
            last_modified: None,
            dns_records,
        }
    }
}

