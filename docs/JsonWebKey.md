# JsonWebKey

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**kid** | Option<**String**> | Key identifier. | [optional]
**kty** | Option<**String**> | JsonWebKey Key Type (kty), as defined in https://tools.ietf.org/html/draft-ietf-jose-json-web-algorithms-40. | [optional]
**key_ops** | Option<**Vec<String>**> |  | [optional]
**n** | Option<**String**> | RSA modulus. | [optional]
**e** | Option<**String**> | RSA public exponent. | [optional]
**d** | Option<**String**> | RSA private exponent, or the D component of an EC private key. | [optional]
**dp** | Option<**String**> | RSA private key parameter. | [optional]
**dq** | Option<**String**> | RSA private key parameter. | [optional]
**qi** | Option<**String**> | RSA private key parameter. | [optional]
**p** | Option<**String**> | RSA secret prime. | [optional]
**q** | Option<**String**> | RSA secret prime, with p < q. | [optional]
**k** | Option<**String**> | Symmetric key. | [optional]
**key_hsm** | Option<**String**> | Protected Key, used with 'Bring Your Own Key'. | [optional]
**crv** | Option<**String**> | Elliptic curve name. For valid values, see JsonWebKeyCurveName. | [optional]
**x** | Option<**String**> | X component of an EC public key. | [optional]
**y** | Option<**String**> | Y component of an EC public key. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


