# PlanObject

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**lago_id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**name** | **String** |  | 
**created_at** | **String** |  | 
**code** | **String** |  | 
**interval** | **String** | Plan interval | 
**description** | Option<**String**> |  | [optional]
**amount_cents** | **i32** |  | 
**amount_currency** | **String** |  | 
**trial_period** | Option<**f32**> |  | [optional]
**pay_in_advance** | Option<**bool**> |  | [optional]
**bill_charges_monthly** | Option<**bool**> |  | [optional]
**active_subscriptions_count** | **i32** |  | 
**draft_invoices_count** | **i32** |  | 
**charges** | Option<[**Vec<crate::models::ChargeObject>**](ChargeObject.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


