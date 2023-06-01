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
pub struct ListVariablesQuery {
    #[serde(rename = "appId")]
    pub app_id: uuid::Uuid,
}

impl ListVariablesQuery {
    pub fn new(app_id: uuid::Uuid) -> ListVariablesQuery {
        ListVariablesQuery {
            app_id,
        }
    }
}


