# StorageBundle

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The storage account id. | [optional][readonly]
**resource_id** | Option<**String**> | The storage account resource id. | [optional][readonly]
**active_key_name** | Option<**String**> | The current active storage account key name. | [optional][readonly]
**auto_regenerate_key** | Option<**bool**> | whether keyvault should manage the storage account for the user. | [optional][readonly]
**regeneration_period** | Option<**String**> | The key regeneration time duration specified in ISO-8601 format. | [optional][readonly]
**attributes** | Option<[**crate::models::StorageAccountAttributes**](StorageAccountAttributes.md)> |  | [optional]
**tags** | Option<**::std::collections::HashMap<String, String>**> | Application specific metadata in the form of key-value pairs | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


