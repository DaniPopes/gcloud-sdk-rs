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
pub struct NetworkAttachmentsScopedList {
    /// A list of NetworkAttachments contained in this scope.
    #[serde(rename = "networkAttachments", skip_serializing_if = "Option::is_none")]
    pub network_attachments:
        Option<Vec<crate::google_rest_apis::compute_v1::models::NetworkAttachment>>,
    #[serde(rename = "warning", skip_serializing_if = "Option::is_none")]
    pub warning: Option<
        Box<crate::google_rest_apis::compute_v1::models::NetworkAttachmentsScopedListWarning>,
    >,
}

impl NetworkAttachmentsScopedList {
    pub fn new() -> NetworkAttachmentsScopedList {
        NetworkAttachmentsScopedList {
            network_attachments: None,
            warning: None,
        }
    }
}