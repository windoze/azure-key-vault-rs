# KeyBundle

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**key** | Option<[**crate::models::JsonWebKey**](JsonWebKey.md)> |  | [optional]
**attributes** | Option<[**crate::models::KeyAttributes**](KeyAttributes.md)> |  | [optional]
**tags** | Option<**::std::collections::HashMap<String, String>**> | Application specific metadata in the form of key-value pairs. | [optional]
**managed** | Option<**bool**> | True if the key's lifetime is managed by key vault. If this is a key backing a certificate, then managed will be true. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


