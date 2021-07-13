# StorageAccountCreateParameters

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**resource_id** | **String** | Storage account resource id. | 
**active_key_name** | **String** | Current active storage account key name. | 
**auto_regenerate_key** | **bool** | whether keyvault should manage the storage account for the user. | 
**regeneration_period** | Option<**String**> | The key regeneration time duration specified in ISO-8601 format. | [optional]
**attributes** | Option<[**crate::models::StorageAccountAttributes**](StorageAccountAttributes.md)> |  | [optional]
**tags** | Option<**::std::collections::HashMap<String, String>**> | Application specific metadata in the form of key-value pairs. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


