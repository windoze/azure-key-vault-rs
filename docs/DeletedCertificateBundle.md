# DeletedCertificateBundle

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**recovery_id** | Option<**String**> | The url of the recovery object, used to identify and recover the deleted certificate. | [optional]
**scheduled_purge_date** | Option<**i32**> | The time when the certificate is scheduled to be purged, in UTC | [optional][readonly]
**deleted_date** | Option<**i32**> | The time when the certificate was deleted, in UTC | [optional][readonly]
**id** | Option<**String**> | The certificate id. | [optional][readonly]
**kid** | Option<**String**> | The key id. | [optional][readonly]
**sid** | Option<**String**> | The secret id. | [optional][readonly]
**x5t** | Option<**String**> | Thumbprint of the certificate. | [optional][readonly]
**policy** | Option<[**crate::models::CertificatePolicy**](CertificatePolicy.md)> |  | [optional]
**cer** | Option<**String**> | CER contents of x509 certificate. | [optional]
**content_type** | Option<**String**> | The content type of the secret. | [optional]
**attributes** | Option<[**crate::models::CertificateAttributes**](CertificateAttributes.md)> |  | [optional]
**tags** | Option<**::std::collections::HashMap<String, String>**> | Application specific metadata in the form of key-value pairs | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


