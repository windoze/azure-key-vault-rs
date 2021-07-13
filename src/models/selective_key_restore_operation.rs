/*
 * KeyVaultClient
 *
 * The key vault client performs cryptographic key operations and vault operations against the Key Vault service.
 *
 * The version of the OpenAPI document: 7.2
 * 
 * Generated by: https://openapi-generator.tech
 */

/// SelectiveKeyRestoreOperation : Selective Key Restore operation



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SelectiveKeyRestoreOperation {
    /// Status of the restore operation.
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// The status details of restore operation.
    #[serde(rename = "statusDetails", skip_serializing_if = "Option::is_none")]
    pub status_details: Option<String>,
    #[serde(rename = "error", skip_serializing_if = "Option::is_none")]
    pub error: Option<Box<crate::models::Error>>,
    /// Identifier for the selective key restore operation.
    #[serde(rename = "jobId", skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    /// The start time of the restore operation
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    pub start_time: Option<i32>,
    /// The end time of the restore operation
    #[serde(rename = "endTime", skip_serializing_if = "Option::is_none")]
    pub end_time: Option<i32>,
}

impl SelectiveKeyRestoreOperation {
    /// Selective Key Restore operation
    pub fn new() -> SelectiveKeyRestoreOperation {
        SelectiveKeyRestoreOperation {
            status: None,
            status_details: None,
            error: None,
            job_id: None,
            start_time: None,
            end_time: None,
        }
    }
}


