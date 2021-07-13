/*
 * KeyVaultClient
 *
 * The key vault client performs cryptographic key operations and vault operations against the Key Vault service.
 *
 * The version of the OpenAPI document: 7.2
 * 
 * Generated by: https://openapi-generator.tech
 */

/// BackupCertificateResult : The backup certificate result, containing the backup blob.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BackupCertificateResult {
    /// The backup blob containing the backed up certificate.
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl BackupCertificateResult {
    /// The backup certificate result, containing the backup blob.
    pub fn new() -> BackupCertificateResult {
        BackupCertificateResult {
            value: None,
        }
    }
}

