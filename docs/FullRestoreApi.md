# \FullRestoreApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**full_restore_operation**](FullRestoreApi.md#full_restore_operation) | **put** /restore | 
[**restore_status**](FullRestoreApi.md#restore_status) | **get** /restore/{jobId}/pending | 



## full_restore_operation

> crate::models::RestoreOperation full_restore_operation(api_version, restore_blob_details)


Restores all key materials using the SAS token pointing to a previously stored Azure Blob storage backup folder

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | Client API version. | [required] |
**restore_blob_details** | Option<[**RestoreOperationParameters**](RestoreOperationParameters.md)> | The Azure blob SAS token pointing to a folder where the previous successful full backup was stored |  |

### Return type

[**crate::models::RestoreOperation**](RestoreOperation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## restore_status

> crate::models::RestoreOperation restore_status(job_id, api_version)


Returns the status of restore operation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **String** | The Job Id returned part of the restore operation | [required] |
**api_version** | **String** | Client API version. | [required] |

### Return type

[**crate::models::RestoreOperation**](RestoreOperation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

