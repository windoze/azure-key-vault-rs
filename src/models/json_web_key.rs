/*
 * KeyVaultClient
 *
 * The key vault client performs cryptographic key operations and vault operations against the Key Vault service.
 *
 * The version of the OpenAPI document: 7.2
 * 
 * Generated by: https://openapi-generator.tech
 */

/// JsonWebKey : As of http://tools.ietf.org/html/draft-ietf-jose-json-web-key-18



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JsonWebKey {
    /// Key identifier.
    #[serde(rename = "kid", skip_serializing_if = "Option::is_none")]
    pub kid: Option<String>,
    /// JsonWebKey Key Type (kty), as defined in https://tools.ietf.org/html/draft-ietf-jose-json-web-algorithms-40.
    #[serde(rename = "kty", skip_serializing_if = "Option::is_none")]
    pub kty: Option<Kty>,
    #[serde(rename = "key_ops", skip_serializing_if = "Option::is_none")]
    pub key_ops: Option<Vec<String>>,
    /// RSA modulus.
    #[serde(rename = "n", skip_serializing_if = "Option::is_none")]
    pub n: Option<String>,
    /// RSA public exponent.
    #[serde(rename = "e", skip_serializing_if = "Option::is_none")]
    pub e: Option<String>,
    /// RSA private exponent, or the D component of an EC private key.
    #[serde(rename = "d", skip_serializing_if = "Option::is_none")]
    pub d: Option<String>,
    /// RSA private key parameter.
    #[serde(rename = "dp", skip_serializing_if = "Option::is_none")]
    pub dp: Option<String>,
    /// RSA private key parameter.
    #[serde(rename = "dq", skip_serializing_if = "Option::is_none")]
    pub dq: Option<String>,
    /// RSA private key parameter.
    #[serde(rename = "qi", skip_serializing_if = "Option::is_none")]
    pub qi: Option<String>,
    /// RSA secret prime.
    #[serde(rename = "p", skip_serializing_if = "Option::is_none")]
    pub p: Option<String>,
    /// RSA secret prime, with p < q.
    #[serde(rename = "q", skip_serializing_if = "Option::is_none")]
    pub q: Option<String>,
    /// Symmetric key.
    #[serde(rename = "k", skip_serializing_if = "Option::is_none")]
    pub k: Option<String>,
    /// Protected Key, used with 'Bring Your Own Key'.
    #[serde(rename = "key_hsm", skip_serializing_if = "Option::is_none")]
    pub key_hsm: Option<String>,
    /// Elliptic curve name. For valid values, see JsonWebKeyCurveName.
    #[serde(rename = "crv", skip_serializing_if = "Option::is_none")]
    pub crv: Option<Crv>,
    /// X component of an EC public key.
    #[serde(rename = "x", skip_serializing_if = "Option::is_none")]
    pub x: Option<String>,
    /// Y component of an EC public key.
    #[serde(rename = "y", skip_serializing_if = "Option::is_none")]
    pub y: Option<String>,
}

impl JsonWebKey {
    /// As of http://tools.ietf.org/html/draft-ietf-jose-json-web-key-18
    pub fn new() -> JsonWebKey {
        JsonWebKey {
            kid: None,
            kty: None,
            key_ops: None,
            n: None,
            e: None,
            d: None,
            dp: None,
            dq: None,
            qi: None,
            p: None,
            q: None,
            k: None,
            key_hsm: None,
            crv: None,
            x: None,
            y: None,
        }
    }
}

/// JsonWebKey Key Type (kty), as defined in https://tools.ietf.org/html/draft-ietf-jose-json-web-algorithms-40.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Kty {
    #[serde(rename = "EC")]
    EC,
    #[serde(rename = "EC-HSM")]
    ECHSM,
    #[serde(rename = "RSA")]
    RSA,
    #[serde(rename = "RSA-HSM")]
    RSAHSM,
    #[serde(rename = "oct")]
    Oct,
    #[serde(rename = "oct-HSM")]
    OctHSM,
}
/// Elliptic curve name. For valid values, see JsonWebKeyCurveName.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Crv {
    #[serde(rename = "P-256")]
    _256,
    #[serde(rename = "P-384")]
    _384,
    #[serde(rename = "P-521")]
    _521,
    #[serde(rename = "P-256K")]
    _256K,
}
