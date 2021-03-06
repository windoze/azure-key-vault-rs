/*
 * KeyVaultClient
 *
 * The key vault client performs cryptographic key operations and vault operations against the Key Vault service.
 *
 * The version of the OpenAPI document: 7.2
 * 
 * Generated by: https://openapi-generator.tech
 */

/// RoleDefinitionListResult : Role definition list operation result.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RoleDefinitionListResult {
    /// Role definition list.
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<Vec<crate::models::RoleDefinition>>,
    /// The URL to use for getting the next set of results.
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}

impl RoleDefinitionListResult {
    /// Role definition list operation result.
    pub fn new() -> RoleDefinitionListResult {
        RoleDefinitionListResult {
            value: None,
            next_link: None,
        }
    }
}


