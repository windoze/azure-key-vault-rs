# \DeletedKeysApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_deleted_key**](DeletedKeysApi.md#get_deleted_key) | **get** /deletedkeys/{key_name} | Gets the public part of a deleted key.
[**get_deleted_keys**](DeletedKeysApi.md#get_deleted_keys) | **get** /deletedkeys | Lists the deleted keys in the specified vault.
[**purge_deleted_key**](DeletedKeysApi.md#purge_deleted_key) | **delete** /deletedkeys/{key_name} | Permanently deletes the specified key.
[**recover_deleted_key**](DeletedKeysApi.md#recover_deleted_key) | **post** /deletedkeys/{key_name}/recover | Recovers the deleted key to its latest version.



## get_deleted_key

> crate::models::DeletedKeyBundle get_deleted_key(key_name, api_version)
Gets the public part of a deleted key.

The Get Deleted Key operation is applicable for soft-delete enabled vaults. While the operation can be invoked on any vault, it will return an error if invoked on a non soft-delete enabled vault. This operation requires the keys/get permission. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key_name** | **String** | The name of the key. | [required] |
**api_version** | **String** | Client API version. | [required] |

### Return type

[**crate::models::DeletedKeyBundle**](DeletedKeyBundle.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_deleted_keys

> crate::models::DeletedKeyListResult get_deleted_keys(api_version, maxresults)
Lists the deleted keys in the specified vault.

Retrieves a list of the keys in the Key Vault as JSON Web Key structures that contain the public part of a deleted key. This operation includes deletion-specific information. The Get Deleted Keys operation is applicable for vaults enabled for soft-delete. While the operation can be invoked on any vault, it will return an error if invoked on a non soft-delete enabled vault. This operation requires the keys/list permission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | Client API version. | [required] |
**maxresults** | Option<**i32**> | Maximum number of results to return in a page. If not specified the service will return up to 25 results. |  |

### Return type

[**crate::models::DeletedKeyListResult**](DeletedKeyListResult.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## purge_deleted_key

> purge_deleted_key(key_name, api_version)
Permanently deletes the specified key.

The Purge Deleted Key operation is applicable for soft-delete enabled vaults. While the operation can be invoked on any vault, it will return an error if invoked on a non soft-delete enabled vault. This operation requires the keys/purge permission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key_name** | **String** | The name of the key | [required] |
**api_version** | **String** | Client API version. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## recover_deleted_key

> crate::models::KeyBundle recover_deleted_key(key_name, api_version)
Recovers the deleted key to its latest version.

The Recover Deleted Key operation is applicable for deleted keys in soft-delete enabled vaults. It recovers the deleted key back to its latest version under /keys. An attempt to recover an non-deleted key will return an error. Consider this the inverse of the delete operation on soft-delete enabled vaults. This operation requires the keys/recover permission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key_name** | **String** | The name of the deleted key. | [required] |
**api_version** | **String** | Client API version. | [required] |

### Return type

[**crate::models::KeyBundle**](KeyBundle.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

