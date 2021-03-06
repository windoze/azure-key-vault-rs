/*
 * KeyVaultClient
 *
 * The key vault client performs cryptographic key operations and vault operations against the Key Vault service.
 *
 * The version of the OpenAPI document: 7.2
 * 
 * Generated by: https://openapi-generator.tech
 */

/// DeletedCertificateItem : The deleted certificate item containing metadata about the deleted certificate.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeletedCertificateItem {
    /// The url of the recovery object, used to identify and recover the deleted certificate.
    #[serde(rename = "recoveryId", skip_serializing_if = "Option::is_none")]
    pub recovery_id: Option<String>,
    /// The time when the certificate is scheduled to be purged, in UTC
    #[serde(rename = "scheduledPurgeDate", skip_serializing_if = "Option::is_none")]
    pub scheduled_purge_date: Option<i32>,
    /// The time when the certificate was deleted, in UTC
    #[serde(rename = "deletedDate", skip_serializing_if = "Option::is_none")]
    pub deleted_date: Option<i32>,
    /// Certificate identifier.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "attributes", skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Box<crate::models::CertificateAttributes>>,
    /// Application specific metadata in the form of key-value pairs.
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    /// Thumbprint of the certificate.
    #[serde(rename = "x5t", skip_serializing_if = "Option::is_none")]
    pub x5t: Option<String>,
}

impl DeletedCertificateItem {
    /// The deleted certificate item containing metadata about the deleted certificate.
    pub fn new() -> DeletedCertificateItem {
        DeletedCertificateItem {
            recovery_id: None,
            scheduled_purge_date: None,
            deleted_date: None,
            id: None,
            attributes: None,
            tags: None,
            x5t: None,
        }
    }
}


