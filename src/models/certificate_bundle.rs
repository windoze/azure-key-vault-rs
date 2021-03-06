/*
 * KeyVaultClient
 *
 * The key vault client performs cryptographic key operations and vault operations against the Key Vault service.
 *
 * The version of the OpenAPI document: 7.2
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CertificateBundle : A certificate bundle consists of a certificate (X509) plus its attributes.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CertificateBundle {
    /// The certificate id.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The key id.
    #[serde(rename = "kid", skip_serializing_if = "Option::is_none")]
    pub kid: Option<String>,
    /// The secret id.
    #[serde(rename = "sid", skip_serializing_if = "Option::is_none")]
    pub sid: Option<String>,
    /// Thumbprint of the certificate.
    #[serde(rename = "x5t", skip_serializing_if = "Option::is_none")]
    pub x5t: Option<String>,
    #[serde(rename = "policy", skip_serializing_if = "Option::is_none")]
    pub policy: Option<Box<crate::models::CertificatePolicy>>,
    /// CER contents of x509 certificate.
    #[serde(rename = "cer", skip_serializing_if = "Option::is_none")]
    pub cer: Option<String>,
    /// The content type of the secret.
    #[serde(rename = "contentType", skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[serde(rename = "attributes", skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Box<crate::models::CertificateAttributes>>,
    /// Application specific metadata in the form of key-value pairs
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

impl CertificateBundle {
    /// A certificate bundle consists of a certificate (X509) plus its attributes.
    pub fn new() -> CertificateBundle {
        CertificateBundle {
            id: None,
            kid: None,
            sid: None,
            x5t: None,
            policy: None,
            cer: None,
            content_type: None,
            attributes: None,
            tags: None,
        }
    }
}


