/*
 * KeyVaultClient
 *
 * The key vault client performs cryptographic key operations and vault operations against the Key Vault service.
 *
 * The version of the OpenAPI document: 7.2
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CertificateRestoreParameters : The certificate restore parameters.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CertificateRestoreParameters {
    /// The backup blob associated with a certificate bundle.
    #[serde(rename = "value")]
    pub value: String,
}

impl CertificateRestoreParameters {
    /// The certificate restore parameters.
    pub fn new(value: String) -> CertificateRestoreParameters {
        CertificateRestoreParameters {
            value,
        }
    }
}


