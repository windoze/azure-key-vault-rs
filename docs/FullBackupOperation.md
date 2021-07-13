# FullBackupOperation

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**status** | Option<**String**> | Status of the backup operation. | [optional]
**status_details** | Option<**String**> | The status details of backup operation. | [optional]
**error** | Option<[**crate::models::Error**](Error.md)> |  | [optional]
**start_time** | Option<**i32**> | The start time of the backup operation in UTC | [optional]
**end_time** | Option<**i32**> | The end time of the backup operation in UTC | [optional]
**job_id** | Option<**String**> | Identifier for the full backup operation. | [optional]
**azure_storage_blob_container_uri** | Option<**String**> | The Azure blob storage container Uri which contains the full backup | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


