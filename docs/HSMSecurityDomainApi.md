# \HSMSecurityDomainApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**h_sm_security_domain_download**](HSMSecurityDomainApi.md#h_sm_security_domain_download) | **post** /securitydomain/download | 
[**h_sm_security_domain_download_pending**](HSMSecurityDomainApi.md#h_sm_security_domain_download_pending) | **get** /securitydomain/download/pending | 
[**h_sm_security_domain_transfer_key**](HSMSecurityDomainApi.md#h_sm_security_domain_transfer_key) | **get** /securitydomain/upload | 
[**h_sm_security_domain_upload**](HSMSecurityDomainApi.md#h_sm_security_domain_upload) | **post** /securitydomain/upload | 
[**h_sm_security_domain_upload_pending**](HSMSecurityDomainApi.md#h_sm_security_domain_upload_pending) | **get** /securitydomain/upload/pending | 



## h_sm_security_domain_download

> crate::models::SecurityDomainObject h_sm_security_domain_download(api_version, certificate_info_object)


Retrieves the Security Domain from the managed HSM. Calling this endpoint can be used to activate a provisioned managed HSM resource.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | Client API version. | [required] |
**certificate_info_object** | [**CertificateInfoObject**](CertificateInfoObject.md) | The Security Domain download operation requires customer to provide N certificates (minimum 3 and maximum 10) containing a public key in JWK format. | [required] |

### Return type

[**crate::models::SecurityDomainObject**](SecurityDomainObject.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## h_sm_security_domain_download_pending

> crate::models::SecurityDomainOperationStatus h_sm_security_domain_download_pending()


Retrieves the Security Domain download operation status

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::SecurityDomainOperationStatus**](SecurityDomainOperationStatus.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## h_sm_security_domain_transfer_key

> crate::models::TransferKey h_sm_security_domain_transfer_key(api_version)


Retrieve Security Domain transfer key

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | Client API version. | [required] |

### Return type

[**crate::models::TransferKey**](TransferKey.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## h_sm_security_domain_upload

> crate::models::SecurityDomainOperationStatus h_sm_security_domain_upload(security_domain)


Restore the provided Security Domain.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**security_domain** | [**SecurityDomainObject**](SecurityDomainObject.md) | The Security Domain to be restored. | [required] |

### Return type

[**crate::models::SecurityDomainOperationStatus**](SecurityDomainOperationStatus.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## h_sm_security_domain_upload_pending

> crate::models::SecurityDomainOperationStatus h_sm_security_domain_upload_pending()


Get Security Domain upload operation status

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::SecurityDomainOperationStatus**](SecurityDomainOperationStatus.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

