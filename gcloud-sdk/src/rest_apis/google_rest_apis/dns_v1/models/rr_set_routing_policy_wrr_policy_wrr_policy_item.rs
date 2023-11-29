use serde::{Deserialize, Serialize}; /*
                                      * Cloud DNS API
                                      *
                                      * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
                                      *
                                      * The version of the OpenAPI document: v1
                                      *
                                      * Generated by: https://openapi-generator.tech
                                      */

/// RrSetRoutingPolicyWrrPolicyWrrPolicyItem : A routing block which contains the routing information for one WRR item.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct RrSetRoutingPolicyWrrPolicyWrrPolicyItem {
    #[serde(
        rename = "healthCheckedTargets",
        skip_serializing_if = "Option::is_none"
    )]
    pub health_checked_targets:
        Option<Box<crate::google_rest_apis::dns_v1::models::RrSetRoutingPolicyHealthCheckTargets>>,
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(rename = "rrdatas", skip_serializing_if = "Option::is_none")]
    pub rrdatas: Option<Vec<String>>,
    /// DNSSEC generated signatures for all the rrdata within this item. Note that if health checked targets are provided for DNSSEC enabled zones, there's a restriction of 1 IP address per item.
    #[serde(rename = "signatureRrdatas", skip_serializing_if = "Option::is_none")]
    pub signature_rrdatas: Option<Vec<String>>,
    /// The weight corresponding to this WrrPolicyItem object. When multiple WrrPolicyItem objects are configured, the probability of returning an WrrPolicyItem object's data is proportional to its weight relative to the sum of weights configured for all items. This weight must be non-negative.
    #[serde(rename = "weight", skip_serializing_if = "Option::is_none")]
    pub weight: Option<f64>,
}

impl RrSetRoutingPolicyWrrPolicyWrrPolicyItem {
    /// A routing block which contains the routing information for one WRR item.
    pub fn new() -> RrSetRoutingPolicyWrrPolicyWrrPolicyItem {
        RrSetRoutingPolicyWrrPolicyWrrPolicyItem {
            health_checked_targets: None,
            kind: None,
            rrdatas: None,
            signature_rrdatas: None,
            weight: None,
        }
    }
}