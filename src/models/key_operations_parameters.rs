/*
 * KeyVaultClient
 *
 * The key vault client performs cryptographic key operations and vault operations against the Key Vault service.
 *
 * The version of the OpenAPI document: 7.2
 * 
 * Generated by: https://openapi-generator.tech
 */

/// KeyOperationsParameters : The key operations parameters.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KeyOperationsParameters {
    /// algorithm identifier
    #[serde(rename = "alg")]
    pub alg: Alg,
    #[serde(rename = "value")]
    pub value: String,
    /// Initialization vector for symmetric algorithms.
    #[serde(rename = "iv", skip_serializing_if = "Option::is_none")]
    pub iv: Option<String>,
    /// Additional data to authenticate but not encrypt/decrypt when using authenticated crypto algorithms.
    #[serde(rename = "aad", skip_serializing_if = "Option::is_none")]
    pub aad: Option<String>,
    /// The tag to authenticate when performing decryption with an authenticated algorithm.
    #[serde(rename = "tag", skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

impl KeyOperationsParameters {
    /// The key operations parameters.
    pub fn new(alg: Alg, value: String) -> KeyOperationsParameters {
        KeyOperationsParameters {
            alg,
            value,
            iv: None,
            aad: None,
            tag: None,
        }
    }
}

/// algorithm identifier
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Alg {
    #[serde(rename = "RSA-OAEP")]
    RSAOAEP,
    #[serde(rename = "RSA-OAEP-256")]
    RSAOAEP256,
    #[serde(rename = "RSA1_5")]
    RSA15,
    #[serde(rename = "A128GCM")]
    A128GCM,
    #[serde(rename = "A192GCM")]
    A192GCM,
    #[serde(rename = "A256GCM")]
    A256GCM,
    #[serde(rename = "A128KW")]
    A128KW,
    #[serde(rename = "A192KW")]
    A192KW,
    #[serde(rename = "A256KW")]
    A256KW,
    #[serde(rename = "A128CBC")]
    A128CBC,
    #[serde(rename = "A192CBC")]
    A192CBC,
    #[serde(rename = "A256CBC")]
    A256CBC,
    #[serde(rename = "A128CBCPAD")]
    A128CBCPAD,
    #[serde(rename = "A192CBCPAD")]
    A192CBCPAD,
    #[serde(rename = "A256CBCPAD")]
    A256CBCPAD,
}

