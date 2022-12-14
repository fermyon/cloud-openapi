/*
 * Fermyon.Cloud.Web
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct UpdateEnvironmentVariableDto {
    #[serde(rename = "key")]
    pub key: String,
    #[serde(rename = "value")]
    pub value: String,
}

impl UpdateEnvironmentVariableDto {
    pub fn new(key: String, value: String) -> UpdateEnvironmentVariableDto {
        UpdateEnvironmentVariableDto {
            key,
            value,
        }
    }
}


