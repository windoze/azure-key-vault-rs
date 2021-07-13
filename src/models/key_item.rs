/*
 * KeyVaultClient
 *
 * The key vault client performs cryptographic key operations and vault operations against the Key Vault service.
 *
 * The version of the OpenAPI document: 7.2
 * 
 * Generated by: https://openapi-generator.tech
 */

/// KeyItem : The key item containing key metadata.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KeyItem {
    /// Key identifier.
    #[serde(rename = "kid", skip_serializing_if = "Option::is_none")]
    pub kid: Option<String>,
    #[serde(rename = "attributes", skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Box<crate::models::KeyAttributes>>,
    /// Application specific metadata in the form of key-value pairs.
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    /// True if the key's lifetime is managed by key vault. If this is a key backing a certificate, then managed will be true.
    #[serde(rename = "managed", skip_serializing_if = "Option::is_none")]
    pub managed: Option<bool>,
}

impl KeyItem {
    /// The key item containing key metadata.
    pub fn new() -> KeyItem {
        KeyItem {
            kid: None,
            attributes: None,
            tags: None,
            managed: None,
        }
    }
}

