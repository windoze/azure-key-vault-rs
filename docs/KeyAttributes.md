# KeyAttributes

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**recoverable_days** | Option<**i32**> | softDelete data retention days. Value should be >=7 and <=90 when softDelete enabled, otherwise 0. | [optional][readonly]
**recovery_level** | Option<**String**> | Reflects the deletion recovery level currently in effect for keys in the current vault. If it contains 'Purgeable' the key can be permanently deleted by a privileged user; otherwise, only the system can purge the key, at the end of the retention interval. | [optional][readonly]
**enabled** | Option<**bool**> | Determines whether the object is enabled. | [optional]
**nbf** | Option<**i32**> | Not before date in UTC. | [optional]
**exp** | Option<**i32**> | Expiry date in UTC. | [optional]
**created** | Option<**i32**> | Creation time in UTC. | [optional][readonly]
**updated** | Option<**i32**> | Last updated time in UTC. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


