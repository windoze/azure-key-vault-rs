/*
 * KeyVaultClient
 *
 * The key vault client performs cryptographic key operations and vault operations against the Key Vault service.
 *
 * The version of the OpenAPI document: 7.2
 * 
 * Generated by: https://openapi-generator.tech
 */

/// SecretSetParameters : The secret set parameters.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecretSetParameters {
    /// The value of the secret.
    #[serde(rename = "value")]
    pub value: String,
    /// Application specific metadata in the form of key-value pairs.
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    /// Type of the secret value such as a password.
    #[serde(rename = "contentType", skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[serde(rename = "attributes", skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Box<crate::models::SecretAttributes>>,
}

impl SecretSetParameters {
    /// The secret set parameters.
    pub fn new(value: String) -> SecretSetParameters {
        SecretSetParameters {
            value,
            tags: None,
            content_type: None,
            attributes: None,
        }
    }
}


