/*
 * KeyVaultClient
 *
 * The key vault client performs cryptographic key operations and vault operations against the Key Vault service.
 *
 * The version of the OpenAPI document: 7.2
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CertificateUpdateParameters : The certificate update parameters.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CertificateUpdateParameters {
    #[serde(rename = "policy", skip_serializing_if = "Option::is_none")]
    pub policy: Option<Box<crate::models::CertificatePolicy>>,
    #[serde(rename = "attributes", skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Box<crate::models::CertificateAttributes>>,
    /// Application specific metadata in the form of key-value pairs.
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

impl CertificateUpdateParameters {
    /// The certificate update parameters.
    pub fn new() -> CertificateUpdateParameters {
        CertificateUpdateParameters {
            policy: None,
            attributes: None,
            tags: None,
        }
    }
}


