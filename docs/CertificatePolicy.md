# CertificatePolicy

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The certificate id. | [optional][readonly]
**key_props** | Option<[**crate::models::KeyProperties**](KeyProperties.md)> |  | [optional]
**secret_props** | Option<[**crate::models::SecretProperties**](SecretProperties.md)> |  | [optional]
**x509_props** | Option<[**crate::models::X509CertificateProperties**](X509CertificateProperties.md)> |  | [optional]
**lifetime_actions** | Option<[**Vec<crate::models::LifetimeAction>**](LifetimeAction.md)> | Actions that will be performed by Key Vault over the lifetime of a certificate. | [optional]
**issuer** | Option<[**crate::models::IssuerParameters**](IssuerParameters.md)> |  | [optional]
**attributes** | Option<[**crate::models::CertificateAttributes**](CertificateAttributes.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


