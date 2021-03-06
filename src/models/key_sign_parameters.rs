/*
 * KeyVaultClient
 *
 * The key vault client performs cryptographic key operations and vault operations against the Key Vault service.
 *
 * The version of the OpenAPI document: 7.2
 * 
 * Generated by: https://openapi-generator.tech
 */

/// KeySignParameters : The key operations parameters.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KeySignParameters {
    /// The signing/verification algorithm identifier. For more information on possible algorithm types, see JsonWebKeySignatureAlgorithm.
    #[serde(rename = "alg")]
    pub alg: Alg,
    #[serde(rename = "value")]
    pub value: String,
}

impl KeySignParameters {
    /// The key operations parameters.
    pub fn new(alg: Alg, value: String) -> KeySignParameters {
        KeySignParameters {
            alg,
            value,
        }
    }
}

/// The signing/verification algorithm identifier. For more information on possible algorithm types, see JsonWebKeySignatureAlgorithm.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Alg {
    #[serde(rename = "PS256")]
    PS256,
    #[serde(rename = "PS384")]
    PS384,
    #[serde(rename = "PS512")]
    PS512,
    #[serde(rename = "RS256")]
    RS256,
    #[serde(rename = "RS384")]
    RS384,
    #[serde(rename = "RS512")]
    RS512,
    #[serde(rename = "RSNULL")]
    RSNULL,
    #[serde(rename = "ES256")]
    ES256,
    #[serde(rename = "ES384")]
    ES384,
    #[serde(rename = "ES512")]
    ES512,
    #[serde(rename = "ES256K")]
    ES256K,
}

