/*
 * KeyVaultClient
 *
 * The key vault client performs cryptographic key operations and vault operations against the Key Vault service.
 *
 * The version of the OpenAPI document: 7.2
 * 
 * Generated by: https://openapi-generator.tech
 */

/// SecretProperties : Properties of the key backing a certificate.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecretProperties {
    /// The media type (MIME type).
    #[serde(rename = "contentType", skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
}

impl SecretProperties {
    /// Properties of the key backing a certificate.
    pub fn new() -> SecretProperties {
        SecretProperties {
            content_type: None,
        }
    }
}


