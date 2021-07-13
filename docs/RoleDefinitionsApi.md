# \RoleDefinitionsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**role_definitions_create_or_update**](RoleDefinitionsApi.md#role_definitions_create_or_update) | **put** /{scope}/providers/Microsoft.Authorization/roleDefinitions/{roleDefinitionName} | 
[**role_definitions_delete**](RoleDefinitionsApi.md#role_definitions_delete) | **delete** /{scope}/providers/Microsoft.Authorization/roleDefinitions/{roleDefinitionName} | 
[**role_definitions_get**](RoleDefinitionsApi.md#role_definitions_get) | **get** /{scope}/providers/Microsoft.Authorization/roleDefinitions/{roleDefinitionName} | 
[**role_definitions_list**](RoleDefinitionsApi.md#role_definitions_list) | **get** /{scope}/providers/Microsoft.Authorization/roleDefinitions | 



## role_definitions_create_or_update

> crate::models::RoleDefinition role_definitions_create_or_update(scope, role_definition_name, api_version, parameters)


Creates or updates a custom role definition.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**scope** | **String** | The scope of the role definition to create or update. Managed HSM only supports '/'. | [required] |
**role_definition_name** | **String** | The name of the role definition to create or update. It can be any valid GUID. | [required] |
**api_version** | **String** | Client API version. | [required] |
**parameters** | [**RoleDefinitionCreateParameters**](RoleDefinitionCreateParameters.md) | Parameters for the role definition. | [required] |

### Return type

[**crate::models::RoleDefinition**](RoleDefinition.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## role_definitions_delete

> crate::models::RoleDefinition role_definitions_delete(scope, role_definition_name, api_version)


Deletes a custom role definition.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**scope** | **String** | The scope of the role definition to delete. Managed HSM only supports '/'. | [required] |
**role_definition_name** | **String** | The name (GUID) of the role definition to delete. | [required] |
**api_version** | **String** | Client API version. | [required] |

### Return type

[**crate::models::RoleDefinition**](RoleDefinition.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## role_definitions_get

> crate::models::RoleDefinition role_definitions_get(scope, role_definition_name, api_version)


Get the specified role definition.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**scope** | **String** | The scope of the role definition to get. Managed HSM only supports '/'. | [required] |
**role_definition_name** | **String** | The name of the role definition to get. | [required] |
**api_version** | **String** | Client API version. | [required] |

### Return type

[**crate::models::RoleDefinition**](RoleDefinition.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## role_definitions_list

> crate::models::RoleDefinitionListResult role_definitions_list(scope, api_version, filter)


Get all role definitions that are applicable at scope and above.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**scope** | **String** | The scope of the role definition. | [required] |
**api_version** | **String** | Client API version. | [required] |
**filter** | Option<**String**> | The filter to apply on the operation. Use atScopeAndBelow filter to search below the given scope as well. |  |

### Return type

[**crate::models::RoleDefinitionListResult**](RoleDefinitionListResult.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

