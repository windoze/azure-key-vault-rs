/*
 * KeyVaultClient
 *
 * The key vault client performs cryptographic key operations and vault operations against the Key Vault service.
 *
 * The version of the OpenAPI document: 7.2
 * 
 * Generated by: https://openapi-generator.tech
 */

/// SecretRestoreParameters : The secret restore parameters.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecretRestoreParameters {
    /// The backup blob associated with a secret bundle.
    #[serde(rename = "value")]
    pub value: String,
}

impl SecretRestoreParameters {
    /// The secret restore parameters.
    pub fn new(value: String) -> SecretRestoreParameters {
        SecretRestoreParameters {
            value,
        }
    }
}


