# DeletedStorageBundle

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**recovery_id** | Option<**String**> | The url of the recovery object, used to identify and recover the deleted storage account. | [optional]
**scheduled_purge_date** | Option<**i32**> | The time when the storage account is scheduled to be purged, in UTC | [optional][readonly]
**deleted_date** | Option<**i32**> | The time when the storage account was deleted, in UTC | [optional][readonly]
**id** | Option<**String**> | The storage account id. | [optional][readonly]
**resource_id** | Option<**String**> | The storage account resource id. | [optional][readonly]
**active_key_name** | Option<**String**> | The current active storage account key name. | [optional][readonly]
**auto_regenerate_key** | Option<**bool**> | whether keyvault should manage the storage account for the user. | [optional][readonly]
**regeneration_period** | Option<**String**> | The key regeneration time duration specified in ISO-8601 format. | [optional][readonly]
**attributes** | Option<[**crate::models::StorageAccountAttributes**](StorageAccountAttributes.md)> |  | [optional]
**tags** | Option<**::std::collections::HashMap<String, String>**> | Application specific metadata in the form of key-value pairs | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


