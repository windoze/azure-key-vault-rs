# \DeletedCertificatesApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_deleted_certificate**](DeletedCertificatesApi.md#get_deleted_certificate) | **get** /deletedcertificates/{certificate_name} | Retrieves information about the specified deleted certificate.
[**get_deleted_certificates**](DeletedCertificatesApi.md#get_deleted_certificates) | **get** /deletedcertificates | Lists the deleted certificates in the specified vault currently available for recovery.
[**purge_deleted_certificate**](DeletedCertificatesApi.md#purge_deleted_certificate) | **delete** /deletedcertificates/{certificate_name} | Permanently deletes the specified deleted certificate.
[**recover_deleted_certificate**](DeletedCertificatesApi.md#recover_deleted_certificate) | **post** /deletedcertificates/{certificate_name}/recover | Recovers the deleted certificate back to its current version under /certificates.



## get_deleted_certificate

> crate::models::DeletedCertificateBundle get_deleted_certificate(certificate_name, api_version)
Retrieves information about the specified deleted certificate.

The GetDeletedCertificate operation retrieves the deleted certificate information plus its attributes, such as retention interval, scheduled permanent deletion and the current deletion recovery level. This operation requires the certificates/get permission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**certificate_name** | **String** | The name of the certificate | [required] |
**api_version** | **String** | Client API version. | [required] |

### Return type

[**crate::models::DeletedCertificateBundle**](DeletedCertificateBundle.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_deleted_certificates

> crate::models::DeletedCertificateListResult get_deleted_certificates(api_version, maxresults, include_pending)
Lists the deleted certificates in the specified vault currently available for recovery.

The GetDeletedCertificates operation retrieves the certificates in the current vault which are in a deleted state and ready for recovery or purging. This operation includes deletion-specific information. This operation requires the certificates/get/list permission. This operation can only be enabled on soft-delete enabled vaults.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | Client API version. | [required] |
**maxresults** | Option<**i32**> | Maximum number of results to return in a page. If not specified the service will return up to 25 results. |  |
**include_pending** | Option<**bool**> | Specifies whether to include certificates which are not completely provisioned. |  |

### Return type

[**crate::models::DeletedCertificateListResult**](DeletedCertificateListResult.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## purge_deleted_certificate

> purge_deleted_certificate(certificate_name, api_version)
Permanently deletes the specified deleted certificate.

The PurgeDeletedCertificate operation performs an irreversible deletion of the specified certificate, without possibility for recovery. The operation is not available if the recovery level does not specify 'Purgeable'. This operation requires the certificate/purge permission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**certificate_name** | **String** | The name of the certificate | [required] |
**api_version** | **String** | Client API version. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## recover_deleted_certificate

> crate::models::CertificateBundle recover_deleted_certificate(certificate_name, api_version)
Recovers the deleted certificate back to its current version under /certificates.

The RecoverDeletedCertificate operation performs the reversal of the Delete operation. The operation is applicable in vaults enabled for soft-delete, and must be issued during the retention interval (available in the deleted certificate's attributes). This operation requires the certificates/recover permission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**certificate_name** | **String** | The name of the deleted certificate | [required] |
**api_version** | **String** | Client API version. | [required] |

### Return type

[**crate::models::CertificateBundle**](CertificateBundle.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

