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
pub struct GetAppLogsVm {
    #[serde(rename = "logs")]
    pub logs: Vec<String>,
}

impl GetAppLogsVm {
    pub fn new(logs: Vec<String>) -> GetAppLogsVm {
        GetAppLogsVm {
            logs,
        }
    }
}


