# AppliedCouponObject

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**lago_id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**lago_coupon_id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**coupon_code** | **String** |  | 
**lago_customer_id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**external_customer_id** | **String** |  | 
**status** | **String** | Status | 
**amount_cents** | **i32** |  | 
**amount_cents_remaining** | Option<**i32**> |  | [optional]
**amount_currency** | **String** |  | 
**percentage_rate** | Option<**f32**> |  | [optional]
**frequency** | **String** | Frequency type | 
**frequency_duration** | Option<**i32**> |  | [optional]
**frequency_duration_remaining** | Option<**i32**> |  | [optional]
**expiration_at** | Option<**String**> |  | [optional]
**created_at** | **String** |  | 
**terminated_at** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


