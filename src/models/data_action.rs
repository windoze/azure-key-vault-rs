/*
 * KeyVaultClient
 *
 * The key vault client performs cryptographic key operations and vault operations against the Key Vault service.
 *
 * The version of the OpenAPI document: 7.2
 * 
 * Generated by: https://openapi-generator.tech
 */

/// DataAction : Supported permissions for data actions.

/// Supported permissions for data actions.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum DataAction {
    #[serde(rename = "Microsoft.KeyVault/managedHsm/keys/read/action")]
    KeysReadAction,
    #[serde(rename = "Microsoft.KeyVault/managedHsm/keys/write/action")]
    KeysWriteAction,
    #[serde(rename = "Microsoft.KeyVault/managedHsm/keys/deletedKeys/read/action")]
    KeysDeletedKeysReadAction,
    #[serde(rename = "Microsoft.KeyVault/managedHsm/keys/deletedKeys/recover/action")]
    KeysDeletedKeysRecoverAction,
    #[serde(rename = "Microsoft.KeyVault/managedHsm/keys/backup/action")]
    KeysBackupAction,
    #[serde(rename = "Microsoft.KeyVault/managedHsm/keys/restore/action")]
    KeysRestoreAction,
    #[serde(rename = "Microsoft.KeyVault/managedHsm/roleAssignments/delete/action")]
    RoleAssignmentsDeleteAction,
    #[serde(rename = "Microsoft.KeyVault/managedHsm/roleAssignments/read/action")]
    RoleAssignmentsReadAction,
    #[serde(rename = "Microsoft.KeyVault/managedHsm/roleAssignments/write/action")]
    RoleAssignmentsWriteAction,
    #[serde(rename = "Microsoft.KeyVault/managedHsm/roleDefinitions/read/action")]
    RoleDefinitionsReadAction,
    #[serde(rename = "Microsoft.KeyVault/managedHsm/keys/encrypt/action")]
    KeysEncryptAction,
    #[serde(rename = "Microsoft.KeyVault/managedHsm/keys/decrypt/action")]
    KeysDecryptAction,
    #[serde(rename = "Microsoft.KeyVault/managedHsm/keys/wrap/action")]
    KeysWrapAction,
    #[serde(rename = "Microsoft.KeyVault/managedHsm/keys/unwrap/action")]
    KeysUnwrapAction,
    #[serde(rename = "Microsoft.KeyVault/managedHsm/keys/sign/action")]
    KeysSignAction,
    #[serde(rename = "Microsoft.KeyVault/managedHsm/keys/verify/action")]
    KeysVerifyAction,
    #[serde(rename = "Microsoft.KeyVault/managedHsm/keys/create")]
    KeysCreate,
    #[serde(rename = "Microsoft.KeyVault/managedHsm/keys/delete")]
    KeysDelete,
    #[serde(rename = "Microsoft.KeyVault/managedHsm/keys/export/action")]
    KeysExportAction,
    #[serde(rename = "Microsoft.KeyVault/managedHsm/keys/import/action")]
    KeysImportAction,
    #[serde(rename = "Microsoft.KeyVault/managedHsm/keys/deletedKeys/delete")]
    KeysDeletedKeysDelete,
    #[serde(rename = "Microsoft.KeyVault/managedHsm/securitydomain/download/action")]
    SecuritydomainDownloadAction,
    #[serde(rename = "Microsoft.KeyVault/managedHsm/securitydomain/upload/action")]
    SecuritydomainUploadAction,
    #[serde(rename = "Microsoft.KeyVault/managedHsm/securitydomain/upload/read")]
    SecuritydomainUploadRead,
    #[serde(rename = "Microsoft.KeyVault/managedHsm/securitydomain/transferkey/read")]
    SecuritydomainTransferkeyRead,
    #[serde(rename = "Microsoft.KeyVault/managedHsm/backup/start/action")]
    BackupStartAction,
    #[serde(rename = "Microsoft.KeyVault/managedHsm/restore/start/action")]
    RestoreStartAction,
    #[serde(rename = "Microsoft.KeyVault/managedHsm/backup/status/action")]
    BackupStatusAction,
    #[serde(rename = "Microsoft.KeyVault/managedHsm/restore/status/action")]
    RestoreStatusAction,

}

impl ToString for DataAction {
    fn to_string(&self) -> String {
        match self {
            Self::KeysReadAction => String::from("Microsoft.KeyVault/managedHsm/keys/read/action"),
            Self::KeysWriteAction => String::from("Microsoft.KeyVault/managedHsm/keys/write/action"),
            Self::KeysDeletedKeysReadAction => String::from("Microsoft.KeyVault/managedHsm/keys/deletedKeys/read/action"),
            Self::KeysDeletedKeysRecoverAction => String::from("Microsoft.KeyVault/managedHsm/keys/deletedKeys/recover/action"),
            Self::KeysBackupAction => String::from("Microsoft.KeyVault/managedHsm/keys/backup/action"),
            Self::KeysRestoreAction => String::from("Microsoft.KeyVault/managedHsm/keys/restore/action"),
            Self::RoleAssignmentsDeleteAction => String::from("Microsoft.KeyVault/managedHsm/roleAssignments/delete/action"),
            Self::RoleAssignmentsReadAction => String::from("Microsoft.KeyVault/managedHsm/roleAssignments/read/action"),
            Self::RoleAssignmentsWriteAction => String::from("Microsoft.KeyVault/managedHsm/roleAssignments/write/action"),
            Self::RoleDefinitionsReadAction => String::from("Microsoft.KeyVault/managedHsm/roleDefinitions/read/action"),
            Self::KeysEncryptAction => String::from("Microsoft.KeyVault/managedHsm/keys/encrypt/action"),
            Self::KeysDecryptAction => String::from("Microsoft.KeyVault/managedHsm/keys/decrypt/action"),
            Self::KeysWrapAction => String::from("Microsoft.KeyVault/managedHsm/keys/wrap/action"),
            Self::KeysUnwrapAction => String::from("Microsoft.KeyVault/managedHsm/keys/unwrap/action"),
            Self::KeysSignAction => String::from("Microsoft.KeyVault/managedHsm/keys/sign/action"),
            Self::KeysVerifyAction => String::from("Microsoft.KeyVault/managedHsm/keys/verify/action"),
            Self::KeysCreate => String::from("Microsoft.KeyVault/managedHsm/keys/create"),
            Self::KeysDelete => String::from("Microsoft.KeyVault/managedHsm/keys/delete"),
            Self::KeysExportAction => String::from("Microsoft.KeyVault/managedHsm/keys/export/action"),
            Self::KeysImportAction => String::from("Microsoft.KeyVault/managedHsm/keys/import/action"),
            Self::KeysDeletedKeysDelete => String::from("Microsoft.KeyVault/managedHsm/keys/deletedKeys/delete"),
            Self::SecuritydomainDownloadAction => String::from("Microsoft.KeyVault/managedHsm/securitydomain/download/action"),
            Self::SecuritydomainUploadAction => String::from("Microsoft.KeyVault/managedHsm/securitydomain/upload/action"),
            Self::SecuritydomainUploadRead => String::from("Microsoft.KeyVault/managedHsm/securitydomain/upload/read"),
            Self::SecuritydomainTransferkeyRead => String::from("Microsoft.KeyVault/managedHsm/securitydomain/transferkey/read"),
            Self::BackupStartAction => String::from("Microsoft.KeyVault/managedHsm/backup/start/action"),
            Self::RestoreStartAction => String::from("Microsoft.KeyVault/managedHsm/restore/start/action"),
            Self::BackupStatusAction => String::from("Microsoft.KeyVault/managedHsm/backup/status/action"),
            Self::RestoreStatusAction => String::from("Microsoft.KeyVault/managedHsm/restore/status/action"),
        }
    }
}




