/*
 * KeyVaultClient
 *
 * The key vault client performs cryptographic key operations and vault operations against the Key Vault service.
 *
 * The version of the OpenAPI document: 7.2
 * 
 * Generated by: https://openapi-generator.tech
 */

/// DeletedSasDefinitionItem : The deleted SAS definition item containing metadata about the deleted SAS definition.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeletedSasDefinitionItem {
    /// The url of the recovery object, used to identify and recover the deleted SAS definition.
    #[serde(rename = "recoveryId", skip_serializing_if = "Option::is_none")]
    pub recovery_id: Option<String>,
    /// The time when the SAS definition is scheduled to be purged, in UTC
    #[serde(rename = "scheduledPurgeDate", skip_serializing_if = "Option::is_none")]
    pub scheduled_purge_date: Option<i32>,
    /// The time when the SAS definition was deleted, in UTC
    #[serde(rename = "deletedDate", skip_serializing_if = "Option::is_none")]
    pub deleted_date: Option<i32>,
    /// The storage SAS identifier.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The storage account SAS definition secret id.
    #[serde(rename = "sid", skip_serializing_if = "Option::is_none")]
    pub sid: Option<String>,
    #[serde(rename = "attributes", skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Box<crate::models::SasDefinitionAttributes>>,
    /// Application specific metadata in the form of key-value pairs.
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

impl DeletedSasDefinitionItem {
    /// The deleted SAS definition item containing metadata about the deleted SAS definition.
    pub fn new() -> DeletedSasDefinitionItem {
        DeletedSasDefinitionItem {
            recovery_id: None,
            scheduled_purge_date: None,
            deleted_date: None,
            id: None,
            sid: None,
            attributes: None,
            tags: None,
        }
    }
}


