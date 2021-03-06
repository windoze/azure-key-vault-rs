/*
 * KeyVaultClient
 *
 * The key vault client performs cryptographic key operations and vault operations against the Key Vault service.
 *
 * The version of the OpenAPI document: 7.2
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IssuerCredentials : The credentials to be used for the certificate issuer.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IssuerCredentials {
    /// The user name/account name/account id.
    #[serde(rename = "account_id", skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// The password/secret/account key.
    #[serde(rename = "pwd", skip_serializing_if = "Option::is_none")]
    pub pwd: Option<String>,
}

impl IssuerCredentials {
    /// The credentials to be used for the certificate issuer.
    pub fn new() -> IssuerCredentials {
        IssuerCredentials {
            account_id: None,
            pwd: None,
        }
    }
}


