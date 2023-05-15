# CouponObject

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**lago_id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**name** | **String** |  | 
**code** | **String** |  | 
**coupon_type** | **String** | Coupon type | 
**amount_cents** | Option<**i32**> |  | [optional]
**amount_currency** | Option<**String**> |  | [optional]
**reusable** | Option<**bool**> |  | [optional]
**limited_plans** | Option<**bool**> |  | [optional]
**plan_codes** | Option<**Vec<String>**> |  | [optional]
**limited_billable_metrics** | Option<**bool**> |  | [optional]
**billable_metric_codes** | Option<**Vec<String>**> |  | [optional]
**created_at** | **String** |  | 
**percentage_rate** | Option<**f32**> |  | [optional]
**frequency** | **String** | Frequency type | 
**frequency_duration** | Option<**i32**> |  | [optional]
**expiration_at** | Option<**String**> |  | [optional]
**expiration** | **String** | Expiration type | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


