# \RoleAssignmentsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**role_assignments_create**](RoleAssignmentsApi.md#role_assignments_create) | **put** /{scope}/providers/Microsoft.Authorization/roleAssignments/{roleAssignmentName} | 
[**role_assignments_delete**](RoleAssignmentsApi.md#role_assignments_delete) | **delete** /{scope}/providers/Microsoft.Authorization/roleAssignments/{roleAssignmentName} | 
[**role_assignments_get**](RoleAssignmentsApi.md#role_assignments_get) | **get** /{scope}/providers/Microsoft.Authorization/roleAssignments/{roleAssignmentName} | 
[**role_assignments_list_for_scope**](RoleAssignmentsApi.md#role_assignments_list_for_scope) | **get** /{scope}/providers/Microsoft.Authorization/roleAssignments | 



## role_assignments_create

> crate::models::RoleAssignment role_assignments_create(scope, role_assignment_name, api_version, parameters)


Creates a role assignment.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**scope** | **String** | The scope of the role assignment to create. | [required] |
**role_assignment_name** | **String** | The name of the role assignment to create. It can be any valid GUID. | [required] |
**api_version** | **String** | Client API version. | [required] |
**parameters** | [**RoleAssignmentCreateParameters**](RoleAssignmentCreateParameters.md) | Parameters for the role assignment. | [required] |

### Return type

[**crate::models::RoleAssignment**](RoleAssignment.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## role_assignments_delete

> crate::models::RoleAssignment role_assignments_delete(scope, role_assignment_name, api_version)


Deletes a role assignment.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**scope** | **String** | The scope of the role assignment to delete. | [required] |
**role_assignment_name** | **String** | The name of the role assignment to delete. | [required] |
**api_version** | **String** | Client API version. | [required] |

### Return type

[**crate::models::RoleAssignment**](RoleAssignment.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## role_assignments_get

> crate::models::RoleAssignment role_assignments_get(scope, role_assignment_name, api_version)


Get the specified role assignment.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**scope** | **String** | The scope of the role assignment. | [required] |
**role_assignment_name** | **String** | The name of the role assignment to get. | [required] |
**api_version** | **String** | Client API version. | [required] |

### Return type

[**crate::models::RoleAssignment**](RoleAssignment.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## role_assignments_list_for_scope

> crate::models::RoleAssignmentListResult role_assignments_list_for_scope(scope, api_version, filter)


Gets role assignments for a scope.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**scope** | **String** | The scope of the role assignments. | [required] |
**api_version** | **String** | Client API version. | [required] |
**filter** | Option<**String**> | The filter to apply on the operation. Use $filter=atScope() to return all role assignments at or above the scope. Use $filter=principalId eq {id} to return all role assignments at, above or below the scope for the specified principal. |  |

### Return type

[**crate::models::RoleAssignmentListResult**](RoleAssignmentListResult.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

