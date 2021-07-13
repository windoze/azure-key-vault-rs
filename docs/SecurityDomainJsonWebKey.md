# SecurityDomainJsonWebKey

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**kid** | **String** | Key identifier. | 
**kty** | **String** | JsonWebKey Key Type (kty), as defined in https://tools.ietf.org/html/draft-ietf-jose-json-web-algorithms-40. For Security Domain this value must be RSA. | 
**key_ops** | **Vec<String>** |  | 
**n** | **String** | RSA modulus. | 
**e** | **String** | RSA public exponent. | 
**x5c** | **Vec<String>** | X509 certificate chain parameter | 
**_use** | Option<**String**> | Public Key Use Parameter. This is optional and if present must be enc. | [optional]
**x5t** | Option<**String**> | X509 certificate SHA1 thumbprint. This is optional. | [optional]
**x5t_s256** | **String** | X509 certificate SHA256 thumbprint. | 
**alg** | **String** | Algorithm intended for use with the key. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


