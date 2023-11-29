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
pub struct LocalDisk {
    /// Specifies the number of such disks.
    #[serde(rename = "diskCount", skip_serializing_if = "Option::is_none")]
    pub disk_count: Option<i32>,
    /// Specifies the size of the disk in base-2 GB.
    #[serde(rename = "diskSizeGb", skip_serializing_if = "Option::is_none")]
    pub disk_size_gb: Option<i32>,
    /// Specifies the desired disk type on the node. This disk type must be a local storage type (e.g.: local-ssd). Note that for nodeTemplates, this should be the name of the disk type and not its URL.
    #[serde(rename = "diskType", skip_serializing_if = "Option::is_none")]
    pub disk_type: Option<String>,
}

impl LocalDisk {
    pub fn new() -> LocalDisk {
        LocalDisk {
            disk_count: None,
            disk_size_gb: None,
            disk_type: None,
        }
    }
}