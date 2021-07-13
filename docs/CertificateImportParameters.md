# CertificateImportParameters

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**value** | **String** | Base64 encoded representation of the certificate object to import. This certificate needs to contain the private key. | 
**pwd** | Option<**String**> | If the private key in base64EncodedCertificate is encrypted, the password used for encryption. | [optional]
**policy** | Option<[**crate::models::CertificatePolicy**](CertificatePolicy.md)> |  | [optional]
**attributes** | Option<[**crate::models::CertificateAttributes**](CertificateAttributes.md)> |  | [optional]
**tags** | Option<**::std::collections::HashMap<String, String>**> | Application specific metadata in the form of key-value pairs. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


