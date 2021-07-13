# Rust API client for openapi

The key vault client performs cryptographic key operations and vault operations against the Key Vault service.

## Overview

This API client was generated by the [OpenAPI Generator](https://openapi-generator.tech) project.  By using the [openapi-spec](https://openapis.org) from a remote server, you can easily generate an API client.

- API version: 7.2
- Package version: 1.0.0
- Build package: org.openapitools.codegen.languages.RustClientCodegen

## Installation

Put the package under your project folder and add the following to `Cargo.toml` under `[dependencies]`:

```
    openapi = { path = "./generated" }
```

## Documentation for API Endpoints

All URIs are relative to *http://localhost*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*DeletedStorageApi* | [**get_deleted_sas_definition**](docs/DeletedStorageApi.md#get_deleted_sas_definition) | **get** /deletedstorage/{storage_account_name}/sas/{sas_definition_name} | Gets the specified deleted sas definition.
*DeletedStorageApi* | [**get_deleted_sas_definitions**](docs/DeletedStorageApi.md#get_deleted_sas_definitions) | **get** /deletedstorage/{storage_account_name}/sas | Lists deleted SAS definitions for the specified vault and storage account.
*DeletedStorageApi* | [**get_deleted_storage_account**](docs/DeletedStorageApi.md#get_deleted_storage_account) | **get** /deletedstorage/{storage_account_name} | Gets the specified deleted storage account.
*DeletedStorageApi* | [**get_deleted_storage_accounts**](docs/DeletedStorageApi.md#get_deleted_storage_accounts) | **get** /deletedstorage | Lists deleted storage accounts for the specified vault.
*DeletedStorageApi* | [**purge_deleted_storage_account**](docs/DeletedStorageApi.md#purge_deleted_storage_account) | **delete** /deletedstorage/{storage_account_name} | Permanently deletes the specified storage account.
*DeletedStorageApi* | [**recover_deleted_sas_definition**](docs/DeletedStorageApi.md#recover_deleted_sas_definition) | **post** /deletedstorage/{storage_account_name}/sas/{sas_definition_name}/recover | Recovers the deleted SAS definition.
*DeletedStorageApi* | [**recover_deleted_storage_account**](docs/DeletedStorageApi.md#recover_deleted_storage_account) | **post** /deletedstorage/{storage_account_name}/recover | Recovers the deleted storage account.
*StorageApi* | [**backup_storage_account**](docs/StorageApi.md#backup_storage_account) | **post** /storage/{storage_account_name}/backup | Backs up the specified storage account.
*StorageApi* | [**delete_sas_definition**](docs/StorageApi.md#delete_sas_definition) | **delete** /storage/{storage_account_name}/sas/{sas_definition_name} | 
*StorageApi* | [**delete_storage_account**](docs/StorageApi.md#delete_storage_account) | **delete** /storage/{storage_account_name} | 
*StorageApi* | [**get_sas_definition**](docs/StorageApi.md#get_sas_definition) | **get** /storage/{storage_account_name}/sas/{sas_definition_name} | 
*StorageApi* | [**get_sas_definitions**](docs/StorageApi.md#get_sas_definitions) | **get** /storage/{storage_account_name}/sas | 
*StorageApi* | [**get_storage_account**](docs/StorageApi.md#get_storage_account) | **get** /storage/{storage_account_name} | 
*StorageApi* | [**get_storage_accounts**](docs/StorageApi.md#get_storage_accounts) | **get** /storage | 
*StorageApi* | [**regenerate_storage_account_key**](docs/StorageApi.md#regenerate_storage_account_key) | **post** /storage/{storage_account_name}/regeneratekey | 
*StorageApi* | [**restore_storage_account**](docs/StorageApi.md#restore_storage_account) | **post** /storage/restore | Restores a backed up storage account to a vault.
*StorageApi* | [**set_sas_definition**](docs/StorageApi.md#set_sas_definition) | **put** /storage/{storage_account_name}/sas/{sas_definition_name} | 
*StorageApi* | [**set_storage_account**](docs/StorageApi.md#set_storage_account) | **put** /storage/{storage_account_name} | 
*StorageApi* | [**update_sas_definition**](docs/StorageApi.md#update_sas_definition) | **patch** /storage/{storage_account_name}/sas/{sas_definition_name} | 
*StorageApi* | [**update_storage_account**](docs/StorageApi.md#update_storage_account) | **patch** /storage/{storage_account_name} | 


## Documentation For Models

 - [BackupStorageResult](docs/BackupStorageResult.md)
 - [DeletedSasDefinitionBundle](docs/DeletedSasDefinitionBundle.md)
 - [DeletedSasDefinitionItem](docs/DeletedSasDefinitionItem.md)
 - [DeletedSasDefinitionListResult](docs/DeletedSasDefinitionListResult.md)
 - [DeletedStorageAccountItem](docs/DeletedStorageAccountItem.md)
 - [DeletedStorageBundle](docs/DeletedStorageBundle.md)
 - [DeletedStorageListResult](docs/DeletedStorageListResult.md)
 - [Error](docs/Error.md)
 - [KeyVaultError](docs/KeyVaultError.md)
 - [SasDefinitionAttributes](docs/SasDefinitionAttributes.md)
 - [SasDefinitionBundle](docs/SasDefinitionBundle.md)
 - [SasDefinitionCreateParameters](docs/SasDefinitionCreateParameters.md)
 - [SasDefinitionItem](docs/SasDefinitionItem.md)
 - [SasDefinitionListResult](docs/SasDefinitionListResult.md)
 - [SasDefinitionUpdateParameters](docs/SasDefinitionUpdateParameters.md)
 - [StorageAccountAttributes](docs/StorageAccountAttributes.md)
 - [StorageAccountCreateParameters](docs/StorageAccountCreateParameters.md)
 - [StorageAccountItem](docs/StorageAccountItem.md)
 - [StorageAccountRegenerteKeyParameters](docs/StorageAccountRegenerteKeyParameters.md)
 - [StorageAccountUpdateParameters](docs/StorageAccountUpdateParameters.md)
 - [StorageBundle](docs/StorageBundle.md)
 - [StorageListResult](docs/StorageListResult.md)
 - [StorageRestoreParameters](docs/StorageRestoreParameters.md)


To get access to the crate's generated documentation, use:

```
cargo doc --open
```

## Author


