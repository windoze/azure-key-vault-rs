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


/// struct for typed errors of method `role_definitions_create_or_update`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RoleDefinitionsCreateOrUpdateError {
    DefaultResponse(crate::models::KeyVaultError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `role_definitions_delete`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RoleDefinitionsDeleteError {
    DefaultResponse(crate::models::KeyVaultError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `role_definitions_get`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RoleDefinitionsGetError {
    DefaultResponse(crate::models::KeyVaultError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `role_definitions_list`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RoleDefinitionsListError {
    DefaultResponse(crate::models::KeyVaultError),
    UnknownValue(serde_json::Value),
}


/// Creates or updates a custom role definition.
pub async fn role_definitions_create_or_update(configuration: &configuration::Configuration, scope: &str, role_definition_name: &str, api_version: &str, parameters: crate::models::RoleDefinitionCreateParameters) -> Result<crate::models::RoleDefinition, Error<RoleDefinitionsCreateOrUpdateError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/{scope}/providers/Microsoft.Authorization/roleDefinitions/{roleDefinitionName}", configuration.base_path, scope=crate::apis::urlencode(scope), roleDefinitionName=crate::apis::urlencode(role_definition_name));
    let mut local_var_req_builder = local_var_client.put(local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("api_version", &api_version.to_string())]);
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&parameters);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<RoleDefinitionsCreateOrUpdateError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Deletes a custom role definition.
pub async fn role_definitions_delete(configuration: &configuration::Configuration, scope: &str, role_definition_name: &str, api_version: &str) -> Result<crate::models::RoleDefinition, Error<RoleDefinitionsDeleteError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/{scope}/providers/Microsoft.Authorization/roleDefinitions/{roleDefinitionName}", configuration.base_path, scope=crate::apis::urlencode(scope), roleDefinitionName=crate::apis::urlencode(role_definition_name));
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
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<RoleDefinitionsDeleteError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Get the specified role definition.
pub async fn role_definitions_get(configuration: &configuration::Configuration, scope: &str, role_definition_name: &str, api_version: &str) -> Result<crate::models::RoleDefinition, Error<RoleDefinitionsGetError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/{scope}/providers/Microsoft.Authorization/roleDefinitions/{roleDefinitionName}", configuration.base_path, scope=crate::apis::urlencode(scope), roleDefinitionName=crate::apis::urlencode(role_definition_name));
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
        let local_var_entity: Option<RoleDefinitionsGetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Get all role definitions that are applicable at scope and above.
pub async fn role_definitions_list(configuration: &configuration::Configuration, scope: &str, api_version: &str, filter: Option<&str>) -> Result<crate::models::RoleDefinitionListResult, Error<RoleDefinitionsListError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/{scope}/providers/Microsoft.Authorization/roleDefinitions", configuration.base_path, scope=crate::apis::urlencode(scope));
    let mut local_var_req_builder = local_var_client.get(local_var_uri_str.as_str());

    if let Some(ref local_var_str) = filter {
        local_var_req_builder = local_var_req_builder.query(&[("$filter", &local_var_str.to_string())]);
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
        let local_var_entity: Option<RoleDefinitionsListError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

