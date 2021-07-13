# CertificateOperation

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The certificate id. | [optional][readonly]
**issuer** | Option<[**crate::models::IssuerParameters**](IssuerParameters.md)> |  | [optional]
**csr** | Option<**String**> | The certificate signing request (CSR) that is being used in the certificate operation. | [optional]
**cancellation_requested** | Option<**bool**> | Indicates if cancellation was requested on the certificate operation. | [optional]
**status** | Option<**String**> | Status of the certificate operation. | [optional]
**status_details** | Option<**String**> | The status details of the certificate operation. | [optional]
**error** | Option<[**crate::models::Error**](Error.md)> |  | [optional]
**target** | Option<**String**> | Location which contains the result of the certificate operation. | [optional]
**request_id** | Option<**String**> | Identifier for the certificate operation. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


