# SasDefinitionAttributes

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**enabled** | Option<**bool**> | the enabled state of the object. | [optional]
**created** | Option<**i32**> | Creation time in UTC. | [optional][readonly]
**updated** | Option<**i32**> | Last updated time in UTC. | [optional][readonly]
**recoverable_days** | Option<**i32**> | softDelete data retention days. Value should be >=7 and <=90 when softDelete enabled, otherwise 0. | [optional][readonly]
**recovery_level** | Option<**String**> | Reflects the deletion recovery level currently in effect for SAS definitions in the current vault. If it contains 'Purgeable' the SAS definition can be permanently deleted by a privileged user; otherwise, only the system can purge the SAS definition, at the end of the retention interval. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


