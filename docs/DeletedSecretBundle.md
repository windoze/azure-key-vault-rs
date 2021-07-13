# DeletedSecretBundle

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**recovery_id** | Option<**String**> | The url of the recovery object, used to identify and recover the deleted secret. | [optional]
**scheduled_purge_date** | Option<**i32**> | The time when the secret is scheduled to be purged, in UTC | [optional][readonly]
**deleted_date** | Option<**i32**> | The time when the secret was deleted, in UTC | [optional][readonly]
**value** | Option<**String**> | The secret value. | [optional]
**id** | Option<**String**> | The secret id. | [optional]
**content_type** | Option<**String**> | The content type of the secret. | [optional]
**attributes** | Option<[**crate::models::SecretAttributes**](SecretAttributes.md)> |  | [optional]
**tags** | Option<**::std::collections::HashMap<String, String>**> | Application specific metadata in the form of key-value pairs. | [optional]
**kid** | Option<**String**> | If this is a secret backing a KV certificate, then this field specifies the corresponding key backing the KV certificate. | [optional][readonly]
**managed** | Option<**bool**> | True if the secret's lifetime is managed by key vault. If this is a secret backing a certificate, then managed will be true. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


