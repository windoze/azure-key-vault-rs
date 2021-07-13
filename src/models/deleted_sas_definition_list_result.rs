/*
 * KeyVaultClient
 *
 * The key vault client performs cryptographic key operations and vault operations against the Key Vault service.
 *
 * The version of the OpenAPI document: 7.2
 * 
 * Generated by: https://openapi-generator.tech
 */

/// DeletedSasDefinitionListResult : The deleted SAS definition list result



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeletedSasDefinitionListResult {
    /// A response message containing a list of the deleted SAS definitions in the vault along with a link to the next page of deleted sas definitions
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<Vec<crate::models::DeletedSasDefinitionItem>>,
    /// The URL to get the next set of deleted SAS definitions.
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}

impl DeletedSasDefinitionListResult {
    /// The deleted SAS definition list result
    pub fn new() -> DeletedSasDefinitionListResult {
        DeletedSasDefinitionListResult {
            value: None,
            next_link: None,
        }
    }
}


