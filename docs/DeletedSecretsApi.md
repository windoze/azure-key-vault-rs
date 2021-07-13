# \DeletedSecretsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_deleted_secret**](DeletedSecretsApi.md#get_deleted_secret) | **get** /deletedsecrets/{secret_name} | Gets the specified deleted secret.
[**get_deleted_secrets**](DeletedSecretsApi.md#get_deleted_secrets) | **get** /deletedsecrets | Lists deleted secrets for the specified vault.
[**purge_deleted_secret**](DeletedSecretsApi.md#purge_deleted_secret) | **delete** /deletedsecrets/{secret_name} | Permanently deletes the specified secret.
[**recover_deleted_secret**](DeletedSecretsApi.md#recover_deleted_secret) | **post** /deletedsecrets/{secret_name}/recover | Recovers the deleted secret to the latest version.



## get_deleted_secret

> crate::models::DeletedSecretBundle get_deleted_secret(secret_name, api_version)
Gets the specified deleted secret.

The Get Deleted Secret operation returns the specified deleted secret along with its attributes. This operation requires the secrets/get permission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**secret_name** | **String** | The name of the secret. | [required] |
**api_version** | **String** | Client API version. | [required] |

### Return type

[**crate::models::DeletedSecretBundle**](DeletedSecretBundle.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_deleted_secrets

> crate::models::DeletedSecretListResult get_deleted_secrets(api_version, maxresults)
Lists deleted secrets for the specified vault.

The Get Deleted Secrets operation returns the secrets that have been deleted for a vault enabled for soft-delete. This operation requires the secrets/list permission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | Client API version. | [required] |
**maxresults** | Option<**i32**> | Maximum number of results to return in a page. If not specified the service will return up to 25 results. |  |

### Return type

[**crate::models::DeletedSecretListResult**](DeletedSecretListResult.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## purge_deleted_secret

> purge_deleted_secret(secret_name, api_version)
Permanently deletes the specified secret.

The purge deleted secret operation removes the secret permanently, without the possibility of recovery. This operation can only be enabled on a soft-delete enabled vault. This operation requires the secrets/purge permission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**secret_name** | **String** | The name of the secret. | [required] |
**api_version** | **String** | Client API version. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## recover_deleted_secret

> crate::models::SecretBundle recover_deleted_secret(secret_name, api_version)
Recovers the deleted secret to the latest version.

Recovers the deleted secret in the specified vault. This operation can only be performed on a soft-delete enabled vault. This operation requires the secrets/recover permission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**secret_name** | **String** | The name of the deleted secret. | [required] |
**api_version** | **String** | Client API version. | [required] |

### Return type

[**crate::models::SecretBundle**](SecretBundle.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

