/*
 * KeyVaultClient
 *
 * The key vault client performs cryptographic key operations and vault operations against the Key Vault service.
 *
 * The version of the OpenAPI document: 7.2
 * 
 * Generated by: https://openapi-generator.tech
 */

/// BackupKeyResult : The backup key result, containing the backup blob.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BackupKeyResult {
    /// The backup blob containing the backed up key.
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl BackupKeyResult {
    /// The backup key result, containing the backup blob.
    pub fn new() -> BackupKeyResult {
        BackupKeyResult {
            value: None,
        }
    }
}


