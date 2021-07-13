/*
 * KeyVaultClient
 *
 * The key vault client performs cryptographic key operations and vault operations against the Key Vault service.
 *
 * The version of the OpenAPI document: 7.2
 * 
 * Generated by: https://openapi-generator.tech
 */

/// KeyRestoreParameters : The key restore parameters.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KeyRestoreParameters {
    /// The backup blob associated with a key bundle.
    #[serde(rename = "value")]
    pub value: String,
}

impl KeyRestoreParameters {
    /// The key restore parameters.
    pub fn new(value: String) -> KeyRestoreParameters {
        KeyRestoreParameters {
            value,
        }
    }
}


