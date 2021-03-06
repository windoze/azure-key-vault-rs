/*
 * KeyVaultClient
 *
 * The key vault client performs cryptographic key operations and vault operations against the Key Vault service.
 *
 * The version of the OpenAPI document: 7.2
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CertificateItem : The certificate item containing certificate metadata.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CertificateItem {
    /// Certificate identifier.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "attributes", skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Box<crate::models::CertificateAttributes>>,
    /// Application specific metadata in the form of key-value pairs.
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    /// Thumbprint of the certificate.
    #[serde(rename = "x5t", skip_serializing_if = "Option::is_none")]
    pub x5t: Option<String>,
}

impl CertificateItem {
    /// The certificate item containing certificate metadata.
    pub fn new() -> CertificateItem {
        CertificateItem {
            id: None,
            attributes: None,
            tags: None,
            x5t: None,
        }
    }
}


