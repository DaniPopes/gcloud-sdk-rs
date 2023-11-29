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
pub struct InterconnectDiagnosticsLinkLacpStatus {
    /// System ID of the port on Google's side of the LACP exchange.
    #[serde(rename = "googleSystemId", skip_serializing_if = "Option::is_none")]
    pub google_system_id: Option<String>,
    /// System ID of the port on the neighbor's side of the LACP exchange.
    #[serde(rename = "neighborSystemId", skip_serializing_if = "Option::is_none")]
    pub neighbor_system_id: Option<String>,
    /// The state of a LACP link, which can take one of the following values: - ACTIVE: The link is configured and active within the bundle. - DETACHED: The link is not configured within the bundle. This means that the rest of the object should be empty.
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<State>,
}

impl InterconnectDiagnosticsLinkLacpStatus {
    pub fn new() -> InterconnectDiagnosticsLinkLacpStatus {
        InterconnectDiagnosticsLinkLacpStatus {
            google_system_id: None,
            neighbor_system_id: None,
            state: None,
        }
    }
}

/// The state of a LACP link, which can take one of the following values: - ACTIVE: The link is configured and active within the bundle. - DETACHED: The link is not configured within the bundle. This means that the rest of the object should be empty.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum State {
    #[serde(rename = "ACTIVE")]
    Active,
    #[serde(rename = "DETACHED")]
    Detached,
}

impl Default for State {
    fn default() -> State {
        Self::Active
    }
}