# X509CertificateProperties

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**subject** | Option<**String**> | The subject name. Should be a valid X509 distinguished Name. | [optional]
**ekus** | Option<**Vec<String>**> | The enhanced key usage. | [optional]
**sans** | Option<[**crate::models::SubjectAlternativeNames**](SubjectAlternativeNames.md)> |  | [optional]
**key_usage** | Option<**Vec<String>**> | List of key usages. | [optional]
**validity_months** | Option<**i32**> | The duration that the certificate is valid in months. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


