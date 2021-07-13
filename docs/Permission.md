# Permission

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**actions** | Option<**Vec<String>**> | Action permissions that are granted. | [optional]
**not_actions** | Option<**Vec<String>**> | Action permissions that are excluded but not denied. They may be granted by other role definitions assigned to a principal. | [optional]
**data_actions** | Option<[**Vec<crate::models::DataAction>**](DataAction.md)> | Data action permissions that are granted. | [optional]
**not_data_actions** | Option<[**Vec<crate::models::DataAction>**](DataAction.md)> | Data action permissions that are excluded but not denied. They may be granted by other role definitions assigned to a principal. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


