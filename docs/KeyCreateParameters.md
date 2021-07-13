# KeyCreateParameters

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**kty** | **String** | The type of key to create. For valid values, see JsonWebKeyType. | 
**key_size** | Option<**i32**> | The key size in bits. For example: 2048, 3072, or 4096 for RSA. | [optional]
**public_exponent** | Option<**i32**> | The public exponent for a RSA key. | [optional]
**key_ops** | Option<**Vec<String>**> |  | [optional]
**attributes** | Option<[**crate::models::KeyAttributes**](KeyAttributes.md)> |  | [optional]
**tags** | Option<**::std::collections::HashMap<String, String>**> | Application specific metadata in the form of key-value pairs. | [optional]
**crv** | Option<**String**> | Elliptic curve name. For valid values, see JsonWebKeyCurveName. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


