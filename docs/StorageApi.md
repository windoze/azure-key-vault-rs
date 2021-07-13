# \StorageApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**backup_storage_account**](StorageApi.md#backup_storage_account) | **post** /storage/{storage_account_name}/backup | Backs up the specified storage account.
[**delete_sas_definition**](StorageApi.md#delete_sas_definition) | **delete** /storage/{storage_account_name}/sas/{sas_definition_name} | 
[**delete_storage_account**](StorageApi.md#delete_storage_account) | **delete** /storage/{storage_account_name} | 
[**get_sas_definition**](StorageApi.md#get_sas_definition) | **get** /storage/{storage_account_name}/sas/{sas_definition_name} | 
[**get_sas_definitions**](StorageApi.md#get_sas_definitions) | **get** /storage/{storage_account_name}/sas | 
[**get_storage_account**](StorageApi.md#get_storage_account) | **get** /storage/{storage_account_name} | 
[**get_storage_accounts**](StorageApi.md#get_storage_accounts) | **get** /storage | 
[**regenerate_storage_account_key**](StorageApi.md#regenerate_storage_account_key) | **post** /storage/{storage_account_name}/regeneratekey | 
[**restore_storage_account**](StorageApi.md#restore_storage_account) | **post** /storage/restore | Restores a backed up storage account to a vault.
[**set_sas_definition**](StorageApi.md#set_sas_definition) | **put** /storage/{storage_account_name}/sas/{sas_definition_name} | 
[**set_storage_account**](StorageApi.md#set_storage_account) | **put** /storage/{storage_account_name} | 
[**update_sas_definition**](StorageApi.md#update_sas_definition) | **patch** /storage/{storage_account_name}/sas/{sas_definition_name} | 
[**update_storage_account**](StorageApi.md#update_storage_account) | **patch** /storage/{storage_account_name} | 



## backup_storage_account

> crate::models::BackupStorageResult backup_storage_account(storage_account_name, api_version)
Backs up the specified storage account.

Requests that a backup of the specified storage account be downloaded to the client. This operation requires the storage/backup permission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**storage_account_name** | **String** | The name of the storage account. | [required] |
**api_version** | **String** | Client API version. | [required] |

### Return type

[**crate::models::BackupStorageResult**](BackupStorageResult.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_sas_definition

> crate::models::DeletedSasDefinitionBundle delete_sas_definition(storage_account_name, sas_definition_name, api_version)


Deletes a SAS definition from a specified storage account. This operation requires the storage/deletesas permission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**storage_account_name** | **String** | The name of the storage account. | [required] |
**sas_definition_name** | **String** | The name of the SAS definition. | [required] |
**api_version** | **String** | Client API version. | [required] |

### Return type

[**crate::models::DeletedSasDefinitionBundle**](DeletedSasDefinitionBundle.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_storage_account

> crate::models::DeletedStorageBundle delete_storage_account(storage_account_name, api_version)


Deletes a storage account. This operation requires the storage/delete permission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**storage_account_name** | **String** | The name of the storage account. | [required] |
**api_version** | **String** | Client API version. | [required] |

### Return type

[**crate::models::DeletedStorageBundle**](DeletedStorageBundle.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_sas_definition

> crate::models::SasDefinitionBundle get_sas_definition(storage_account_name, sas_definition_name, api_version)


Gets information about a SAS definition for the specified storage account. This operation requires the storage/getsas permission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**storage_account_name** | **String** | The name of the storage account. | [required] |
**sas_definition_name** | **String** | The name of the SAS definition. | [required] |
**api_version** | **String** | Client API version. | [required] |

### Return type

[**crate::models::SasDefinitionBundle**](SasDefinitionBundle.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_sas_definitions

> crate::models::SasDefinitionListResult get_sas_definitions(storage_account_name, api_version, maxresults)


List storage SAS definitions for the given storage account. This operation requires the storage/listsas permission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**storage_account_name** | **String** | The name of the storage account. | [required] |
**api_version** | **String** | Client API version. | [required] |
**maxresults** | Option<**i32**> | Maximum number of results to return in a page. If not specified the service will return up to 25 results. |  |

### Return type

[**crate::models::SasDefinitionListResult**](SasDefinitionListResult.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_storage_account

> crate::models::StorageBundle get_storage_account(storage_account_name, api_version)


Gets information about a specified storage account. This operation requires the storage/get permission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**storage_account_name** | **String** | The name of the storage account. | [required] |
**api_version** | **String** | Client API version. | [required] |

### Return type

[**crate::models::StorageBundle**](StorageBundle.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_storage_accounts

> crate::models::StorageListResult get_storage_accounts(api_version, maxresults)


List storage accounts managed by the specified key vault. This operation requires the storage/list permission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | Client API version. | [required] |
**maxresults** | Option<**i32**> | Maximum number of results to return in a page. If not specified the service will return up to 25 results. |  |

### Return type

[**crate::models::StorageListResult**](StorageListResult.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## regenerate_storage_account_key

> crate::models::StorageBundle regenerate_storage_account_key(storage_account_name, api_version, parameters)


Regenerates the specified key value for the given storage account. This operation requires the storage/regeneratekey permission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**storage_account_name** | **String** | The name of the storage account. | [required] |
**api_version** | **String** | Client API version. | [required] |
**parameters** | [**StorageAccountRegenerteKeyParameters**](StorageAccountRegenerteKeyParameters.md) | The parameters to regenerate storage account key. | [required] |

### Return type

[**crate::models::StorageBundle**](StorageBundle.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## restore_storage_account

> crate::models::StorageBundle restore_storage_account(api_version, parameters)
Restores a backed up storage account to a vault.

Restores a backed up storage account to a vault. This operation requires the storage/restore permission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | Client API version. | [required] |
**parameters** | [**StorageRestoreParameters**](StorageRestoreParameters.md) | The parameters to restore the storage account. | [required] |

### Return type

[**crate::models::StorageBundle**](StorageBundle.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_sas_definition

> crate::models::SasDefinitionBundle set_sas_definition(storage_account_name, sas_definition_name, api_version, parameters)


Creates or updates a new SAS definition for the specified storage account. This operation requires the storage/setsas permission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**storage_account_name** | **String** | The name of the storage account. | [required] |
**sas_definition_name** | **String** | The name of the SAS definition. | [required] |
**api_version** | **String** | Client API version. | [required] |
**parameters** | [**SasDefinitionCreateParameters**](SasDefinitionCreateParameters.md) | The parameters to create a SAS definition. | [required] |

### Return type

[**crate::models::SasDefinitionBundle**](SasDefinitionBundle.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_storage_account

> crate::models::StorageBundle set_storage_account(storage_account_name, api_version, parameters)


Creates or updates a new storage account. This operation requires the storage/set permission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**storage_account_name** | **String** | The name of the storage account. | [required] |
**api_version** | **String** | Client API version. | [required] |
**parameters** | [**StorageAccountCreateParameters**](StorageAccountCreateParameters.md) | The parameters to create a storage account. | [required] |

### Return type

[**crate::models::StorageBundle**](StorageBundle.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_sas_definition

> crate::models::SasDefinitionBundle update_sas_definition(storage_account_name, sas_definition_name, api_version, parameters)


Updates the specified attributes associated with the given SAS definition. This operation requires the storage/setsas permission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**storage_account_name** | **String** | The name of the storage account. | [required] |
**sas_definition_name** | **String** | The name of the SAS definition. | [required] |
**api_version** | **String** | Client API version. | [required] |
**parameters** | [**SasDefinitionUpdateParameters**](SasDefinitionUpdateParameters.md) | The parameters to update a SAS definition. | [required] |

### Return type

[**crate::models::SasDefinitionBundle**](SasDefinitionBundle.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_storage_account

> crate::models::StorageBundle update_storage_account(storage_account_name, api_version, parameters)


Updates the specified attributes associated with the given storage account. This operation requires the storage/set/update permission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**storage_account_name** | **String** | The name of the storage account. | [required] |
**api_version** | **String** | Client API version. | [required] |
**parameters** | [**StorageAccountUpdateParameters**](StorageAccountUpdateParameters.md) | The parameters to update a storage account. | [required] |

### Return type

[**crate::models::StorageBundle**](StorageBundle.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

