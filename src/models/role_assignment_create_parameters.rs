/*
 * KeyVaultClient
 *
 * The key vault client performs cryptographic key operations and vault operations against the Key Vault service.
 *
 * The version of the OpenAPI document: 7.2
 * 
 * Generated by: https://openapi-generator.tech
 */

/// RoleAssignmentCreateParameters : Role assignment create parameters.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RoleAssignmentCreateParameters {
    #[serde(rename = "properties")]
    pub properties: Box<crate::models::RoleAssignmentProperties>,
}

impl RoleAssignmentCreateParameters {
    /// Role assignment create parameters.
    pub fn new(properties: crate::models::RoleAssignmentProperties) -> RoleAssignmentCreateParameters {
        RoleAssignmentCreateParameters {
            properties: Box::new(properties),
        }
    }
}


