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
pub struct InterconnectDiagnosticsLinkStatus {
    /// A list of InterconnectDiagnostics.ARPEntry objects, describing the ARP neighbor entries seen on this link. This will be empty if the link is bundled
    #[serde(rename = "arpCaches", skip_serializing_if = "Option::is_none")]
    pub arp_caches:
        Option<Vec<crate::google_rest_apis::compute_v1::models::InterconnectDiagnosticsArpEntry>>,
    /// The unique ID for this link assigned during turn up by Google.
    #[serde(rename = "circuitId", skip_serializing_if = "Option::is_none")]
    pub circuit_id: Option<String>,
    /// The Demarc address assigned by Google and provided in the LoA.
    #[serde(rename = "googleDemarc", skip_serializing_if = "Option::is_none")]
    pub google_demarc: Option<String>,
    #[serde(rename = "lacpStatus", skip_serializing_if = "Option::is_none")]
    pub lacp_status: Option<
        Box<crate::google_rest_apis::compute_v1::models::InterconnectDiagnosticsLinkLacpStatus>,
    >,
    #[serde(rename = "macsec", skip_serializing_if = "Option::is_none")]
    pub macsec: Option<
        Box<crate::google_rest_apis::compute_v1::models::InterconnectDiagnosticsMacsecStatus>,
    >,
    /// The operational status of the link.
    #[serde(rename = "operationalStatus", skip_serializing_if = "Option::is_none")]
    pub operational_status: Option<OperationalStatus>,
    #[serde(
        rename = "receivingOpticalPower",
        skip_serializing_if = "Option::is_none"
    )]
    pub receiving_optical_power: Option<
        Box<crate::google_rest_apis::compute_v1::models::InterconnectDiagnosticsLinkOpticalPower>,
    >,
    #[serde(
        rename = "transmittingOpticalPower",
        skip_serializing_if = "Option::is_none"
    )]
    pub transmitting_optical_power: Option<
        Box<crate::google_rest_apis::compute_v1::models::InterconnectDiagnosticsLinkOpticalPower>,
    >,
}

impl InterconnectDiagnosticsLinkStatus {
    pub fn new() -> InterconnectDiagnosticsLinkStatus {
        InterconnectDiagnosticsLinkStatus {
            arp_caches: None,
            circuit_id: None,
            google_demarc: None,
            lacp_status: None,
            macsec: None,
            operational_status: None,
            receiving_optical_power: None,
            transmitting_optical_power: None,
        }
    }
}

/// The operational status of the link.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum OperationalStatus {
    #[serde(rename = "LINK_OPERATIONAL_STATUS_DOWN")]
    Down,
    #[serde(rename = "LINK_OPERATIONAL_STATUS_UP")]
    Up,
}

impl Default for OperationalStatus {
    fn default() -> OperationalStatus {
        Self::Down
    }
}