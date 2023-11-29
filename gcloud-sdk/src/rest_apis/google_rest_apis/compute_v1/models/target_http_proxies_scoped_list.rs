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
pub struct TargetHttpProxiesScopedList {
    /// A list of TargetHttpProxies contained in this scope.
    #[serde(rename = "targetHttpProxies", skip_serializing_if = "Option::is_none")]
    pub target_http_proxies:
        Option<Vec<crate::google_rest_apis::compute_v1::models::TargetHttpProxy>>,
    #[serde(rename = "warning", skip_serializing_if = "Option::is_none")]
    pub warning:
        Option<Box<crate::google_rest_apis::compute_v1::models::BackendServicesScopedListWarning>>,
}

impl TargetHttpProxiesScopedList {
    pub fn new() -> TargetHttpProxiesScopedList {
        TargetHttpProxiesScopedList {
            target_http_proxies: None,
            warning: None,
        }
    }
}