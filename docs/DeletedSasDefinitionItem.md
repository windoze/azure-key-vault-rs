# DeletedSasDefinitionItem

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**recovery_id** | Option<**String**> | The url of the recovery object, used to identify and recover the deleted SAS definition. | [optional]
**scheduled_purge_date** | Option<**i32**> | The time when the SAS definition is scheduled to be purged, in UTC | [optional][readonly]
**deleted_date** | Option<**i32**> | The time when the SAS definition was deleted, in UTC | [optional][readonly]
**id** | Option<**String**> | The storage SAS identifier. | [optional][readonly]
**sid** | Option<**String**> | The storage account SAS definition secret id. | [optional][readonly]
**attributes** | Option<[**crate::models::SasDefinitionAttributes**](SasDefinitionAttributes.md)> |  | [optional]
**tags** | Option<**::std::collections::HashMap<String, String>**> | Application specific metadata in the form of key-value pairs. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


