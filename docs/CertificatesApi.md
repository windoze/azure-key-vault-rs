# \CertificatesApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**backup_certificate**](CertificatesApi.md#backup_certificate) | **post** /certificates/{certificate_name}/backup | Backs up the specified certificate.
[**create_certificate**](CertificatesApi.md#create_certificate) | **post** /certificates/{certificate_name}/create | Creates a new certificate.
[**delete_certificate**](CertificatesApi.md#delete_certificate) | **delete** /certificates/{certificate_name} | Deletes a certificate from a specified key vault.
[**delete_certificate_contacts**](CertificatesApi.md#delete_certificate_contacts) | **delete** /certificates/contacts | Deletes the certificate contacts for a specified key vault.
[**delete_certificate_issuer**](CertificatesApi.md#delete_certificate_issuer) | **delete** /certificates/issuers/{issuer_name} | Deletes the specified certificate issuer.
[**delete_certificate_operation**](CertificatesApi.md#delete_certificate_operation) | **delete** /certificates/{certificate_name}/pending | Deletes the creation operation for a specific certificate.
[**get_certificate**](CertificatesApi.md#get_certificate) | **get** /certificates/{certificate_name}/{certificate_version} | Gets information about a certificate.
[**get_certificate_contacts**](CertificatesApi.md#get_certificate_contacts) | **get** /certificates/contacts | Lists the certificate contacts for a specified key vault.
[**get_certificate_issuer**](CertificatesApi.md#get_certificate_issuer) | **get** /certificates/issuers/{issuer_name} | Lists the specified certificate issuer.
[**get_certificate_issuers**](CertificatesApi.md#get_certificate_issuers) | **get** /certificates/issuers | List certificate issuers for a specified key vault.
[**get_certificate_operation**](CertificatesApi.md#get_certificate_operation) | **get** /certificates/{certificate_name}/pending | Gets the creation operation of a certificate.
[**get_certificate_policy**](CertificatesApi.md#get_certificate_policy) | **get** /certificates/{certificate_name}/policy | Lists the policy for a certificate.
[**get_certificate_versions**](CertificatesApi.md#get_certificate_versions) | **get** /certificates/{certificate_name}/versions | List the versions of a certificate.
[**get_certificates**](CertificatesApi.md#get_certificates) | **get** /certificates | List certificates in a specified key vault
[**import_certificate**](CertificatesApi.md#import_certificate) | **post** /certificates/{certificate_name}/import | Imports a certificate into a specified key vault.
[**merge_certificate**](CertificatesApi.md#merge_certificate) | **post** /certificates/{certificate_name}/pending/merge | Merges a certificate or a certificate chain with a key pair existing on the server.
[**restore_certificate**](CertificatesApi.md#restore_certificate) | **post** /certificates/restore | Restores a backed up certificate to a vault.
[**set_certificate_contacts**](CertificatesApi.md#set_certificate_contacts) | **put** /certificates/contacts | Sets the certificate contacts for the specified key vault.
[**set_certificate_issuer**](CertificatesApi.md#set_certificate_issuer) | **put** /certificates/issuers/{issuer_name} | Sets the specified certificate issuer.
[**update_certificate**](CertificatesApi.md#update_certificate) | **patch** /certificates/{certificate_name}/{certificate_version} | Updates the specified attributes associated with the given certificate.
[**update_certificate_issuer**](CertificatesApi.md#update_certificate_issuer) | **patch** /certificates/issuers/{issuer_name} | Updates the specified certificate issuer.
[**update_certificate_operation**](CertificatesApi.md#update_certificate_operation) | **patch** /certificates/{certificate_name}/pending | Updates a certificate operation.
[**update_certificate_policy**](CertificatesApi.md#update_certificate_policy) | **patch** /certificates/{certificate_name}/policy | Updates the policy for a certificate.



## backup_certificate

> crate::models::BackupCertificateResult backup_certificate(certificate_name, api_version)
Backs up the specified certificate.

Requests that a backup of the specified certificate be downloaded to the client. All versions of the certificate will be downloaded. This operation requires the certificates/backup permission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**certificate_name** | **String** | The name of the certificate. | [required] |
**api_version** | **String** | Client API version. | [required] |

### Return type

[**crate::models::BackupCertificateResult**](BackupCertificateResult.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_certificate

> crate::models::CertificateOperation create_certificate(certificate_name, api_version, parameters)
Creates a new certificate.

If this is the first version, the certificate resource is created. This operation requires the certificates/create permission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**certificate_name** | **String** | The name of the certificate. | [required] |
**api_version** | **String** | Client API version. | [required] |
**parameters** | [**CertificateCreateParameters**](CertificateCreateParameters.md) | The parameters to create a certificate. | [required] |

### Return type

[**crate::models::CertificateOperation**](CertificateOperation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_certificate

> crate::models::DeletedCertificateBundle delete_certificate(certificate_name, api_version)
Deletes a certificate from a specified key vault.

Deletes all versions of a certificate object along with its associated policy. Delete certificate cannot be used to remove individual versions of a certificate object. This operation requires the certificates/delete permission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**certificate_name** | **String** | The name of the certificate. | [required] |
**api_version** | **String** | Client API version. | [required] |

### Return type

[**crate::models::DeletedCertificateBundle**](DeletedCertificateBundle.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_certificate_contacts

> crate::models::Contacts delete_certificate_contacts(api_version)
Deletes the certificate contacts for a specified key vault.

Deletes the certificate contacts for a specified key vault certificate. This operation requires the certificates/managecontacts permission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | Client API version. | [required] |

### Return type

[**crate::models::Contacts**](Contacts.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_certificate_issuer

> crate::models::IssuerBundle delete_certificate_issuer(issuer_name, api_version)
Deletes the specified certificate issuer.

The DeleteCertificateIssuer operation permanently removes the specified certificate issuer from the vault. This operation requires the certificates/manageissuers/deleteissuers permission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**issuer_name** | **String** | The name of the issuer. | [required] |
**api_version** | **String** | Client API version. | [required] |

### Return type

[**crate::models::IssuerBundle**](IssuerBundle.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_certificate_operation

> crate::models::CertificateOperation delete_certificate_operation(certificate_name, api_version)
Deletes the creation operation for a specific certificate.

Deletes the creation operation for a specified certificate that is in the process of being created. The certificate is no longer created. This operation requires the certificates/update permission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**certificate_name** | **String** | The name of the certificate. | [required] |
**api_version** | **String** | Client API version. | [required] |

### Return type

[**crate::models::CertificateOperation**](CertificateOperation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_certificate

> crate::models::CertificateBundle get_certificate(certificate_name, certificate_version, api_version)
Gets information about a certificate.

Gets information about a specific certificate. This operation requires the certificates/get permission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**certificate_name** | **String** | The name of the certificate in the given vault. | [required] |
**certificate_version** | **String** | The version of the certificate. This URI fragment is optional. If not specified, the latest version of the certificate is returned. | [required] |
**api_version** | **String** | Client API version. | [required] |

### Return type

[**crate::models::CertificateBundle**](CertificateBundle.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_certificate_contacts

> crate::models::Contacts get_certificate_contacts(api_version)
Lists the certificate contacts for a specified key vault.

The GetCertificateContacts operation returns the set of certificate contact resources in the specified key vault. This operation requires the certificates/managecontacts permission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | Client API version. | [required] |

### Return type

[**crate::models::Contacts**](Contacts.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_certificate_issuer

> crate::models::IssuerBundle get_certificate_issuer(issuer_name, api_version)
Lists the specified certificate issuer.

The GetCertificateIssuer operation returns the specified certificate issuer resources in the specified key vault. This operation requires the certificates/manageissuers/getissuers permission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**issuer_name** | **String** | The name of the issuer. | [required] |
**api_version** | **String** | Client API version. | [required] |

### Return type

[**crate::models::IssuerBundle**](IssuerBundle.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_certificate_issuers

> crate::models::CertificateIssuerListResult get_certificate_issuers(api_version, maxresults)
List certificate issuers for a specified key vault.

The GetCertificateIssuers operation returns the set of certificate issuer resources in the specified key vault. This operation requires the certificates/manageissuers/getissuers permission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | Client API version. | [required] |
**maxresults** | Option<**i32**> | Maximum number of results to return in a page. If not specified the service will return up to 25 results. |  |

### Return type

[**crate::models::CertificateIssuerListResult**](CertificateIssuerListResult.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_certificate_operation

> crate::models::CertificateOperation get_certificate_operation(certificate_name, api_version)
Gets the creation operation of a certificate.

Gets the creation operation associated with a specified certificate. This operation requires the certificates/get permission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**certificate_name** | **String** | The name of the certificate. | [required] |
**api_version** | **String** | Client API version. | [required] |

### Return type

[**crate::models::CertificateOperation**](CertificateOperation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_certificate_policy

> crate::models::CertificatePolicy get_certificate_policy(certificate_name, api_version)
Lists the policy for a certificate.

The GetCertificatePolicy operation returns the specified certificate policy resources in the specified key vault. This operation requires the certificates/get permission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**certificate_name** | **String** | The name of the certificate in a given key vault. | [required] |
**api_version** | **String** | Client API version. | [required] |

### Return type

[**crate::models::CertificatePolicy**](CertificatePolicy.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_certificate_versions

> crate::models::CertificateListResult get_certificate_versions(certificate_name, api_version, maxresults)
List the versions of a certificate.

The GetCertificateVersions operation returns the versions of a certificate in the specified key vault. This operation requires the certificates/list permission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**certificate_name** | **String** | The name of the certificate. | [required] |
**api_version** | **String** | Client API version. | [required] |
**maxresults** | Option<**i32**> | Maximum number of results to return in a page. If not specified the service will return up to 25 results. |  |

### Return type

[**crate::models::CertificateListResult**](CertificateListResult.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_certificates

> crate::models::CertificateListResult get_certificates(api_version, maxresults, include_pending)
List certificates in a specified key vault

The GetCertificates operation returns the set of certificates resources in the specified key vault. This operation requires the certificates/list permission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | Client API version. | [required] |
**maxresults** | Option<**i32**> | Maximum number of results to return in a page. If not specified the service will return up to 25 results. |  |
**include_pending** | Option<**bool**> | Specifies whether to include certificates which are not completely provisioned. |  |

### Return type

[**crate::models::CertificateListResult**](CertificateListResult.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## import_certificate

> crate::models::CertificateBundle import_certificate(certificate_name, api_version, parameters)
Imports a certificate into a specified key vault.

Imports an existing valid certificate, containing a private key, into Azure Key Vault. The certificate to be imported can be in either PFX or PEM format. If the certificate is in PEM format the PEM file must contain the key as well as x509 certificates. This operation requires the certificates/import permission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**certificate_name** | **String** | The name of the certificate. | [required] |
**api_version** | **String** | Client API version. | [required] |
**parameters** | [**CertificateImportParameters**](CertificateImportParameters.md) | The parameters to import the certificate. | [required] |

### Return type

[**crate::models::CertificateBundle**](CertificateBundle.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## merge_certificate

> crate::models::CertificateBundle merge_certificate(certificate_name, api_version, parameters)
Merges a certificate or a certificate chain with a key pair existing on the server.

The MergeCertificate operation performs the merging of a certificate or certificate chain with a key pair currently available in the service. This operation requires the certificates/create permission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**certificate_name** | **String** | The name of the certificate. | [required] |
**api_version** | **String** | Client API version. | [required] |
**parameters** | [**CertificateMergeParameters**](CertificateMergeParameters.md) | The parameters to merge certificate. | [required] |

### Return type

[**crate::models::CertificateBundle**](CertificateBundle.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## restore_certificate

> crate::models::CertificateBundle restore_certificate(api_version, parameters)
Restores a backed up certificate to a vault.

Restores a backed up certificate, and all its versions, to a vault. This operation requires the certificates/restore permission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | Client API version. | [required] |
**parameters** | [**CertificateRestoreParameters**](CertificateRestoreParameters.md) | The parameters to restore the certificate. | [required] |

### Return type

[**crate::models::CertificateBundle**](CertificateBundle.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_certificate_contacts

> crate::models::Contacts set_certificate_contacts(api_version, contacts)
Sets the certificate contacts for the specified key vault.

Sets the certificate contacts for the specified key vault. This operation requires the certificates/managecontacts permission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_version** | **String** | Client API version. | [required] |
**contacts** | [**Contacts**](Contacts.md) | The contacts for the key vault certificate. | [required] |

### Return type

[**crate::models::Contacts**](Contacts.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_certificate_issuer

> crate::models::IssuerBundle set_certificate_issuer(issuer_name, api_version, parameter)
Sets the specified certificate issuer.

The SetCertificateIssuer operation adds or updates the specified certificate issuer. This operation requires the certificates/setissuers permission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**issuer_name** | **String** | The name of the issuer. | [required] |
**api_version** | **String** | Client API version. | [required] |
**parameter** | [**CertificateIssuerSetParameters**](CertificateIssuerSetParameters.md) | Certificate issuer set parameter. | [required] |

### Return type

[**crate::models::IssuerBundle**](IssuerBundle.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_certificate

> crate::models::CertificateBundle update_certificate(certificate_name, certificate_version, api_version, parameters)
Updates the specified attributes associated with the given certificate.

The UpdateCertificate operation applies the specified update on the given certificate; the only elements updated are the certificate's attributes. This operation requires the certificates/update permission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**certificate_name** | **String** | The name of the certificate in the given key vault. | [required] |
**certificate_version** | **String** | The version of the certificate. | [required] |
**api_version** | **String** | Client API version. | [required] |
**parameters** | [**CertificateUpdateParameters**](CertificateUpdateParameters.md) | The parameters for certificate update. | [required] |

### Return type

[**crate::models::CertificateBundle**](CertificateBundle.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_certificate_issuer

> crate::models::IssuerBundle update_certificate_issuer(issuer_name, api_version, parameter)
Updates the specified certificate issuer.

The UpdateCertificateIssuer operation performs an update on the specified certificate issuer entity. This operation requires the certificates/setissuers permission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**issuer_name** | **String** | The name of the issuer. | [required] |
**api_version** | **String** | Client API version. | [required] |
**parameter** | [**CertificateIssuerUpdateParameters**](CertificateIssuerUpdateParameters.md) | Certificate issuer update parameter. | [required] |

### Return type

[**crate::models::IssuerBundle**](IssuerBundle.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_certificate_operation

> crate::models::CertificateOperation update_certificate_operation(certificate_name, api_version, certificate_operation)
Updates a certificate operation.

Updates a certificate creation operation that is already in progress. This operation requires the certificates/update permission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**certificate_name** | **String** | The name of the certificate. | [required] |
**api_version** | **String** | Client API version. | [required] |
**certificate_operation** | [**CertificateOperationUpdateParameter**](CertificateOperationUpdateParameter.md) | The certificate operation response. | [required] |

### Return type

[**crate::models::CertificateOperation**](CertificateOperation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_certificate_policy

> crate::models::CertificatePolicy update_certificate_policy(certificate_name, api_version, certificate_policy)
Updates the policy for a certificate.

Set specified members in the certificate policy. Leave others as null. This operation requires the certificates/update permission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**certificate_name** | **String** | The name of the certificate in the given vault. | [required] |
**api_version** | **String** | Client API version. | [required] |
**certificate_policy** | [**CertificatePolicy**](CertificatePolicy.md) | The policy for the certificate. | [required] |

### Return type

[**crate::models::CertificatePolicy**](CertificatePolicy.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

