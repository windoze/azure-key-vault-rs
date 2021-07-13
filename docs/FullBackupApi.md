# \FullBackupApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**full_backup**](FullBackupApi.md#full_backup) | **post** /backup | 
[**full_backup_status**](FullBackupApi.md#full_backup_status) | **get** /backup/{jobId}/pending | 



## full_backup

> crate::models::FullBackupOperation full_backup(api_version, azure_storage_blob_container_uri)


Creates a full backup using a user-provided SAS token to an Azure blob storage container.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | Client API version. | [required] |
**azure_storage_blob_container_uri** | Option<[**SasTokenParameter**](SasTokenParameter.md)> | Azure blob shared access signature token pointing to a valid Azure blob container where full backup needs to be stored. This token needs to be valid for at least next 24 hours from the time of making this call |  |

### Return type

[**crate::models::FullBackupOperation**](FullBackupOperation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## full_backup_status

> crate::models::FullBackupOperation full_backup_status(job_id, api_version)


Returns the status of full backup operation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **String** | The id returned as part of the backup request | [required] |
**api_version** | **String** | Client API version. | [required] |

### Return type

[**crate::models::FullBackupOperation**](FullBackupOperation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

