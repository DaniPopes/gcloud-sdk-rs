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
pub struct ReservationsResizeRequest {
    /// Number of allocated resources can be resized with minimum = 1 and maximum = 1000.
    #[serde(rename = "specificSkuCount", skip_serializing_if = "Option::is_none")]
    pub specific_sku_count: Option<String>,
}

impl ReservationsResizeRequest {
    pub fn new() -> ReservationsResizeRequest {
        ReservationsResizeRequest {
            specific_sku_count: None,
        }
    }
}