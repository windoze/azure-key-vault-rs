# \DeletedStorageApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_deleted_sas_definition**](DeletedStorageApi.md#get_deleted_sas_definition) | **get** /deletedstorage/{storage_account_name}/sas/{sas_definition_name} | Gets the specified deleted sas definition.
[**get_deleted_sas_definitions**](DeletedStorageApi.md#get_deleted_sas_definitions) | **get** /deletedstorage/{storage_account_name}/sas | Lists deleted SAS definitions for the specified vault and storage account.
[**get_deleted_storage_account**](DeletedStorageApi.md#get_deleted_storage_account) | **get** /deletedstorage/{storage_account_name} | Gets the specified deleted storage account.
[**get_deleted_storage_accounts**](DeletedStorageApi.md#get_deleted_storage_accounts) | **get** /deletedstorage | Lists deleted storage accounts for the specified vault.
[**purge_deleted_storage_account**](DeletedStorageApi.md#purge_deleted_storage_account) | **delete** /deletedstorage/{storage_account_name} | Permanently deletes the specified storage account.
[**recover_deleted_sas_definition**](DeletedStorageApi.md#recover_deleted_sas_definition) | **post** /deletedstorage/{storage_account_name}/sas/{sas_definition_name}/recover | Recovers the deleted SAS definition.
[**recover_deleted_storage_account**](DeletedStorageApi.md#recover_deleted_storage_account) | **post** /deletedstorage/{storage_account_name}/recover | Recovers the deleted storage account.



## get_deleted_sas_definition

> crate::models::DeletedSasDefinitionBundle get_deleted_sas_definition(storage_account_name, sas_definition_name, api_version)
Gets the specified deleted sas definition.

The Get Deleted SAS Definition operation returns the specified deleted SAS definition along with its attributes. This operation requires the storage/getsas permission.

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


## get_deleted_sas_definitions

> crate::models::DeletedSasDefinitionListResult get_deleted_sas_definitions(storage_account_name, api_version, maxresults)
Lists deleted SAS definitions for the specified vault and storage account.

The Get Deleted Sas Definitions operation returns the SAS definitions that have been deleted for a vault enabled for soft-delete. This operation requires the storage/listsas permission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**storage_account_name** | **String** | The name of the storage account. | [required] |
**api_version** | **String** | Client API version. | [required] |
**maxresults** | Option<**i32**> | Maximum number of results to return in a page. If not specified the service will return up to 25 results. |  |

### Return type

[**crate::models::DeletedSasDefinitionListResult**](DeletedSasDefinitionListResult.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_deleted_storage_account

> crate::models::DeletedStorageBundle get_deleted_storage_account(storage_account_name, api_version)
Gets the specified deleted storage account.

The Get Deleted Storage Account operation returns the specified deleted storage account along with its attributes. This operation requires the storage/get permission.

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


## get_deleted_storage_accounts

> crate::models::DeletedStorageListResult get_deleted_storage_accounts(api_version, maxresults)
Lists deleted storage accounts for the specified vault.

The Get Deleted Storage Accounts operation returns the storage accounts that have been deleted for a vault enabled for soft-delete. This operation requires the storage/list permission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | Client API version. | [required] |
**maxresults** | Option<**i32**> | Maximum number of results to return in a page. If not specified the service will return up to 25 results. |  |

### Return type

[**crate::models::DeletedStorageListResult**](DeletedStorageListResult.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## purge_deleted_storage_account

> purge_deleted_storage_account(storage_account_name, api_version)
Permanently deletes the specified storage account.

The purge deleted storage account operation removes the secret permanently, without the possibility of recovery. This operation can only be performed on a soft-delete enabled vault. This operation requires the storage/purge permission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**storage_account_name** | **String** | The name of the storage account. | [required] |
**api_version** | **String** | Client API version. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## recover_deleted_sas_definition

> crate::models::SasDefinitionBundle recover_deleted_sas_definition(storage_account_name, sas_definition_name, api_version)
Recovers the deleted SAS definition.

Recovers the deleted SAS definition for the specified storage account. This operation can only be performed on a soft-delete enabled vault. This operation requires the storage/recover permission.

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


## recover_deleted_storage_account

> crate::models::StorageBundle recover_deleted_storage_account(storage_account_name, api_version)
Recovers the deleted storage account.

Recovers the deleted storage account in the specified vault. This operation can only be performed on a soft-delete enabled vault. This operation requires the storage/recover permission.

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

