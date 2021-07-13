/*
 * KeyVaultClient
 *
 * The key vault client performs cryptographic key operations and vault operations against the Key Vault service.
 *
 * The version of the OpenAPI document: 7.2
 * 
 * Generated by: https://openapi-generator.tech
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};


/// struct for typed errors of method `get_deleted_sas_definition`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetDeletedSasDefinitionError {
    DefaultResponse(crate::models::KeyVaultError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `get_deleted_sas_definitions`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetDeletedSasDefinitionsError {
    DefaultResponse(crate::models::KeyVaultError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `get_deleted_storage_account`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetDeletedStorageAccountError {
    DefaultResponse(crate::models::KeyVaultError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `get_deleted_storage_accounts`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetDeletedStorageAccountsError {
    DefaultResponse(crate::models::KeyVaultError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `purge_deleted_storage_account`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PurgeDeletedStorageAccountError {
    DefaultResponse(crate::models::KeyVaultError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `recover_deleted_sas_definition`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RecoverDeletedSasDefinitionError {
    DefaultResponse(crate::models::KeyVaultError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `recover_deleted_storage_account`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RecoverDeletedStorageAccountError {
    DefaultResponse(crate::models::KeyVaultError),
    UnknownValue(serde_json::Value),
}


/// The Get Deleted SAS Definition operation returns the specified deleted SAS definition along with its attributes. This operation requires the storage/getsas permission.
pub async fn get_deleted_sas_definition(configuration: &configuration::Configuration, storage_account_name: &str, sas_definition_name: &str, api_version: &str) -> Result<crate::models::DeletedSasDefinitionBundle, Error<GetDeletedSasDefinitionError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/deletedstorage/{storage_account_name}/sas/{sas_definition_name}", configuration.base_path, storage_account_name=crate::apis::urlencode(storage_account_name), sas_definition_name=crate::apis::urlencode(sas_definition_name));
    let mut local_var_req_builder = local_var_client.get(local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("api_version", &api_version.to_string())]);
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetDeletedSasDefinitionError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// The Get Deleted Sas Definitions operation returns the SAS definitions that have been deleted for a vault enabled for soft-delete. This operation requires the storage/listsas permission.
pub async fn get_deleted_sas_definitions(configuration: &configuration::Configuration, storage_account_name: &str, api_version: &str, maxresults: Option<i32>) -> Result<crate::models::DeletedSasDefinitionListResult, Error<GetDeletedSasDefinitionsError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/deletedstorage/{storage_account_name}/sas", configuration.base_path, storage_account_name=crate::apis::urlencode(storage_account_name));
    let mut local_var_req_builder = local_var_client.get(local_var_uri_str.as_str());

    if let Some(ref local_var_str) = maxresults {
        local_var_req_builder = local_var_req_builder.query(&[("maxresults", &local_var_str.to_string())]);
    }
    local_var_req_builder = local_var_req_builder.query(&[("api_version", &api_version.to_string())]);
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetDeletedSasDefinitionsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// The Get Deleted Storage Account operation returns the specified deleted storage account along with its attributes. This operation requires the storage/get permission.
pub async fn get_deleted_storage_account(configuration: &configuration::Configuration, storage_account_name: &str, api_version: &str) -> Result<crate::models::DeletedStorageBundle, Error<GetDeletedStorageAccountError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/deletedstorage/{storage_account_name}", configuration.base_path, storage_account_name=crate::apis::urlencode(storage_account_name));
    let mut local_var_req_builder = local_var_client.get(local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("api_version", &api_version.to_string())]);
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetDeletedStorageAccountError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// The Get Deleted Storage Accounts operation returns the storage accounts that have been deleted for a vault enabled for soft-delete. This operation requires the storage/list permission.
pub async fn get_deleted_storage_accounts(configuration: &configuration::Configuration, api_version: &str, maxresults: Option<i32>) -> Result<crate::models::DeletedStorageListResult, Error<GetDeletedStorageAccountsError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/deletedstorage", configuration.base_path);
    let mut local_var_req_builder = local_var_client.get(local_var_uri_str.as_str());

    if let Some(ref local_var_str) = maxresults {
        local_var_req_builder = local_var_req_builder.query(&[("maxresults", &local_var_str.to_string())]);
    }
    local_var_req_builder = local_var_req_builder.query(&[("api_version", &api_version.to_string())]);
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetDeletedStorageAccountsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// The purge deleted storage account operation removes the secret permanently, without the possibility of recovery. This operation can only be performed on a soft-delete enabled vault. This operation requires the storage/purge permission.
pub async fn purge_deleted_storage_account(configuration: &configuration::Configuration, storage_account_name: &str, api_version: &str) -> Result<(), Error<PurgeDeletedStorageAccountError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/deletedstorage/{storage_account_name}", configuration.base_path, storage_account_name=crate::apis::urlencode(storage_account_name));
    let mut local_var_req_builder = local_var_client.delete(local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("api_version", &api_version.to_string())]);
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<PurgeDeletedStorageAccountError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Recovers the deleted SAS definition for the specified storage account. This operation can only be performed on a soft-delete enabled vault. This operation requires the storage/recover permission.
pub async fn recover_deleted_sas_definition(configuration: &configuration::Configuration, storage_account_name: &str, sas_definition_name: &str, api_version: &str) -> Result<crate::models::SasDefinitionBundle, Error<RecoverDeletedSasDefinitionError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/deletedstorage/{storage_account_name}/sas/{sas_definition_name}/recover", configuration.base_path, storage_account_name=crate::apis::urlencode(storage_account_name), sas_definition_name=crate::apis::urlencode(sas_definition_name));
    let mut local_var_req_builder = local_var_client.post(local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("api_version", &api_version.to_string())]);
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<RecoverDeletedSasDefinitionError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Recovers the deleted storage account in the specified vault. This operation can only be performed on a soft-delete enabled vault. This operation requires the storage/recover permission.
pub async fn recover_deleted_storage_account(configuration: &configuration::Configuration, storage_account_name: &str, api_version: &str) -> Result<crate::models::StorageBundle, Error<RecoverDeletedStorageAccountError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/deletedstorage/{storage_account_name}/recover", configuration.base_path, storage_account_name=crate::apis::urlencode(storage_account_name));
    let mut local_var_req_builder = local_var_client.post(local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("api_version", &api_version.to_string())]);
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<RecoverDeletedStorageAccountError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

