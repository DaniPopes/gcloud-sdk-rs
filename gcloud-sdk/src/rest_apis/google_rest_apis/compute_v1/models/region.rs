use serde::{Deserialize, Serialize}; /*
                                      * Compute Engine API
                                      *
                                      * Creates and runs virtual machines on Google Cloud Platform.
                                      *
                                      * The version of the OpenAPI document: v1
                                      *
                                      * Generated by: https://openapi-generator.tech
                                      */

/// Region : Represents a Region resource. A region is a geographical area where a resource is located. For more information, read Regions and Zones.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Region {
    /// [Output Only] Creation timestamp in RFC3339 text format.
    #[serde(rename = "creationTimestamp", skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<String>,
    #[serde(rename = "deprecated", skip_serializing_if = "Option::is_none")]
    pub deprecated: Option<Box<crate::google_rest_apis::compute_v1::models::DeprecationStatus>>,
    /// [Output Only] Textual description of the resource.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// [Output Only] The unique identifier for the resource. This identifier is defined by the server.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// [Output Only] Type of the resource. Always compute#region for regions.
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// [Output Only] Name of the resource.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// [Output Only] Quotas assigned to this region.
    #[serde(rename = "quotas", skip_serializing_if = "Option::is_none")]
    pub quotas: Option<Vec<crate::google_rest_apis::compute_v1::models::Quota>>,
    /// [Output Only] Server-defined URL for the resource.
    #[serde(rename = "selfLink", skip_serializing_if = "Option::is_none")]
    pub self_link: Option<String>,
    /// [Output Only] Status of the region, either UP or DOWN.
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    /// [Output Only] Reserved for future use.
    #[serde(rename = "supportsPzs", skip_serializing_if = "Option::is_none")]
    pub supports_pzs: Option<bool>,
    /// [Output Only] A list of zones available in this region, in the form of resource URLs.
    #[serde(rename = "zones", skip_serializing_if = "Option::is_none")]
    pub zones: Option<Vec<String>>,
}

impl Region {
    /// Represents a Region resource. A region is a geographical area where a resource is located. For more information, read Regions and Zones.
    pub fn new() -> Region {
        Region {
            creation_timestamp: None,
            deprecated: None,
            description: None,
            id: None,
            kind: None,
            name: None,
            quotas: None,
            self_link: None,
            status: None,
            supports_pzs: None,
            zones: None,
        }
    }
}

/// [Output Only] Status of the region, either UP or DOWN.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "DOWN")]
    Down,
    #[serde(rename = "UP")]
    Up,
}

impl Default for Status {
    fn default() -> Status {
        Self::Down
    }
}