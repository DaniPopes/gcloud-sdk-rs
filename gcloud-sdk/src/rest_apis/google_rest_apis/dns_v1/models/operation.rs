use serde::{Deserialize, Serialize}; /*
                                      * Cloud DNS API
                                      *
                                      * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
                                      *
                                      * The version of the OpenAPI document: v1
                                      *
                                      * Generated by: https://openapi-generator.tech
                                      */

/// Operation : An operation represents a successful mutation performed on a Cloud DNS resource. Operations provide: - An audit log of server resource mutations. - A way to recover/retry API calls in the case where the response is never received by the caller. Use the caller specified client_operation_id.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Operation {
    #[serde(rename = "dnsKeyContext", skip_serializing_if = "Option::is_none")]
    pub dns_key_context:
        Option<Box<crate::google_rest_apis::dns_v1::models::OperationDnsKeyContext>>,
    /// Unique identifier for the resource. This is the client_operation_id if the client specified it when the mutation was initiated, otherwise, it is generated by the server. The name must be 1-63 characters long and match the regular expression [-a-z0-9]? (output only)
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// The time that this operation was started by the server. This is in RFC3339 text format (output only).
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// Status of the operation. Can be one of the following: \"PENDING\" or \"DONE\" (output only). A status of \"DONE\" means that the request to update the authoritative servers has been sent, but the servers might not be updated yet.
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    /// Type of the operation. Operations include insert, update, and delete (output only).
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// User who requested the operation, for example: user@example.com. cloud-dns-system for operations automatically done by the system. (output only)
    #[serde(rename = "user", skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    #[serde(rename = "zoneContext", skip_serializing_if = "Option::is_none")]
    pub zone_context:
        Option<Box<crate::google_rest_apis::dns_v1::models::OperationManagedZoneContext>>,
}

impl Operation {
    /// An operation represents a successful mutation performed on a Cloud DNS resource. Operations provide: - An audit log of server resource mutations. - A way to recover/retry API calls in the case where the response is never received by the caller. Use the caller specified client_operation_id.
    pub fn new() -> Operation {
        Operation {
            dns_key_context: None,
            id: None,
            kind: None,
            start_time: None,
            status: None,
            r#type: None,
            user: None,
            zone_context: None,
        }
    }
}

/// Status of the operation. Can be one of the following: \"PENDING\" or \"DONE\" (output only). A status of \"DONE\" means that the request to update the authoritative servers has been sent, but the servers might not be updated yet.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "done")]
    Done,
}

impl Default for Status {
    fn default() -> Status {
        Self::Pending
    }
}