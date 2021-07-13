/*
 * KeyVaultClient
 *
 * The key vault client performs cryptographic key operations and vault operations against the Key Vault service.
 *
 * The version of the OpenAPI document: 7.2
 * 
 * Generated by: https://openapi-generator.tech
 */

/// DeletedStorageListResult : The deleted storage account list result



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeletedStorageListResult {
    /// A response message containing a list of the deleted storage accounts in the vault along with a link to the next page of deleted storage accounts
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<Vec<crate::models::DeletedStorageAccountItem>>,
    /// The URL to get the next set of deleted storage accounts.
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}

impl DeletedStorageListResult {
    /// The deleted storage account list result
    pub fn new() -> DeletedStorageListResult {
        DeletedStorageListResult {
            value: None,
            next_link: None,
        }
    }
}


