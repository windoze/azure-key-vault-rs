/*
 * KeyVaultClient
 *
 * The key vault client performs cryptographic key operations and vault operations against the Key Vault service.
 *
 * The version of the OpenAPI document: 7.2
 * 
 * Generated by: https://openapi-generator.tech
 */

/// DeletedSasDefinitionBundle : A deleted SAS definition bundle consisting of its previous id, attributes and its tags, as well as information on when it will be purged.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeletedSasDefinitionBundle {
    /// The url of the recovery object, used to identify and recover the deleted SAS definition.
    #[serde(rename = "recoveryId", skip_serializing_if = "Option::is_none")]
    pub recovery_id: Option<String>,
    /// The time when the SAS definition is scheduled to be purged, in UTC
    #[serde(rename = "scheduledPurgeDate", skip_serializing_if = "Option::is_none")]
    pub scheduled_purge_date: Option<i32>,
    /// The time when the SAS definition was deleted, in UTC
    #[serde(rename = "deletedDate", skip_serializing_if = "Option::is_none")]
    pub deleted_date: Option<i32>,
    /// The SAS definition id.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Storage account SAS definition secret id.
    #[serde(rename = "sid", skip_serializing_if = "Option::is_none")]
    pub sid: Option<String>,
    /// The SAS definition token template signed with an arbitrary key.  Tokens created according to the SAS definition will have the same properties as the template.
    #[serde(rename = "templateUri", skip_serializing_if = "Option::is_none")]
    pub template_uri: Option<String>,
    /// The type of SAS token the SAS definition will create.
    #[serde(rename = "sasType", skip_serializing_if = "Option::is_none")]
    pub sas_type: Option<SasType>,
    /// The validity period of SAS tokens created according to the SAS definition.
    #[serde(rename = "validityPeriod", skip_serializing_if = "Option::is_none")]
    pub validity_period: Option<String>,
    #[serde(rename = "attributes", skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Box<crate::models::SasDefinitionAttributes>>,
    /// Application specific metadata in the form of key-value pairs
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

impl DeletedSasDefinitionBundle {
    /// A deleted SAS definition bundle consisting of its previous id, attributes and its tags, as well as information on when it will be purged.
    pub fn new() -> DeletedSasDefinitionBundle {
        DeletedSasDefinitionBundle {
            recovery_id: None,
            scheduled_purge_date: None,
            deleted_date: None,
            id: None,
            sid: None,
            template_uri: None,
            sas_type: None,
            validity_period: None,
            attributes: None,
            tags: None,
        }
    }
}

/// The type of SAS token the SAS definition will create.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SasType {
    #[serde(rename = "account")]
    Account,
    #[serde(rename = "service")]
    Service,
}
