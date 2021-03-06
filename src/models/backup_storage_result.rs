/*
 * KeyVaultClient
 *
 * The key vault client performs cryptographic key operations and vault operations against the Key Vault service.
 *
 * The version of the OpenAPI document: 7.2
 * 
 * Generated by: https://openapi-generator.tech
 */

/// BackupStorageResult : The backup storage result, containing the backup blob.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BackupStorageResult {
    /// The backup blob containing the backed up storage account.
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl BackupStorageResult {
    /// The backup storage result, containing the backup blob.
    pub fn new() -> BackupStorageResult {
        BackupStorageResult {
            value: None,
        }
    }
}


