/*
 * KeyVaultClient
 *
 * The key vault client performs cryptographic key operations and vault operations against the Key Vault service.
 *
 * The version of the OpenAPI document: 7.2
 * 
 * Generated by: https://openapi-generator.tech
 */

/// DeletedStorageAccountItem : The deleted storage account item containing metadata about the deleted storage account.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeletedStorageAccountItem {
    /// The url of the recovery object, used to identify and recover the deleted storage account.
    #[serde(rename = "recoveryId", skip_serializing_if = "Option::is_none")]
    pub recovery_id: Option<String>,
    /// The time when the storage account is scheduled to be purged, in UTC
    #[serde(rename = "scheduledPurgeDate", skip_serializing_if = "Option::is_none")]
    pub scheduled_purge_date: Option<i32>,
    /// The time when the storage account was deleted, in UTC
    #[serde(rename = "deletedDate", skip_serializing_if = "Option::is_none")]
    pub deleted_date: Option<i32>,
    /// Storage identifier.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Storage account resource Id.
    #[serde(rename = "resourceId", skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[serde(rename = "attributes", skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Box<crate::models::StorageAccountAttributes>>,
    /// Application specific metadata in the form of key-value pairs.
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

impl DeletedStorageAccountItem {
    /// The deleted storage account item containing metadata about the deleted storage account.
    pub fn new() -> DeletedStorageAccountItem {
        DeletedStorageAccountItem {
            recovery_id: None,
            scheduled_purge_date: None,
            deleted_date: None,
            id: None,
            resource_id: None,
            attributes: None,
            tags: None,
        }
    }
}


