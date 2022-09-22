/*
 * Fermyon.Cloud.Web
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0
 * 
 * Generated by: https://openapi-generator.tech
 */


/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum JobStatus {
    #[serde(rename = "Unknown")]
    Unknown,
    #[serde(rename = "Pending")]
    Pending,
    #[serde(rename = "Running")]
    Running,
    #[serde(rename = "Dead")]
    Dead,

}

impl ToString for JobStatus {
    fn to_string(&self) -> String {
        match self {
            Self::Unknown => String::from("Unknown"),
            Self::Pending => String::from("Pending"),
            Self::Running => String::from("Running"),
            Self::Dead => String::from("Dead"),
        }
    }
}

impl Default for JobStatus {
    fn default() -> JobStatus {
        Self::Unknown
    }
}




