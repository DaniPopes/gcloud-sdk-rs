use serde::{Deserialize, Serialize}; /*
                                      * Compute Engine API
                                      *
                                      * Creates and runs virtual machines on Google Cloud Platform.
                                      *
                                      * The version of the OpenAPI document: v1
                                      *
                                      * Generated by: https://openapi-generator.tech
                                      */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct InstancesSetServiceAccountRequest {
    /// Email address of the service account.
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// The list of scopes to be made available for this service account.
    #[serde(rename = "scopes", skip_serializing_if = "Option::is_none")]
    pub scopes: Option<Vec<String>>,
}

impl InstancesSetServiceAccountRequest {
    pub fn new() -> InstancesSetServiceAccountRequest {
        InstancesSetServiceAccountRequest {
            email: None,
            scopes: None,
        }
    }
}