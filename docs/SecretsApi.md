# \SecretsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**backup_secret**](SecretsApi.md#backup_secret) | **post** /secrets/{secret_name}/backup | Backs up the specified secret.
[**delete_secret**](SecretsApi.md#delete_secret) | **delete** /secrets/{secret_name} | Deletes a secret from a specified key vault.
[**get_secret**](SecretsApi.md#get_secret) | **get** /secrets/{secret_name}/{secret_version} | Get a specified secret from a given key vault.
[**get_secret_versions**](SecretsApi.md#get_secret_versions) | **get** /secrets/{secret_name}/versions | List all versions of the specified secret.
[**get_secrets**](SecretsApi.md#get_secrets) | **get** /secrets | List secrets in a specified key vault.
[**restore_secret**](SecretsApi.md#restore_secret) | **post** /secrets/restore | Restores a backed up secret to a vault.
[**set_secret**](SecretsApi.md#set_secret) | **put** /secrets/{secret_name} | Sets a secret in a specified key vault.
[**update_secret**](SecretsApi.md#update_secret) | **patch** /secrets/{secret_name}/{secret_version} | Updates the attributes associated with a specified secret in a given key vault.



## backup_secret

> crate::models::BackupSecretResult backup_secret(secret_name, api_version)
Backs up the specified secret.

Requests that a backup of the specified secret be downloaded to the client. All versions of the secret will be downloaded. This operation requires the secrets/backup permission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**secret_name** | **String** | The name of the secret. | [required] |
**api_version** | **String** | Client API version. | [required] |

### Return type

[**crate::models::BackupSecretResult**](BackupSecretResult.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_secret

> crate::models::DeletedSecretBundle delete_secret(secret_name, api_version)
Deletes a secret from a specified key vault.

The DELETE operation applies to any secret stored in Azure Key Vault. DELETE cannot be applied to an individual version of a secret. This operation requires the secrets/delete permission.

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


## get_secret

> crate::models::SecretBundle get_secret(secret_name, secret_version, api_version)
Get a specified secret from a given key vault.

The GET operation is applicable to any secret stored in Azure Key Vault. This operation requires the secrets/get permission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**secret_name** | **String** | The name of the secret. | [required] |
**secret_version** | **String** | The version of the secret. This URI fragment is optional. If not specified, the latest version of the secret is returned. | [required] |
**api_version** | **String** | Client API version. | [required] |

### Return type

[**crate::models::SecretBundle**](SecretBundle.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_secret_versions

> crate::models::SecretListResult get_secret_versions(secret_name, api_version, maxresults)
List all versions of the specified secret.

The full secret identifier and attributes are provided in the response. No values are returned for the secrets. This operations requires the secrets/list permission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**secret_name** | **String** | The name of the secret. | [required] |
**api_version** | **String** | Client API version. | [required] |
**maxresults** | Option<**i32**> | Maximum number of results to return in a page. If not specified, the service will return up to 25 results. |  |

### Return type

[**crate::models::SecretListResult**](SecretListResult.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_secrets

> crate::models::SecretListResult get_secrets(api_version, maxresults)
List secrets in a specified key vault.

The Get Secrets operation is applicable to the entire vault. However, only the base secret identifier and its attributes are provided in the response. Individual secret versions are not listed in the response. This operation requires the secrets/list permission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | Client API version. | [required] |
**maxresults** | Option<**i32**> | Maximum number of results to return in a page. If not specified, the service will return up to 25 results. |  |

### Return type

[**crate::models::SecretListResult**](SecretListResult.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## restore_secret

> crate::models::SecretBundle restore_secret(api_version, parameters)
Restores a backed up secret to a vault.

Restores a backed up secret, and all its versions, to a vault. This operation requires the secrets/restore permission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | Client API version. | [required] |
**parameters** | [**SecretRestoreParameters**](SecretRestoreParameters.md) | The parameters to restore the secret. | [required] |

### Return type

[**crate::models::SecretBundle**](SecretBundle.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_secret

> crate::models::SecretBundle set_secret(secret_name, api_version, parameters)
Sets a secret in a specified key vault.

 The SET operation adds a secret to the Azure Key Vault. If the named secret already exists, Azure Key Vault creates a new version of that secret. This operation requires the secrets/set permission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**secret_name** | **String** | The name of the secret. | [required] |
**api_version** | **String** | Client API version. | [required] |
**parameters** | [**SecretSetParameters**](SecretSetParameters.md) | The parameters for setting the secret. | [required] |

### Return type

[**crate::models::SecretBundle**](SecretBundle.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_secret

> crate::models::SecretBundle update_secret(secret_name, secret_version, api_version, parameters)
Updates the attributes associated with a specified secret in a given key vault.

The UPDATE operation changes specified attributes of an existing stored secret. Attributes that are not specified in the request are left unchanged. The value of a secret itself cannot be changed. This operation requires the secrets/set permission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**secret_name** | **String** | The name of the secret. | [required] |
**secret_version** | **String** | The version of the secret. | [required] |
**api_version** | **String** | Client API version. | [required] |
**parameters** | [**SecretUpdateParameters**](SecretUpdateParameters.md) | The parameters for update secret operation. | [required] |

### Return type

[**crate::models::SecretBundle**](SecretBundle.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

