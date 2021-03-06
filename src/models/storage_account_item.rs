/*
 * KeyVaultClient
 *
 * The key vault client performs cryptographic key operations and vault operations against the Key Vault service.
 *
 * The version of the OpenAPI document: 7.2
 * 
 * Generated by: https://openapi-generator.tech
 */

/// StorageAccountItem : The storage account item containing storage account metadata.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageAccountItem {
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

impl StorageAccountItem {
    /// The storage account item containing storage account metadata.
    pub fn new() -> StorageAccountItem {
        StorageAccountItem {
            id: None,
            resource_id: None,
            attributes: None,
            tags: None,
        }
    }
}


