# SasDefinitionUpdateParameters

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**template_uri** | Option<**String**> | The SAS definition token template signed with an arbitrary key.  Tokens created according to the SAS definition will have the same properties as the template. | [optional]
**sas_type** | Option<**String**> | The type of SAS token the SAS definition will create. | [optional]
**validity_period** | Option<**String**> | The validity period of SAS tokens created according to the SAS definition. | [optional]
**attributes** | Option<[**crate::models::SasDefinitionAttributes**](SasDefinitionAttributes.md)> |  | [optional]
**tags** | Option<**::std::collections::HashMap<String, String>**> | Application specific metadata in the form of key-value pairs. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


