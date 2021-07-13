# SasDefinitionBundle

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The SAS definition id. | [optional][readonly]
**sid** | Option<**String**> | Storage account SAS definition secret id. | [optional][readonly]
**template_uri** | Option<**String**> | The SAS definition token template signed with an arbitrary key.  Tokens created according to the SAS definition will have the same properties as the template. | [optional][readonly]
**sas_type** | Option<**String**> | The type of SAS token the SAS definition will create. | [optional][readonly]
**validity_period** | Option<**String**> | The validity period of SAS tokens created according to the SAS definition. | [optional][readonly]
**attributes** | Option<[**crate::models::SasDefinitionAttributes**](SasDefinitionAttributes.md)> |  | [optional]
**tags** | Option<**::std::collections::HashMap<String, String>**> | Application specific metadata in the form of key-value pairs | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


