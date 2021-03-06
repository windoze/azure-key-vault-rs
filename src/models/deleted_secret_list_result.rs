/*
 * KeyVaultClient
 *
 * The key vault client performs cryptographic key operations and vault operations against the Key Vault service.
 *
 * The version of the OpenAPI document: 7.2
 * 
 * Generated by: https://openapi-generator.tech
 */

/// DeletedSecretListResult : The deleted secret list result



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeletedSecretListResult {
    /// A response message containing a list of the deleted secrets in the vault along with a link to the next page of deleted secrets
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<Vec<crate::models::DeletedSecretItem>>,
    /// The URL to get the next set of deleted secrets.
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}

impl DeletedSecretListResult {
    /// The deleted secret list result
    pub fn new() -> DeletedSecretListResult {
        DeletedSecretListResult {
            value: None,
            next_link: None,
        }
    }
}


