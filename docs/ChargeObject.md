# ChargeObject

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**lago_id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**lago_billable_metric_id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**billable_metric_code** | **String** |  | 
**created_at** | **String** |  | 
**charge_model** | **String** | Charge model type | 
**instant** | Option<**bool**> |  | [optional]
**min_amount_cents** | Option<**i32**> |  | [optional]
**properties** | Option<[**serde_json::Value**](.md)> |  | [optional]
**group_properties** | Option<[**Vec<crate::models::GroupPropertiesObject>**](GroupPropertiesObject.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


