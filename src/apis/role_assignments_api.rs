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


/// struct for typed errors of method `role_assignments_create`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RoleAssignmentsCreateError {
    DefaultResponse(crate::models::KeyVaultError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `role_assignments_delete`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RoleAssignmentsDeleteError {
    DefaultResponse(crate::models::KeyVaultError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `role_assignments_get`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RoleAssignmentsGetError {
    DefaultResponse(crate::models::KeyVaultError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `role_assignments_list_for_scope`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RoleAssignmentsListForScopeError {
    DefaultResponse(crate::models::KeyVaultError),
    UnknownValue(serde_json::Value),
}


/// Creates a role assignment.
pub async fn role_assignments_create(configuration: &configuration::Configuration, scope: &str, role_assignment_name: &str, api_version: &str, parameters: crate::models::RoleAssignmentCreateParameters) -> Result<crate::models::RoleAssignment, Error<RoleAssignmentsCreateError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/{scope}/providers/Microsoft.Authorization/roleAssignments/{roleAssignmentName}", configuration.base_path, scope=crate::apis::urlencode(scope), roleAssignmentName=crate::apis::urlencode(role_assignment_name));
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
        let local_var_entity: Option<RoleAssignmentsCreateError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Deletes a role assignment.
pub async fn role_assignments_delete(configuration: &configuration::Configuration, scope: &str, role_assignment_name: &str, api_version: &str) -> Result<crate::models::RoleAssignment, Error<RoleAssignmentsDeleteError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/{scope}/providers/Microsoft.Authorization/roleAssignments/{roleAssignmentName}", configuration.base_path, scope=crate::apis::urlencode(scope), roleAssignmentName=crate::apis::urlencode(role_assignment_name));
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
        let local_var_entity: Option<RoleAssignmentsDeleteError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Get the specified role assignment.
pub async fn role_assignments_get(configuration: &configuration::Configuration, scope: &str, role_assignment_name: &str, api_version: &str) -> Result<crate::models::RoleAssignment, Error<RoleAssignmentsGetError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/{scope}/providers/Microsoft.Authorization/roleAssignments/{roleAssignmentName}", configuration.base_path, scope=crate::apis::urlencode(scope), roleAssignmentName=crate::apis::urlencode(role_assignment_name));
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
        let local_var_entity: Option<RoleAssignmentsGetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Gets role assignments for a scope.
pub async fn role_assignments_list_for_scope(configuration: &configuration::Configuration, scope: &str, api_version: &str, filter: Option<&str>) -> Result<crate::models::RoleAssignmentListResult, Error<RoleAssignmentsListForScopeError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/{scope}/providers/Microsoft.Authorization/roleAssignments", configuration.base_path, scope=crate::apis::urlencode(scope));
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
        let local_var_entity: Option<RoleAssignmentsListForScopeError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

