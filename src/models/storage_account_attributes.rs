/*
 * KeyVaultClient
 *
 * The key vault client performs cryptographic key operations and vault operations against the Key Vault service.
 *
 * The version of the OpenAPI document: 7.2
 * 
 * Generated by: https://openapi-generator.tech
 */

/// StorageAccountAttributes : The storage account management attributes.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageAccountAttributes {
    /// the enabled state of the object.
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// Creation time in UTC.
    #[serde(rename = "created", skip_serializing_if = "Option::is_none")]
    pub created: Option<i32>,
    /// Last updated time in UTC.
    #[serde(rename = "updated", skip_serializing_if = "Option::is_none")]
    pub updated: Option<i32>,
    /// softDelete data retention days. Value should be >=7 and <=90 when softDelete enabled, otherwise 0.
    #[serde(rename = "recoverableDays", skip_serializing_if = "Option::is_none")]
    pub recoverable_days: Option<i32>,
    /// Reflects the deletion recovery level currently in effect for storage accounts in the current vault. If it contains 'Purgeable' the storage account can be permanently deleted by a privileged user; otherwise, only the system can purge the storage account, at the end of the retention interval.
    #[serde(rename = "recoveryLevel", skip_serializing_if = "Option::is_none")]
    pub recovery_level: Option<RecoveryLevel>,
}

impl StorageAccountAttributes {
    /// The storage account management attributes.
    pub fn new() -> StorageAccountAttributes {
        StorageAccountAttributes {
            enabled: None,
            created: None,
            updated: None,
            recoverable_days: None,
            recovery_level: None,
        }
    }
}

/// Reflects the deletion recovery level currently in effect for storage accounts in the current vault. If it contains 'Purgeable' the storage account can be permanently deleted by a privileged user; otherwise, only the system can purge the storage account, at the end of the retention interval.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RecoveryLevel {
    #[serde(rename = "Purgeable")]
    Purgeable,
    #[serde(rename = "Recoverable+Purgeable")]
    RecoverablePurgeable,
    #[serde(rename = "Recoverable")]
    Recoverable,
    #[serde(rename = "Recoverable+ProtectedSubscription")]
    RecoverableProtectedSubscription,
    #[serde(rename = "CustomizedRecoverable+Purgeable")]
    CustomizedRecoverablePurgeable,
    #[serde(rename = "CustomizedRecoverable")]
    CustomizedRecoverable,
    #[serde(rename = "CustomizedRecoverable+ProtectedSubscription")]
    CustomizedRecoverableProtectedSubscription,
}

