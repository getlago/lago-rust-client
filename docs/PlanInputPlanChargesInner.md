# PlanInputPlanChargesInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**billable_metric_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**charge_model** | Option<**String**> | Charge model type | [optional]
**instant** | Option<**bool**> |  | [optional]
**min_amount_cents** | Option<**i32**> |  | [optional]
**properties** | Option<[**serde_json::Value**](.md)> |  | [optional]
**group_properties** | Option<[**Vec<crate::models::PlanInputPlanChargesInnerGroupPropertiesInner>**](PlanInput_plan_charges_inner_group_properties_inner.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


